use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::ops::Add;

use tg4::Member;
use tg_bindings::{Ed25519Pubkey, Pubkey};
use tg_utils::{Duration, Expiration, JailingDuration};

use crate::error::ContractError;
use crate::state::{DistributionContract, OperatorInfo, ValidatorInfo, ValidatorSlashing};
use cosmwasm_std::{Addr, Api, BlockInfo, Coin, Decimal, Timestamp};

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct InstantiateMsg {
    /// Address allowed to jail, meant to be a OC voting contract. If `None`, then jailing is
    /// impossible in this contract.
    pub admin: Option<String>,
    /// Address of a cw4 contract with the raw membership used to feed the validator set
    pub membership: String,
    /// Minimum points needed by an address in `membership` to be considered for the validator set.
    /// 0-point members are always filtered out.
    /// (use points for cw4, power for Tendermint)
    pub min_points: u64,
    /// The maximum number of validators that can be included in the Tendermint validator set.
    /// If there are more validators than slots, we select the top N by membership points
    /// descending. (In case of ties at the last slot, select by "first" Tendermint pubkey,
    /// lexicographically sorted).
    pub max_validators: u32,
    /// Number of seconds in one epoch. We update the Tendermint validator set only once per epoch.
    /// Epoch # is env.block.time/epoch_length (round down). The first block with a new epoch number
    /// will trigger a new validator calculation.
    pub epoch_length: u64,
    /// Total reward paid out at each epoch. This will be split among all validators during the last
    /// epoch.
    /// (epoch_reward.amount * 86_400 * 30 / epoch_length) is the amount of reward tokens to mint
    /// each month.
    /// Ensure this is sensible in relation to the total token supply.
    pub epoch_reward: Coin,

    /// Initial operators and validator keys registered.
    /// If you do not set this, the validators need to register themselves before
    /// making this privileged/calling the EndBlockers, so that we have a non-empty validator set
    pub initial_keys: Vec<OperatorInitInfo>,

    /// A scaling factor to multiply cw4-group points to produce the Tendermint validator power
    pub scaling: Option<u32>,

    /// Percentage of total accumulated fees that is subtracted from tokens minted as rewards.
    /// 50% by default. To disable this feature just set it to 0 (which effectively means that fees
    /// don't affect the per-epoch reward).
    #[serde(default = "default_fee_percentage")]
    pub fee_percentage: Decimal,

    /// Flag determining if validators should be automatically unjailed after the jailing period;
    /// false by default.
    #[serde(default)]
    pub auto_unjail: bool,

    /// Validators who are caught double signing are jailed forever and their bonded tokens are
    /// slashed based on this value.
    #[serde(default = "default_double_sign_slash")]
    pub double_sign_slash_ratio: Decimal,

    /// Addresses where part of the reward for non-validators is sent for further distribution. These are
    /// required to handle the `Distribute {}` message (eg. tg4-engagement contract) which would
    /// distribute the funds sent with this message.
    ///
    /// The sum of ratios here has to be in the [0, 1] range. The remainder is sent to validators via the
    /// rewards contract.
    ///
    /// Note that the particular algorithm this contract uses calculates token rewards for distribution
    /// contracts by applying decimal division to the pool of reward tokens, and then passes the remainder
    /// to validators via the contract instantiated from `rewards_code_is`. This will cause edge cases where
    /// indivisible tokens end up with the validators. For example if the reward pool for an epoch is 1 token
    /// and there are two distribution contracts with 50% ratio each, that token will end up with the
    /// validators.
    pub distribution_contracts: UnvalidatedDistributionContracts,

    /// Code id of the contract which would be used to distribute the rewards of this token, assuming
    /// `tg4-engagement`. The contract will be initialized with the message:
    /// ```json
    /// {
    ///     "admin": "valset_addr",
    ///     "denom": "reward_denom",
    /// }
    /// ```
    ///
    /// This contract has to support all the `RewardsDistribution` messages
    pub validator_group_code_id: u64,

    /// When a validator joins the valset, verify they sign the first block since joining
    /// or jail them for a period otherwise.
    ///
    /// The verification happens every time the validator becomes an active validator,
    /// including when they are unjailed or when they just gain enough power to participate.
    pub verify_validators: bool,

    /// The duration to jail a validator for in case they don't sign their first epoch
    /// boundary block. After the period, they have to pass verification again, ad infinitum.
    pub offline_jail_duration: Duration,
}

impl InstantiateMsg {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.epoch_length == 0 {
            return Err(ContractError::InvalidEpoch {});
        }
        if self.min_points == 0 {
            return Err(ContractError::InvalidMinPoints {});
        }
        if self.max_validators == 0 {
            return Err(ContractError::InvalidMaxValidators {});
        }
        if self.scaling == Some(0) {
            return Err(ContractError::InvalidScaling {});
        }
        // Current denom regexp in the SDK is [a-zA-Z][a-zA-Z0-9/]{2,127}
        if self.epoch_reward.denom.len() < 2 || self.epoch_reward.denom.len() > 127 {
            return Err(ContractError::InvalidRewardDenom {});
        }
        for op in self.initial_keys.iter() {
            op.validate()?
        }
        Ok(())
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    /// Change the admin
    UpdateAdmin {
        admin: Option<String>,
    },
    /// Alter config values
    UpdateConfig {
        /// minimum points needed by an address in `membership` to be considered for the validator set.
        /// 0-point members are always filtered out.
        min_points: Option<u64>,
        /// The maximum number of validators that can be included in the Tendermint validator set.
        /// If there are more validators than slots, we select the top N by membership points
        /// descending.
        max_validators: Option<u32>,
        /// A scaling factor to multiply tg4-engagement points to produce the tendermint validator power
        scaling: Option<u32>,
        /// Total reward paid out each epoch. This will be split among all validators during the last
        /// epoch.
        /// (epoch_reward.amount * 86_400 * 30 / epoch_length) is reward tokens to mint each month.
        /// Ensure this is sensible in relation to the total token supply.
        epoch_reward: Option<Coin>,
        /// Percentage of total accumulated fees which is subtracted from tokens minted as a rewards.
        /// 50% as default. To disable this feature just set it to 0 (which effectively means that fees
        /// doesn't affect the per epoch reward).
        fee_percentage: Option<Decimal>,
        /// Flag determining if validators should be automatically unjailed after jailing period, false
        /// by default.
        auto_unjail: Option<bool>,

        /// Validators who are caught double signing are jailed forever and their bonded tokens are
        /// slashed based on this value.
        double_sign_slash_ratio: Option<Decimal>,

        /// Addresses where part of the reward for non-validators is sent for further distribution. These are
        /// required to handle the `Distribute {}` message (eg. tg4-engagement contract) which would
        /// distribute the funds sent with this message.
        /// The sum of ratios here has to be in the [0, 1] range. The remainder is sent to validators via the
        /// rewards contract.
        distribution_contracts: Option<Vec<DistributionContract>>,

        /// If this is enabled, signed blocks are watched for, and if a validator fails to sign any blocks
        /// in a string of a number of blocks (typically 1000 blocks), they are jailed.
        verify_validators: Option<bool>,

        /// The duration to jail a validator for in case they don't sign any blocks for a period of time,
        /// if `verify_validators` is enabled.
        /// After the jailing period, they will be jailed again if not signing blocks, ad infinitum.
        offline_jail_duration: Option<Duration>,
    },
    /// Links info.sender (operator) to this Tendermint consensus key.
    /// The operator cannot re-register another key.
    /// No two operators may have the same consensus_key.
    RegisterValidatorKey {
        pubkey: Pubkey,
        /// Additional metadata assigned to this validator
        metadata: ValidatorMetadata,
    },
    UpdateMetadata(ValidatorMetadata),
    /// Jails validator. Can be executed only by the admin.
    Jail {
        /// Operator which should be jailed
        operator: String,
        /// Duration for how long validator is jailed
        duration: JailingDuration,
    },
    /// Unjails validator. Admin can unjail anyone anytime, others can unjail only themselves and
    /// only if the jail period passed.
    Unjail {
        /// Address to unjail. Optional, as if not provided it is assumed to be the sender of the
        /// message (for convenience when unjailing self after the jail period).
        operator: Option<String>,
    },
    /// To be called by admin only. Slashes a given address (by forwarding slash to both rewards
    /// contract and engagement contract)
    Slash {
        addr: String,
        portion: Decimal,
    },

    /// This will update the validator set with the passed list.
    /// Used for testing validators storage.
    #[cfg(debug_assertions)]
    SimulateValidatorSet {
        validators: Vec<ValidatorInfo>,
    },
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    /// Returns configuration
    Configuration {},
    /// Returns EpochResponse - get info on current and next epochs
    Epoch {},

    /// Returns the validator key and associated metadata (if present) for the given operator.
    /// Returns ValidatorResponse
    Validator { operator: String },
    /// Paginate over all operators, using operator address as pagination.
    /// Returns ListValidatorsResponse
    ListValidators {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// List the current validator set, sorted by power descending
    ListActiveValidators {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// Returns ListValidatorsResponse
    ListJailedValidators {
        start_after: Option<String>,
        limit: Option<u32>,
    },

    /// This will calculate who the new validators would be if
    /// we recalculated end block right now.
    /// Also returns ListActiveValidatorsResponse
    SimulateActiveValidators {},

    /// Returns a list of validator slashing events.
    /// Returns ListValidatorSlashingResponse
    ListValidatorSlashing { operator: String },

    /// Returns cw_controllers::AdminResponse
    Admin {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct UnvalidatedDistributionContract {
    /// The unvalidated address of the contract to which part of the reward tokens is sent to.
    pub contract: String,
    /// The ratio of total reward tokens for an epoch to be sent to that contract for further
    /// distribution.
    pub ratio: Decimal,
}

impl UnvalidatedDistributionContract {
    fn validate(self, api: &dyn Api) -> Result<DistributionContract, ContractError> {
        Ok(DistributionContract {
            contract: api.addr_validate(&self.contract)?,
            ratio: self.ratio,
        })
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug, Default)]
#[serde(transparent)]
pub struct UnvalidatedDistributionContracts {
    pub inner: Vec<UnvalidatedDistributionContract>,
}

impl UnvalidatedDistributionContracts {
    /// Validates the addresses and the sum of ratios.
    pub fn validate(self, api: &dyn Api) -> Result<Vec<DistributionContract>, ContractError> {
        if self.sum_ratios() > Decimal::one() {
            return Err(ContractError::InvalidRewardsRatio {});
        }

        self.inner.into_iter().map(|c| c.validate(api)).collect()
    }

    fn sum_ratios(&self) -> Decimal {
        self.inner
            .iter()
            .map(|c| c.ratio)
            .fold(Decimal::zero(), Decimal::add)
    }
}

pub fn default_fee_percentage() -> Decimal {
    Decimal::zero()
}

pub fn default_validators_reward_ratio() -> Decimal {
    Decimal::one()
}

pub fn default_double_sign_slash() -> Decimal {
    Decimal::percent(50)
}

/// Validator Metadata modeled after the Cosmos SDK staking module
#[derive(
    Serialize, Deserialize, Clone, Eq, PartialEq, Ord, PartialOrd, JsonSchema, Debug, Default,
)]
pub struct ValidatorMetadata {
    /// The validator's name (required)
    pub moniker: String,

    /// The optional identity signature (ex. UPort or Keybase)
    pub identity: Option<String>,

    /// The validator's (optional) website
    pub website: Option<String>,

    /// The validator's (optional) security contact email
    pub security_contact: Option<String>,

    /// The validator's (optional) details
    pub details: Option<String>,
}

pub const MIN_MONIKER_LENGTH: usize = 3;
pub const MIN_METADATA_SIZE: usize = 1;
pub const MAX_METADATA_SIZE: usize = 256;

impl ValidatorMetadata {
    pub fn validate(&self) -> Result<(), ContractError> {
        if self.moniker.len() < MIN_MONIKER_LENGTH || self.moniker.len() > MAX_METADATA_SIZE {
            return Err(ContractError::InvalidMetadata {
                data: "moniker",
                min: MIN_MONIKER_LENGTH,
                max: MAX_METADATA_SIZE,
            });
        }
        if let Some(identity) = &self.identity {
            if identity.is_empty() || identity.len() > MAX_METADATA_SIZE {
                return Err(ContractError::InvalidMetadata {
                    data: "identity",
                    min: MIN_METADATA_SIZE,
                    max: MAX_METADATA_SIZE,
                });
            }
        }
        if let Some(website) = &self.website {
            if website.is_empty() || website.len() > MAX_METADATA_SIZE {
                return Err(ContractError::InvalidMetadata {
                    data: "website",
                    min: MIN_METADATA_SIZE,
                    max: MAX_METADATA_SIZE,
                });
            } else if !website.starts_with("https://") && !website.starts_with("http://") {
                return Err(ContractError::InvalidMetadataWebsitePrefix {});
            }
        }
        if let Some(security_contract) = &self.security_contact {
            if security_contract.is_empty() || security_contract.len() > MAX_METADATA_SIZE {
                return Err(ContractError::InvalidMetadata {
                    data: "security_contract",
                    min: MIN_METADATA_SIZE,
                    max: MAX_METADATA_SIZE,
                });
            }
        }
        if let Some(details) = &self.details {
            if details.is_empty() || details.len() > MAX_METADATA_SIZE {
                return Err(ContractError::InvalidMetadata {
                    data: "details",
                    min: MIN_METADATA_SIZE,
                    max: MAX_METADATA_SIZE,
                });
            }
        }
        Ok(())
    }
}

/// Maps an sdk address to a Tendermint pubkey.
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct OperatorInitInfo {
    pub operator: String,
    pub validator_pubkey: Pubkey,
    pub metadata: ValidatorMetadata,
}

impl OperatorInitInfo {
    pub fn validate(&self) -> Result<(), ContractError> {
        Ed25519Pubkey::try_from(&self.validator_pubkey)?;
        self.metadata.validate()
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct EpochResponse {
    /// Number of seconds in one epoch. We update the Tendermint validator set only once per epoch.
    pub epoch_length: u64,
    /// The current epoch # (env.block.time/epoch_length, rounding down)
    pub current_epoch: u64,
    /// The last time we updated the validator set - block time and height
    pub last_update_time: u64,
    pub last_update_height: u64,
    /// Seconds (UTC UNIX time) of next timestamp that will trigger a validator recalculation
    pub next_update_time: u64,
}

// data behind one operator
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct OperatorResponse {
    pub operator: String,
    pub pubkey: Pubkey,
    pub metadata: ValidatorMetadata,
    pub jailed_until: Option<JailingPeriod>,
    pub active_validator: bool,
}

impl OperatorResponse {
    pub fn from_info(
        info: OperatorInfo,
        operator: String,
        jailed_until: impl Into<Option<JailingPeriod>>,
    ) -> Self {
        OperatorResponse {
            operator,
            pubkey: info.pubkey.into(),
            metadata: info.metadata,
            jailed_until: jailed_until.into(),
            active_validator: info.active_validator,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct JailingPeriod {
    pub start: Timestamp,
    pub end: JailingEnd,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum JailingEnd {
    Until(Expiration),
    Forever {},
}

impl JailingPeriod {
    pub fn from_duration(duration: JailingDuration, block: &BlockInfo) -> Self {
        Self {
            start: block.time,
            end: match duration {
                JailingDuration::Duration(duration) => JailingEnd::Until(duration.after(block)),
                JailingDuration::Forever {} => JailingEnd::Forever {},
            },
        }
    }

    pub fn is_forever(&self) -> bool {
        matches!(self.end, JailingEnd::Forever {})
    }

    pub fn is_expired(&self, block: &BlockInfo) -> bool {
        match self.end {
            JailingEnd::Forever {} => false,
            JailingEnd::Until(expires) => expires.is_expired(block),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ValidatorResponse {
    /// This is unset if no validator registered
    pub validator: Option<OperatorResponse>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ListValidatorResponse {
    pub validators: Vec<OperatorResponse>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ListActiveValidatorsResponse {
    pub validators: Vec<ValidatorInfo>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
pub struct ListValidatorSlashingResponse {
    /// Operator address
    pub addr: String,
    /// Block height of first validator addition to validators set
    pub start_height: u64,
    /// Slashing events, if any
    pub slashing: Vec<ValidatorSlashing>,
    /// Whether or not a validator has been tombstoned (killed out of
    /// validator set)
    pub tombstoned: bool,
    /// If validator is jailed, it will show expiration time
    pub jailed_until: Option<Expiration>,
}

/// Messages sent by this contract to an external contract
#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum DistributionMsg {
    /// Message sent to `distribution_contract` with funds which are part of the reward to be split
    /// between engaged operators
    DistributeRewards {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct RewardsInstantiateMsg {
    pub admin: Addr,
    pub denom: String,
    pub members: Vec<Member>,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum RewardsDistribution {
    UpdateMembers {
        remove: Vec<String>,
        add: Vec<Member>,
    },
    DistributeRewards {},
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct InstantiateResponse {
    pub validator_group: Addr,
}

#[derive(Serialize, Deserialize, Clone, PartialEq, Eq, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub struct MigrateMsg {
    pub min_points: Option<u64>,
    pub max_validators: Option<u32>,
    pub distribution_contracts: Option<Vec<DistributionContract>>,
    pub verify_validators: Option<bool>,
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::error::ContractError;
    use crate::test_helpers::{invalid_operator, valid_operator};
    use cosmwasm_std::coin;

    #[test]
    fn validate_operator_key() {
        valid_operator("foo").validate().unwrap();
        let err = invalid_operator().validate().unwrap_err();
        assert_eq!(err, ContractError::InvalidPubkey {});
    }

    #[test]
    fn validate_init_msg() {
        let proper = InstantiateMsg {
            admin: None,
            membership: "contract-addr".into(),
            min_points: 5,
            max_validators: 20,
            epoch_length: 5000,
            epoch_reward: coin(7777, "foobar"),
            initial_keys: vec![valid_operator("foo"), valid_operator("bar")],
            scaling: None,
            fee_percentage: Decimal::zero(),
            auto_unjail: false,
            double_sign_slash_ratio: Decimal::percent(50),
            distribution_contracts: UnvalidatedDistributionContracts::default(),
            validator_group_code_id: 0,
            verify_validators: false,
            offline_jail_duration: Duration::new(0),
        };
        proper.validate().unwrap();

        // with scaling also works
        let mut with_scaling = proper.clone();
        with_scaling.scaling = Some(10);
        with_scaling.validate().unwrap();

        // fails on 0 scaling
        let mut invalid = proper.clone();
        invalid.scaling = Some(0);
        let err = invalid.validate().unwrap_err();
        assert_eq!(err, ContractError::InvalidScaling {});

        // fails on 0 min points
        let mut invalid = proper.clone();
        invalid.min_points = 0;
        let err = invalid.validate().unwrap_err();
        assert_eq!(err, ContractError::InvalidMinPoints {});

        // fails on 0 max validators
        let mut invalid = proper.clone();
        invalid.max_validators = 0;
        let err = invalid.validate().unwrap_err();
        assert_eq!(err, ContractError::InvalidMaxValidators {});

        // fails on 0 epoch size
        let mut invalid = proper.clone();
        invalid.epoch_length = 0;
        let err = invalid.validate().unwrap_err();
        assert_eq!(err, ContractError::InvalidEpoch {});

        // allows no operators
        let mut no_operators = proper.clone();
        no_operators.initial_keys = vec![];
        no_operators.validate().unwrap();

        // fails on invalid operator
        let mut invalid = proper.clone();
        invalid.initial_keys = vec![valid_operator("foo"), invalid_operator()];
        let err = invalid.validate().unwrap_err();
        assert_eq!(err, ContractError::InvalidPubkey {});

        // fails if no denom set for reward
        let mut invalid = proper;
        invalid.epoch_reward.denom = "".into();
        let err = invalid.validate().unwrap_err();
        assert_eq!(err, ContractError::InvalidRewardDenom {});
    }

    #[test]
    fn validate_metadata() {
        let meta = ValidatorMetadata {
            moniker: "example".to_owned(),
            identity: Some((0..MAX_METADATA_SIZE + 1).map(|_| "X").collect::<String>()),
            website: Some((0..MAX_METADATA_SIZE + 1).map(|_| "X").collect::<String>()),
            security_contact: Some((0..MAX_METADATA_SIZE + 1).map(|_| "X").collect::<String>()),
            details: Some((0..MAX_METADATA_SIZE + 1).map(|_| "X").collect::<String>()),
        };
        let resp = meta.validate().unwrap_err();
        assert_eq!(
            ContractError::InvalidMetadata {
                data: "identity",
                min: MIN_METADATA_SIZE,
                max: MAX_METADATA_SIZE
            },
            resp
        );

        let meta = ValidatorMetadata {
            identity: Some("identity".to_owned()),
            ..meta
        };
        let resp = meta.validate().unwrap_err();
        assert_eq!(
            ContractError::InvalidMetadata {
                data: "website",
                min: MIN_METADATA_SIZE,
                max: MAX_METADATA_SIZE,
            },
            resp
        );

        let meta = ValidatorMetadata {
            website: Some("https://website".to_owned()),
            ..meta
        };
        let resp = meta.validate().unwrap_err();
        assert_eq!(
            ContractError::InvalidMetadata {
                data: "security_contract",
                min: MIN_METADATA_SIZE,
                max: MAX_METADATA_SIZE,
            },
            resp
        );

        let meta = ValidatorMetadata {
            security_contact: Some("contract".to_owned()),
            ..meta
        };
        let resp = meta.validate().unwrap_err();
        assert_eq!(
            ContractError::InvalidMetadata {
                data: "details",
                min: MIN_METADATA_SIZE,
                max: MAX_METADATA_SIZE,
            },
            resp
        );

        let meta = ValidatorMetadata {
            identity: Some(String::new()),
            website: Some(String::new()),
            security_contact: Some(String::new()),
            details: Some(String::new()),
            ..meta
        };
        let resp = meta.validate().unwrap_err();
        assert_eq!(
            ContractError::InvalidMetadata {
                data: "identity",
                min: MIN_METADATA_SIZE,
                max: MAX_METADATA_SIZE
            },
            resp
        );

        let meta = ValidatorMetadata {
            identity: Some("identity".to_owned()),
            ..meta
        };
        let resp = meta.validate().unwrap_err();
        assert_eq!(
            ContractError::InvalidMetadata {
                data: "website",
                min: MIN_METADATA_SIZE,
                max: MAX_METADATA_SIZE,
            },
            resp
        );

        let meta = ValidatorMetadata {
            website: Some("http://website".to_owned()),
            ..meta
        };
        let resp = meta.validate().unwrap_err();
        assert_eq!(
            ContractError::InvalidMetadata {
                data: "security_contract",
                min: MIN_METADATA_SIZE,
                max: MAX_METADATA_SIZE,
            },
            resp
        );

        let meta = ValidatorMetadata {
            security_contact: Some("contract".to_owned()),
            ..meta
        };
        let resp = meta.validate().unwrap_err();
        assert_eq!(
            ContractError::InvalidMetadata {
                data: "details",
                min: MIN_METADATA_SIZE,
                max: MAX_METADATA_SIZE,
            },
            resp
        );

        let meta = ValidatorMetadata {
            website: Some("website".to_owned()),
            ..meta
        };
        let resp = meta.validate().unwrap_err();
        assert_eq!(ContractError::InvalidMetadataWebsitePrefix {}, resp);
    }
}
