# Changelog

## [Unreleased](https://github.com/confio/poe-contracts/tree/HEAD)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.16.0...HEAD)

## [v0.16.0](https://github.com/confio/poe-contracts/tree/v0.16.0) (2022-12-16)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.15.2...v0.16.0)

**Fixed bugs:**

- Fix claims auto-release of vesting amounts [\#198](https://github.com/confio/poe-contracts/pull/198) ([maurolacy](https://github.com/maurolacy))

## [v0.15.2](https://github.com/confio/poe-contracts/tree/v0.15.2) (2022-12-09)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.15.1...v0.15.2)

**Closed issues:**

- Update LICENSE to Apache [\#192](https://github.com/confio/poe-contracts/issues/192)

**Merged pull requests:**

- Add MigrateMsg with update config options [\#195](https://github.com/confio/poe-contracts/pull/195) ([maurolacy](https://github.com/maurolacy))
- Replace closed source license with Apache 2.0 [\#193](https://github.com/confio/poe-contracts/pull/193) ([ethanfrey](https://github.com/ethanfrey))

## [v0.15.1](https://github.com/confio/poe-contracts/tree/v0.15.1) (2022-09-22)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.15.0...v0.15.1)

**Merged pull requests:**

- Fix/contract name [\#189](https://github.com/confio/poe-contracts/pull/189) ([maurolacy](https://github.com/maurolacy))
- Add promote / demote contract to privileged gov proposal [\#188](https://github.com/confio/poe-contracts/pull/188) ([maurolacy](https://github.com/maurolacy))

## [v0.15.0](https://github.com/confio/poe-contracts/tree/v0.15.0) (2022-09-19)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.14.0...v0.15.0)

**Closed issues:**

- tgrade-valset: Wrong automatic jailing logic [\#184](https://github.com/confio/poe-contracts/issues/184)

**Merged pull requests:**

- tgrade-valset migrate contract update [\#186](https://github.com/confio/poe-contracts/pull/186) ([maurolacy](https://github.com/maurolacy))
- Update config extensions [\#185](https://github.com/confio/poe-contracts/pull/185) ([maurolacy](https://github.com/maurolacy))

## [v0.14.0](https://github.com/confio/poe-contracts/tree/v0.14.0) (2022-09-13)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.13.0...v0.14.0)

**Fixed bugs:**

- Zero amount claims during release [\#180](https://github.com/confio/poe-contracts/issues/180)

**Closed issues:**

- Backport multisig fixes to voting contracts [\#176](https://github.com/confio/poe-contracts/issues/176)
- Benchmark `Ed25519Pubkey::to_address` [\#172](https://github.com/confio/poe-contracts/issues/172)
- \[tg4-stake\] Contract migration to integrate unbonding of vesting tokens [\#170](https://github.com/confio/poe-contracts/issues/170)
- Deprecate `ensure_from_older_version` [\#165](https://github.com/confio/poe-contracts/issues/165)
- Validator proposal creation should fail when description is empty [\#155](https://github.com/confio/poe-contracts/issues/155)
- valset: do not jail validators on first missed vote with "VerifyValidators" enabled [\#149](https://github.com/confio/poe-contracts/issues/149)

**Merged pull requests:**

- Fix zero amounts during unbound and release [\#179](https://github.com/confio/poe-contracts/pull/179) ([maurolacy](https://github.com/maurolacy))
- Backport multisig fixes to voting contracts [\#178](https://github.com/confio/poe-contracts/pull/178) ([0xFable](https://github.com/0xFable))
- Deprecate `ensure_from_older_version` [\#177](https://github.com/confio/poe-contracts/pull/177) ([ueco-jb](https://github.com/ueco-jb))
- Update mixer benchmark gas values [\#175](https://github.com/confio/poe-contracts/pull/175) ([maurolacy](https://github.com/maurolacy))
- Remove / replace coins\_to\_string helper [\#174](https://github.com/confio/poe-contracts/pull/174) ([maurolacy](https://github.com/maurolacy))
- Verify validators revisited [\#167](https://github.com/confio/poe-contracts/pull/167) ([maurolacy](https://github.com/maurolacy))

## [v0.13.0](https://github.com/confio/poe-contracts/tree/v0.13.0) (2022-07-26)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.12.0...v0.13.0)

**Closed issues:**

- bug when defining the jailing start time [\#162](https://github.com/confio/poe-contracts/issues/162)
- \[tgrade-validator-voting\] Add proposal validation during creation [\#156](https://github.com/confio/poe-contracts/issues/156)

**Merged pull requests:**

- Older version migration helper [\#164](https://github.com/confio/poe-contracts/pull/164) ([maurolacy](https://github.com/maurolacy))
- Validate proposal during creation [\#163](https://github.com/confio/poe-contracts/pull/163) ([maurolacy](https://github.com/maurolacy))

## [v0.12.0](https://github.com/confio/poe-contracts/tree/v0.12.0) (2022-07-14)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.11.0...v0.12.0)

**Closed issues:**

- Review TODOs / FIXMEs [\#138](https://github.com/confio/poe-contracts/issues/138)

**Merged pull requests:**

- TODO / FIXME review [\#160](https://github.com/confio/poe-contracts/pull/160) ([maurolacy](https://github.com/maurolacy))
- Tgrade gov change params 2 [\#159](https://github.com/confio/poe-contracts/pull/159) ([maurolacy](https://github.com/maurolacy))
- Implement the ChangeParams proposal [\#157](https://github.com/confio/poe-contracts/pull/157) ([maurolacy](https://github.com/maurolacy))

## [v0.11.0](https://github.com/confio/poe-contracts/tree/v0.11.0) (2022-06-10)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.10.0...v0.11.0)

**Breaking changes:**

- Rename rewards to points in mixer [\#151](https://github.com/confio/poe-contracts/pull/151) ([maurolacy](https://github.com/maurolacy))
- Remove denom from the `TotalPoints` query [\#150](https://github.com/confio/poe-contracts/pull/150) ([maurolacy](https://github.com/maurolacy))

**Fixed bugs:**

- \[tg4-stake\] TotalPoints query shouldn't return denom [\#147](https://github.com/confio/poe-contracts/issues/147)
- valset: errors with "VerifyValidators" enabled on startup block 0 [\#146](https://github.com/confio/poe-contracts/issues/146)

**Closed issues:**

- Create documentation for `voting-contract` [\#59](https://github.com/confio/poe-contracts/issues/59)

**Merged pull requests:**

- Mixer mixing fn benches [\#152](https://github.com/confio/poe-contracts/pull/152) ([maurolacy](https://github.com/maurolacy))

## [v0.10.0](https://github.com/confio/poe-contracts/tree/v0.10.0) (2022-06-02)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.9.0...v0.10.0)

**Fixed bugs:**

- Valset: export/ import `admin` [\#139](https://github.com/confio/poe-contracts/issues/139)

**Merged pull requests:**

- Add missing Admin to state export / import [\#141](https://github.com/confio/poe-contracts/pull/141) ([maurolacy](https://github.com/maurolacy))
- \[tgrade-valset\] Offline validator auto-unjail tests [\#140](https://github.com/confio/poe-contracts/pull/140) ([maurolacy](https://github.com/maurolacy))

## [v0.9.0](https://github.com/confio/poe-contracts/tree/v0.9.0) (2022-04-20)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.8.1...v0.9.0)

**Breaking changes:**

- Ensure validator is online before adding to the active set [\#36](https://github.com/confio/poe-contracts/issues/36)
- valset: fair rule to determine active valset when equal EP/Stake [\#22](https://github.com/confio/poe-contracts/issues/22)
- Verify new validators are online [\#127](https://github.com/confio/poe-contracts/pull/127) ([uint](https://github.com/uint))
- Valset tie-breaking \#3 [\#118](https://github.com/confio/poe-contracts/pull/118) ([maurolacy](https://github.com/maurolacy))

**Merged pull requests:**

- Update to latest cw-plus [\#134](https://github.com/confio/poe-contracts/pull/134) ([maurolacy](https://github.com/maurolacy))
- `tgrade-valset` export import against `main` [\#133](https://github.com/confio/poe-contracts/pull/133) ([maurolacy](https://github.com/maurolacy))

## [v0.8.1](https://github.com/confio/poe-contracts/tree/v0.8.1) (2022-03-29)

Maintenance release. Wrong tgrade-valset version published to crates-io.

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.8.0...v0.8.1)

## [v0.8.0](https://github.com/confio/poe-contracts/tree/v0.8.0) (2022-03-29)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.6.2...v0.8.0)

**Closed issues:**

- grade-validator-voting: Operate on all contracts [\#119](https://github.com/confio/poe-contracts/issues/119)
- QA Steps to check breakages in CI [\#71](https://github.com/confio/poe-contracts/issues/71)
- Release 0.7.0-alpha1 [\#115](https://github.com/confio/poe-contracts/issues/115)
- Update to cw-plus 0.12.1 [\#90](https://github.com/confio/poe-contracts/issues/90)

**Merged pull requests:**

- Stake vesting accounts [\#124](https://github.com/confio/poe-contracts/pull/124) ([maurolacy](https://github.com/maurolacy))
- Release 0.7.0-alpha2 [\#117](https://github.com/confio/poe-contracts/pull/117) ([ueco-jb](https://github.com/ueco-jb))
- Release 0.7.0-alpha1 [\#116](https://github.com/confio/poe-contracts/pull/116) ([maurolacy](https://github.com/maurolacy))
- Replace ballots with indexed map [\#114](https://github.com/confio/poe-contracts/pull/114) ([ueco-jb](https://github.com/ueco-jb))
- Implement Querier for TgradeApp [\#111](https://github.com/confio/poe-contracts/pull/111) ([ethanfrey](https://github.com/ethanfrey))
- Add description to tgrade-gov-reflect's Cargo.toml [\#107](https://github.com/confio/poe-contracts/pull/107) ([ueco-jb](https://github.com/ueco-jb))
- Update to cw-plus 0.12.1 [\#103](https://github.com/confio/poe-contracts/pull/103) ([maurolacy](https://github.com/maurolacy))

**Breaking changes:**

- `JailingPeriod` - add information about start period [\#94](https://github.com/confio/poe-contracts/issues/94)
- Refactor `BALLOTS` and `BALLOTS_BY_VOTER` in `voting-contract` [\#83](https://github.com/confio/poe-contracts/issues/83)
- valset: store and expose the start of a jailing period [\#112](https://github.com/confio/poe-contracts/pull/112) ([uint](https://github.com/uint))

## [v0.6.2](https://github.com/confio/poe-contracts/tree/v0.6.2) (2022-02-18)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.6.1...v0.6.2)

**Merged pull requests:**

- Decustomize `tg4-group` [\#109](https://github.com/confio/poe-contracts/pull/109) ([maurolacy](https://github.com/maurolacy))

## [v0.6.1](https://github.com/confio/poe-contracts/tree/v0.6.1) (2022-02-16)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.6.0...v0.6.1)

**Closed issues:**

- Add API migration guide [\#70](https://github.com/confio/poe-contracts/issues/70)

**Merged pull requests:**

- Add `tg4-group` contract [\#105](https://github.com/confio/poe-contracts/pull/105) ([maurolacy](https://github.com/maurolacy))

## [v0.6.0](https://github.com/confio/poe-contracts/tree/v0.6.0) (2022-02-15)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.6.0-rc2...v0.6.0)

**Breaking changes:**

- Rename `Tg4Contract::total_weight` to `Tg4Contract::total_points` in `packages/tg4`.

- Some `Response` `attributes` renaming in `tg4-engagement`:
  - "distribute_tokens" -> "distribute_rewards"
  - "withdraw_tokens" -> "withdraw_rewards"
  - "token" -> "rewards"
  -  "weight" -> "points"

**Closed issues:**

- Refactor local names to match new naming rules [\#86](https://github.com/confio/poe-contracts/issues/86)

**Merged pull requests:**

- Aligning local names to APIs [\#102](https://github.com/confio/poe-contracts/pull/102) ([hashedone](https://github.com/hashedone))

## [v0.6.0-rc2](https://github.com/confio/poe-contracts/tree/v0.6.0-rc2) (2022-02-10)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.5.5...v0.6.0-rc2)

**Breaking changes:**

- Use specilaized tg3 version of voting API for tgrade contracts [\#85](https://github.com/confio/poe-contracts/issues/85)
- Valset config contract names [\#96](https://github.com/confio/poe-contracts/pull/96) ([maurolacy](https://github.com/maurolacy))
- tg3: Common voting interfaces for tgrade [\#93](https://github.com/confio/poe-contracts/pull/93) ([hashedone](https://github.com/hashedone))
- Rename API to points and rewards [\#50](https://github.com/confio/poe-contracts/pull/50) ([ethanfrey](https://github.com/ethanfrey))
- `valset`: Optimize VALIDATORS storage [\#42](https://github.com/confio/poe-contracts/issues/42)
- Optimize validators storage [\#61](https://github.com/confio/poe-contracts/pull/61) ([maurolacy](https://github.com/maurolacy))
- \[voting-contract\] Record proposal creator [\#31](https://github.com/confio/poe-contracts/issues/31)
- Store information about operators' validator status [\#37](https://github.com/confio/poe-contracts/pull/37) ([uint](https://github.com/uint))

**Closed issues:**

- Tag 0.6.0-rc1 [\#88](https://github.com/confio/poe-contracts/issues/88)
- Valset: add `ListJailedValidators` query [\#87](https://github.com/confio/poe-contracts/issues/87)
- Add `list_votes_by_voter` query to `voting_contract` [\#78](https://github.com/confio/poe-contracts/issues/78)
- Increase max limit [\#76](https://github.com/confio/poe-contracts/issues/76)
- Missing items found when updating Go code [\#75](https://github.com/confio/poe-contracts/issues/75)
- Tools to help build API Migration Guide [\#72](https://github.com/confio/poe-contracts/issues/72)
- \[tgrade-valset\] Metadata issues [\#66](https://github.com/confio/poe-contracts/issues/66)
- Tag v0.6.0-beta1 [\#67](https://github.com/confio/poe-contracts/issues/67)
- Voting's helper `update_status()` should save changed status [\#62](https://github.com/confio/poe-contracts/issues/62)
- \[voting-contract\] More status issues [\#55](https://github.com/confio/poe-contracts/issues/55)
- Validator Voting: check `migrate_msg` content to be \>0 [\#46](https://github.com/confio/poe-contracts/issues/46)
- Add Open Text Proposals [\#15](https://github.com/confio/poe-contracts/issues/15)

**Merged pull requests:**

- Release 0.6.0-rc2 [\#99](https://github.com/confio/poe-contracts/pull/99) ([ueco-jb](https://github.com/ueco-jb))
- tg3: align version [\#100](https://github.com/confio/poe-contracts/pull/100) ([uint](https://github.com/uint))
- Fix typo in publish.sh script [\#98](https://github.com/confio/poe-contracts/pull/98) ([ueco-jb](https://github.com/ueco-jb))
- Add tg3 package to publishing script [\#97](https://github.com/confio/poe-contracts/pull/97) ([maurolacy](https://github.com/maurolacy))
- Release 0.6.0-rc1 [\#95](https://github.com/confio/poe-contracts/pull/95) ([ueco-jb](https://github.com/ueco-jb))
- Valset: Implement ListJailedValidators query [\#92](https://github.com/confio/poe-contracts/pull/92) ([ueco-jb](https://github.com/ueco-jb))
- Valset: metadata validation [\#84](https://github.com/confio/poe-contracts/pull/84) ([ueco-jb](https://github.com/ueco-jb))
- Increase MAX\_LIMIT constant to 100 and DEFAULT\_LIMIT to 30 [\#80](https://github.com/confio/poe-contracts/pull/80) ([ueco-jb](https://github.com/ueco-jb))
- Voting Contract: Add list\_votes\_by\_voter query [\#79](https://github.com/confio/poe-contracts/pull/79) ([ueco-jb](https://github.com/ueco-jb))
- Feature-gated `SimulateValidatorSet` [\#77](https://github.com/confio/poe-contracts/pull/77) ([maurolacy](https://github.com/maurolacy))
- Add `diff_schema.sh` / `diff_state.sh` tools [\#74](https://github.com/confio/poe-contracts/pull/74) ([maurolacy](https://github.com/maurolacy))
- Release v0.6.0-beta1 [\#73](https://github.com/confio/poe-contracts/pull/73) ([maurolacy](https://github.com/maurolacy))
- Voting Contract: vote and close uses current status [\#60](https://github.com/confio/poe-contracts/pull/60) ([ueco-jb](https://github.com/ueco-jb))
- Update rust toolchain to v1.58.1 [\#58](https://github.com/confio/poe-contracts/pull/58) ([uint](https://github.com/uint))
- 0.6.0-alpha1 release [\#56](https://github.com/confio/poe-contracts/pull/56) ([uint](https://github.com/uint))
- Voting contract: save info about creator of proposal [\#54](https://github.com/confio/poe-contracts/pull/54) ([ueco-jb](https://github.com/ueco-jb))
- Fix `remove_hook` helper [\#53](https://github.com/confio/poe-contracts/pull/53) ([maurolacy](https://github.com/maurolacy))
- Validator set query pagination [\#51](https://github.com/confio/poe-contracts/pull/51) ([maurolacy](https://github.com/maurolacy))
- ValidatorVoting - make sure proposal migrate msg is not empty [\#48](https://github.com/confio/poe-contracts/pull/48) ([ueco-jb](https://github.com/ueco-jb))
- Fix tg4-engagement docs / comments [\#47](https://github.com/confio/poe-contracts/pull/47) ([maurolacy](https://github.com/maurolacy))
- valset: Add a feature to update `min_weight` and `max_validators` [\#45](https://github.com/confio/poe-contracts/pull/45) ([uint](https://github.com/uint))
- Update rust to v1.54.0 in CI [\#43](https://github.com/confio/poe-contracts/pull/43) ([maurolacy](https://github.com/maurolacy))
- Valset: Fix JailMsg inconsistencies [\#39](https://github.com/confio/poe-contracts/pull/39) ([ueco-jb](https://github.com/ueco-jb))
- Valset: better unjail error message when jail lock didn't expire [\#38](https://github.com/confio/poe-contracts/pull/38) ([ueco-jb](https://github.com/ueco-jb))
- Fix tag consolidation for matching CHANGELOG entries [\#32](https://github.com/confio/poe-contracts/pull/32) ([maurolacy](https://github.com/maurolacy))
- Open Text Proposals [\#27](https://github.com/confio/poe-contracts/pull/27) ([uint](https://github.com/uint))

## [v0.5.5](https://github.com/confio/poe-contracts/tree/v0.5.5) (2022-01-27)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.5.4...v0.5.5)

**Closed issues:**

- Fix wasm-build [\#41](https://github.com/confio/poe-contracts/issues/41)
- valset: better error message when jail lock not expired [\#34](https://github.com/confio/poe-contracts/issues/34)
- valset: limit active\_valset query and add pagination [\#33](https://github.com/confio/poe-contracts/issues/33)
- valset: update max\_validators [\#28](https://github.com/confio/poe-contracts/issues/28)
- valset: mark "active" validators in ValidatorInfo [\#23](https://github.com/confio/poe-contracts/issues/23)
- Fix JailMsg inconsistencies [\#20](https://github.com/confio/poe-contracts/issues/20)

## [v0.5.4](https://github.com/confio/poe-contracts/tree/v0.5.4) (2022-01-20)

[Full Changelog](https://github.com/confio/poe-contracts/compare/v0.5.3-2...v0.5.4)

**Merged pull requests:**

- Allow migrations [\#29](https://github.com/confio/poe-contracts/pull/29) ([ethanfrey](https://github.com/ethanfrey))

## [v0.5.3](https://github.com/confio/poe-contracts/tree/v0.5.3-2) (2022-01-18)

[Full Changelog](https://github.com/confio/poe-contracts/compare/7a91033173dbd32d835373b31ad1c1b7c7db4296...v0.5.3-2)

**Merged pull requests:**

- test utils moved from tgrade contracts [\#25](https://github.com/confio/poe-contracts/pull/25) ([hashedone](https://github.com/hashedone))
- bindings-test: added missing genesis constructor [\#24](https://github.com/confio/poe-contracts/pull/24) ([hashedone](https://github.com/hashedone))
- Add publish script and add license to Cargo.toml files [\#21](https://github.com/confio/poe-contracts/pull/21) ([ethanfrey](https://github.com/ethanfrey))

**Fixed bugs:**

- voting: Abstaining should be able to trigger early end [\#16](https://github.com/confio/poe-contracts/issues/16)
- Proposals cannot be executed based on quorum after voting period over [\#14](https://github.com/confio/poe-contracts/issues/14)
- community-pool proposals can be executed multiple times [\#12](https://github.com/confio/poe-contracts/issues/12)

**Closed issues:**

- Tag poe-contracts v0.5.3 [\#13](https://github.com/confio/poe-contracts/issues/13)
- Move over gov-reflect and vesting-contract to this repo [\#7](https://github.com/confio/poe-contracts/issues/7)
- Multitests for tgrade-voting-contract [\#6](https://github.com/confio/poe-contracts/issues/6)

**Merged pull requests:**

- 0.5.3 release [\#19](https://github.com/confio/poe-contracts/pull/19) ([maurolacy](https://github.com/maurolacy))
- Fix proposal status not updated [\#18](https://github.com/confio/poe-contracts/pull/18) ([maurolacy](https://github.com/maurolacy))
- Move over gov-reflect and vesting-account contract to this repo [\#11](https://github.com/confio/poe-contracts/pull/11) ([ueco-jb](https://github.com/ueco-jb))
- voting-contract: Move rules builder follow-up [\#10](https://github.com/confio/poe-contracts/pull/10) ([uint](https://github.com/uint))
- Move `RulesBuilder` into `voting-contract` [\#9](https://github.com/confio/poe-contracts/pull/9) ([uint](https://github.com/uint))
- voting-contract: tests [\#8](https://github.com/confio/poe-contracts/pull/8) ([uint](https://github.com/uint))
- tgrade-validator-voting and tgrade-community-pool contracts moved [\#5](https://github.com/confio/poe-contracts/pull/5) ([hashedone](https://github.com/hashedone))
- Moved PoE contracts: engagement, mixer, stake, valset [\#4](https://github.com/confio/poe-contracts/pull/4) ([hashedone](https://github.com/hashedone))
- Basic CI config [\#2](https://github.com/confio/poe-contracts/pull/2) ([hashedone](https://github.com/hashedone))
- readme, utility scripts [\#1](https://github.com/confio/poe-contracts/pull/1) ([hashedone](https://github.com/hashedone))


\* *This Changelog was automatically generated by [github_changelog_generator](https://github.com/github-changelog-generator/github-changelog-generator)*
