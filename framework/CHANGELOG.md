# Abstract Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased] - yyyy-mm-dd

### Added

- `state.json` now included in binary in release mode, allowing using binaries on a different environment than it's been built.
- `module_instantiate2_address_raw` for `AbstractClient`, allowing to install a different version than the dependency version.

### Changed

- Renamed `account_id` to `expected_account_id` for `abstract_client::AccountBuilder` for clarity

### Removed

- unused `custom_swap` of `DexCommand`

### Fixed

## [0.21.0] - 2024-02-20

### Added
  
- Added a `.execute` method on the AuthZ API to execute `CosmosMsg` types on behalf of a granter.
- Add IBC helpers to account client.
- Abstract Client builder: register dexes on ANS
- `.sub_accounts` method on `Account` for getting Abstract Client Sub Accounts
- Publish adapter method of Abstract Client Publisher now returns Adapter object
- Added a `.account_from` method on the `AbstractClient` for retrieving `Account`s.
- Creating Sub Account from `AbstractClient` Account builder.
- Installing apps and adapters for `AbstractClient` Account builder
- Attaching funds to account creation on `AbstractClient` Account builder
- Added `unchecked_account_id` method on version control.
- Ability to provide expected local AccountId
- Reinstallation of the same version of an app is now disabled
- `.authorize_on_adapters` method on `Application` for authorizing application on adapters
- Added method to assign expected `.account_id` for Abstract Client Account builder
- `.next_local_account_id` for `AbstractClient` to query next local account sequence
- `.module_instantiate2_address` for `AbstractClient` to get predicted address

### Changed

- Updated UsageFee api to use `Address`, instead of `Api` + unchecked address
- Tests now use `MockBech32` due to use of instantiate2.

### Removed

### Fixed

- Added a validation on `account_id` method on version control.
- Creating sub-account from account factory is restricted. Use Create Sub Account method of the manager instead

## [0.20.0] - 2024-01-24

### Added

- `AppDeployer` and `AdapterDeployer` now take a `DeployStrategy` field.
- `Astrovault` integrated into dex and cw-staking adapters
- `AuthZ` API added
- Interchain Abstract Accounts can now be created!
- Added snapshot tests
- Method `query_account_owner()` for Apps Admin object
- Query `registered_dexes` for `AbstractNameServiceClient`
- Query `top_level_owner` for manager and apps(as base query)
- Support of `ConcentratedLiquidity` pool type for swaps. Stake/unstake currently not supported
- Account namespace is unclaimed after `Renounce`
- Resolve trait for `cw-orch` `AnsHost` interface

### Changed

- `is_module_installed` moved from `Manager` to `Account`.
- `account_id()` method of `AccountRegistry` is now exposed.
- Allow module-id to be passed in as a valid authorized address when allowing new addresses on adapter contracts.
- `BaseInstantiateMsg` is now removed from install app API, now only `ModuleMsg` should be provided.
- `Modules`, `Manager` and `Proxy` are now instantiated via instantiate2 message.
- `FeeGrant` API updated.
- Bump `cw-orch` to `v0.18`.
- Top level account owner now has admin privileges on the apps and adapters
- Multiple `AbstractAccount`s now don't overlap
- Top level account owner can now claim pending sub-accounts directly
- `Clearable` helper type was added to the messages where clearing optional state could be useful
- Only incremental version migration of modules allowed (0.10 -> 0.11 is allowed but 0.10 -> 0.12 not because it skips 0.11)
- Module `tag_response` and `custom_tag_response` no longer require `Response` as an argument as well as renamed to `response` and `custom_response` respectively.
- Having sub accounts will prevent you from `Renounce`
- Version Control `Namespace` query now doesn't return an error when namespace is unclaimed
- `NamespaceResponse` type updated to be able to represent claimed and unclaimed namespace

### Removed

- `DepositMsgs` removed (now `deposit()` returns `Vec<CosmosMsg>`)
- Abstract removed from the fields where it's redundant
- InstantiateMsg is now removed from the install_adapter API
- Removed `wasm_smart_query` helper, since it's accessible from `Querier` object
- Removed Adapter base `Remove` action

### Fixed

- Namespace registration fee fixed
- Version Control smart query now returns Version Control config instead of factory address
- Sub accounts now unregister themselves on owning manager if renounced

## [0.19.0] - 2023-09-26

### Added

- Install modules on account or Sub-account creation.
- Manager stores his sub-accounts and sub-accounts can register or unregister in case of ownership change.
- Query on module factory to see how much funds needs to be attached for installing modules.
- Version control on instantiation to the Apps alongside with registry traits.
- Instantiation funds added to module configuration, allowing modules to perform external setup calls.
- An `adapter_msg_types` similar to `app_msg_types`. This can be used to easily define the top-level entrypoint messages.

### Changed

- Updated fetch_data arguments of CwStakingCommand
- StakingInfoResponse now returns staking target(which is either contract address or pool id) instead of always staking contract address.
- Owner of the sub-accounts now Proxy, allowing modules to interact with sub-accounts.
- Install modules replaced install module method on module factory to reduce gas consumption for multi-install cases.
- Modified the account id structure. Each account is now identified with a unique ID and a trace. This is a requirement for Abstract IBC.
- Register Module(and Add Module) will now accept list of items, which reduces gas for multi-module install
- Removed the `CustomSwap` option on the dex adapter.
- Stake methods on cw-staking adapter now accept list, allowing users to do multi-stake/unstake/etc.
- Added must_use attribute on abstract sdk methods
- Renamed `abstract-(dex/staking)-adapter-traits` to `abstract-(dex/staking)-standard`

### Fixed

- Partially fixed cw-staking for Osmosis.
- Manager governance now changes only after new "owner" claimed ownership.
- Fixed and separated cw-staking and dex adapters for kujira.
- `ExecOnModule` calls now forward any provided funds to the module that is called.
- Manager queries of standalone module versions will now return version of the contract from the Version Control storage instead of error

## [0.17.2] - 2023-07-27

### Added
- Neutron + Archway to registry

### Changed

### Fixed

## [0.17.1] - 2023-07-26

### Added

- Ability to set admin to native contracts during instantiation
- Query handler for module data
- Added neutron

### Changed

- Address of App/Adapter returned and set by default.

### Fixed

## [0.17.0] - 2023-07-05

### Added

- Ability to add module metadata.
- Ability to set an install fee for modules.
- Account interaction helpers

### Changed

- Removed the ability to claim multiple namespaces.
- It is now possible to replace a module code-id/address on testnets.

### Fixed

- Adapter execution from the manager with a provided proxy address is now allowed.

## [0.7.0] - 2023-02-15

### Added

### Changed

- Errors now need to implement `From<AbstractError>` and `From<AbstractSdkError>`

### Fixed

## [0.7.0] - 2023-02-01

### Added

### Changed

- Version Control `Modules` / `ModuleList`

### Fixed

## [0.5.2] - 2023-01-10

### Added

### Changed

### Fixed

- Fixed abstract-interface publishing

## [0.5.0] - 2022-01-08

### Added

### Changed

### Fixed

- Fixed wasming with `write_api` error in the `abstract-adapter` and `abstract-app`

## [0.5.0] - 2022-01-08

### Added

#### Module Factory

- unit testing

#### Ans Host

- `Config` query

#### Abstract SDK

- Better querying of app and adapter directly vs message construction

### Changed

- `PoolId` is now renamed to `PoolAddress` to avoid confusion with the Abstract Pool Id (and because it can be resolved
  to an address / id)

### Removed

- `construct_staking_entry` from `ContractEntry`, which had previously violated the SRP.

### Fixed
