// Copied from cw-plus repository: https://github.com/CosmWasm/cw-plus/tree/main/packages/controllers
// Original file distributed on Apache license

use itertools::Itertools;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::{
    Addr, BlockInfo, CustomQuery, Decimal, Deps, Order, StdResult, Storage, Uint128,
};
use cw_storage_plus::{Bound, Index, IndexList, IndexedMap, MultiIndex, PrefixBound};
use tg_utils::Expiration;

// settings for pagination
const MAX_LIMIT: u32 = 100;
const DEFAULT_LIMIT: u32 = 30;

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct TokenReleaseInfo {
    pub addr: Addr,
    pub amount: Uint128,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ReleaseData {
    pub liquid_releases: Vec<TokenReleaseInfo>,
    pub vesting_releases: Vec<TokenReleaseInfo>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
pub struct Claim {
    /// Address owning the claim
    pub addr: Addr,
    /// Liquid amount of tokens in claim
    pub amount: Uint128,
    /// Vesting amount of tokens in claim
    pub vesting_amount: Option<Uint128>,
    /// Release time of the claim. Originally in `cw_controllers` it is an `Expiration` type, but
    /// here we need to query for claims via release time, and expiration is impossible to be
    /// properly sorted, as it is impossible to properly compare expiration by height and
    /// expiration by time.
    pub release_at: Expiration,
    /// Height of a blockchain in a moment of creation of this claim
    pub creation_height: u64,
}

struct ClaimIndexes<'a> {
    // Last type param defines the pk deserialization type
    pub release_at: MultiIndex<'a, u64, Claim, (Addr, u64)>,
}

impl<'a> IndexList<Claim> for ClaimIndexes<'a> {
    fn get_indexes(&'_ self) -> Box<dyn Iterator<Item = &'_ dyn Index<Claim>> + '_> {
        let v: Vec<&dyn Index<Claim>> = vec![&self.release_at];
        Box::new(v.into_iter())
    }
}

impl Claim {
    pub fn new(
        addr: Addr,
        amount: u128,
        vesting_amount: u128,
        released: Expiration,
        creation_height: u64,
    ) -> Self {
        Claim {
            addr,
            amount: amount.into(),
            vesting_amount: Some(vesting_amount.into()),
            release_at: released,
            creation_height,
        }
    }
}

pub struct Claims<'a> {
    /// Claims are indexed by `(addr, release_at)` pair. Claims falling into the same key are
    /// merged (summarized) as there is no point to distinguish them.
    claims: IndexedMap<'a, (&'a Addr, u64), Claim, ClaimIndexes<'a>>,
}

impl<'a> Claims<'a> {
    pub fn new(storage_key: &'a str, release_subkey: &'a str) -> Self {
        let indexes = ClaimIndexes {
            release_at: MultiIndex::new(
                |_, claim| claim.release_at.as_key(),
                storage_key,
                release_subkey,
            ),
        };
        let claims = IndexedMap::new(storage_key, indexes);

        Self { claims }
    }

    /// This creates a claim, such that the given address can claim an amount of tokens after
    /// the release date.
    pub fn create_claim(
        &self,
        storage: &mut dyn Storage,
        addr: Addr,
        amount: Uint128,
        vesting_amount: Uint128,
        release_at: Expiration,
        creation_height: u64,
    ) -> StdResult<()> {
        let addr = &addr;
        // Add a claim to this user to get their tokens after the unbonding period
        self.claims.update(
            storage,
            (addr, release_at.as_key()),
            move |claim| -> StdResult<_> {
                match claim {
                    Some(mut claim) => {
                        claim.amount += amount;
                        claim.vesting_amount =
                            Some(claim.vesting_amount.unwrap_or_default() + vesting_amount);
                        Ok(claim)
                    }
                    None => Ok(Claim {
                        addr: addr.clone(),
                        amount,
                        vesting_amount: Some(vesting_amount),
                        release_at,
                        creation_height,
                    }),
                }
            },
        )?;

        Ok(())
    }

    /// This iterates over all mature claims for the address, and removes them, up to an optional limit.
    /// It removes the finished claims and returns the total amount of tokens to be released.
    pub fn claim_addr(
        &self,
        storage: &mut dyn Storage,
        addr: &Addr,
        block: &BlockInfo,
        limit: impl Into<Option<u64>>,
    ) -> StdResult<(Uint128, Uint128)> {
        let claims = self
            .claims
            .prefix(addr)
            // take all claims for the addr
            .range_raw(
                storage,
                None,
                Some(Bound::inclusive(Expiration::now(block).as_key())),
                Order::Ascending,
            );

        let claims = self.collect_claims(claims, limit.into())?;
        let amount = claims.iter().map(|claim| claim.amount).sum();
        let vesting_amount = claims
            .iter()
            .map(|claim| claim.vesting_amount.unwrap_or_default())
            .sum();

        self.release_claims(storage, claims)?;

        Ok((amount, vesting_amount))
    }

    /// This iterates over all mature claims of any addresses, and removes them. Up to `limit`
    /// claims would be processed, starting from the oldest. It removes the finished claims and
    /// returns a pair of vectors representing the amounts of liquid and vesting tokens
    /// to be released to particular addresses.
    pub(crate) fn claim_expired(
        &self,
        storage: &mut dyn Storage,
        block: &BlockInfo,
        limit: impl Into<Option<u64>>,
    ) -> StdResult<ReleaseData> {
        let claims = self
            .claims
            .idx
            .release_at
            // take all claims which are expired (at most same timestamp as current block)
            .prefix_range_raw(
                storage,
                None,
                Some(PrefixBound::inclusive(block.time.nanos())),
                Order::Ascending,
            );

        let mut claims = self.collect_claims(claims, limit.into())?;
        claims.sort_by_key(|claim| claim.addr.clone());

        let liquid_releases = claims
            .iter()
            // TODO: use `slice::group_by` in place of `Itertools::group_by` when `slice_group_by`
            // is stabilized [https://github.com/rust-lang/rust/issues/80552]
            .group_by(|claim| &claim.addr)
            .into_iter()
            .map(|(addr, group)| TokenReleaseInfo {
                addr: addr.clone(),
                amount: group.map(|claim| claim.amount).sum(),
            })
            .collect();

        let vesting_releases = claims
            .iter()
            // TODO: use `slice::group_by` in place of `Itertools::group_by` when `slice_group_by`
            // is stabilized [https://github.com/rust-lang/rust/issues/80552]
            .group_by(|claim| &claim.addr)
            .into_iter()
            .map(|(addr, group)| TokenReleaseInfo {
                addr: addr.clone(),
                amount: group
                    .map(|claim| claim.vesting_amount.unwrap_or_default())
                    .sum(),
            })
            .collect();

        self.release_claims(storage, claims)?;

        let release_data = ReleaseData {
            liquid_releases,
            vesting_releases,
        };

        Ok(release_data)
    }

    /// Processes claims filtering those which are to be released. Returns vector of claims to be
    /// released
    fn collect_claims(
        &self,
        claims: impl IntoIterator<Item = StdResult<(Vec<u8>, Claim)>>,
        limit: Option<u64>,
    ) -> StdResult<Vec<Claim>> {
        // apply limit and collect - it is needed to collect intermediately, as it is impossible to
        // remove from map while iterating as it borrows map internally; collecting to result, so
        // it returns early on failure; collecting would also trigger a final map, so amount would
        // be properly fulfilled
        let claims = claims.into_iter().map(|r| r.map(|(_, c)| c));
        if let Some(limit) = limit {
            claims.take(limit as usize).collect()
        } else {
            claims.collect()
        }
    }

    /// Releases given claims by removing them from storage
    fn release_claims(
        &self,
        storage: &mut dyn Storage,
        claims: impl IntoIterator<Item = Claim>,
    ) -> StdResult<()> {
        for claim in claims {
            self.claims
                .remove(storage, (&claim.addr, claim.release_at.as_key()))?;
        }

        Ok(())
    }

    pub fn slash_claims_for_addr(
        &self,
        storage: &mut dyn Storage,
        address: Addr,
        portion: Decimal,
    ) -> StdResult<(Uint128, Uint128)> {
        let claims: StdResult<Vec<_>> = self
            .claims
            .prefix(&address)
            .range(storage, None, None, Order::Ascending)
            .collect();
        let claims = claims?;

        let mut total_slashed = Uint128::zero();
        let mut total_vesting_slashed = Uint128::zero();

        for (release_at, claim) in claims {
            let key = (&address, release_at);

            let slashed = claim.amount * portion;
            let vesting_slashed = claim.vesting_amount.unwrap_or_default() * portion;
            let mut new_claim = claim.clone();
            new_claim.amount -= slashed;
            new_claim.vesting_amount =
                Some(claim.vesting_amount.unwrap_or_default() - vesting_slashed);

            self.claims
                .replace(storage, key, Some(&new_claim), Some(&claim))?;

            total_slashed += slashed;
            total_vesting_slashed += vesting_slashed;
        }

        Ok((total_slashed, total_vesting_slashed))
    }

    pub fn query_claims<Q: CustomQuery>(
        &self,
        deps: Deps<Q>,
        address: Addr,
        limit: Option<u32>,
        start_after: Option<Expiration>,
    ) -> StdResult<Vec<Claim>> {
        let limit = limit.unwrap_or(DEFAULT_LIMIT).min(MAX_LIMIT) as usize;
        let start = start_after.map(|s| Bound::exclusive(s.as_key()));

        self.claims
            .prefix(&address)
            .range(deps.storage, start, None, Order::Ascending)
            .map(|claim| match claim {
                Ok((_, claim)) => Ok(claim),
                Err(err) => Err(err),
            })
            .take(limit)
            .collect()
    }
}
