use abstract_core::{
    manager::ModuleInstallConfig,
    objects::module::{ModuleInfo, ModuleVersion},
};
use cosmwasm_std::to_json_binary;
use cw_orch::prelude::{CwOrchError::StdErr, *};
use semver::Version;
use serde::Serialize;

use crate::Abstract;

/// Trait to access module information tied directly to the type.
pub trait RegisteredModule {
    /// The init message for the module.
    type InitMsg: Serialize;
    /// The id of the module.
    fn module_id<'a>() -> &'a str;
    /// The version of the module.
    fn module_version<'a>() -> &'a str;
}

/// Trait to access module dependency information tied directly to the type.
pub trait DependencyCreation {
    /// Type that exposes the dependencies's configurations if that's required.
    type DependenciesConfig;

    /// Function that returns the [`ModuleInstallConfig`] for each dependent module.
    #[allow(unused_variables)]
    fn dependency_install_configs(
        configuration: Self::DependenciesConfig,
    ) -> Result<Vec<ModuleInstallConfig>, crate::AbstractInterfaceError> {
        Ok(vec![])
    }
}

/// Trait to make it easier to construct [`ModuleInfo`] and [`ModuleInstallConfig`] for a
/// [`RegisteredModule`].
pub trait InstallConfig: RegisteredModule {
    /// Constructs the [`ModuleInfo`] by using information from [`RegisteredModule`].
    fn module_info() -> Result<ModuleInfo, crate::AbstractInterfaceError> {
        ModuleInfo::from_id(Self::module_id(), Self::module_version().into()).map_err(Into::into)
    }

    /// Constructs the [`ModuleInstallConfig`] for an App Interface
    fn install_config(
        init_msg: &Self::InitMsg,
    ) -> Result<ModuleInstallConfig, crate::AbstractInterfaceError> {
        Ok(ModuleInstallConfig::new(
            Self::module_info()?,
            Some(to_json_binary(init_msg)?),
        ))
    }
}

// Blanket implemention.
impl<T> InstallConfig for T where T: RegisteredModule {}

/// Strategy for deploying
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeployStrategy {
    /// Error if already present
    Error,
    /// Ignore if already present
    Try,
    /// Force deployment
    Force,
}

/// Trait for deploying Adapters
pub trait AdapterDeployer<Chain: CwEnv, CustomInitMsg: Serialize>: ContractInstance<Chain>
    + CwOrchInstantiate<Chain, InstantiateMsg = abstract_core::adapter::InstantiateMsg<CustomInitMsg>>
    + Uploadable
    + Sized
{
    /// Deploys the adapter. If the adapter is already deployed, it will return an error.
    /// Use `maybe_deploy` if you want to deploy the adapter only if it is not already deployed.
    fn deploy(
        &self,
        version: Version,
        custom_init_msg: CustomInitMsg,
        strategy: DeployStrategy,
    ) -> Result<(), crate::AbstractInterfaceError> {
        // retrieve the deployment
        let abstr = Abstract::load_from(self.get_chain().to_owned())?;

        // check for existing version, if not force strategy
        let version_check = || {
            abstr
                .version_control
                .get_adapter_addr(&self.id(), ModuleVersion::from(version.to_string()))
        };

        match strategy {
            DeployStrategy::Error => {
                if version_check().is_ok() {
                    return Err(StdErr(format!(
                        "Adapter {} already exists with version {}",
                        self.id(),
                        version
                    ))
                    .into());
                }
            }
            DeployStrategy::Try => {
                if version_check().is_ok() {
                    return Ok(());
                }
            }
            DeployStrategy::Force => {}
        }

        if self.upload_if_needed()?.is_some() {
            let init_msg = abstract_core::adapter::InstantiateMsg {
                module: custom_init_msg,
                base: abstract_core::adapter::BaseInstantiateMsg {
                    ans_host_address: abstr.ans_host.address()?.into(),
                    version_control_address: abstr.version_control.address()?.into(),
                },
            };
            self.instantiate(&init_msg, None, None)?;

            abstr
                .version_control
                .register_adapters(vec![(self.as_instance(), version.to_string())])?;
        }
        Ok(())
    }
}

/// Trait for deploying APPs
pub trait AppDeployer<Chain: CwEnv>: Sized + Uploadable + ContractInstance<Chain> {
    /// Deploys the app. If the app is already deployed, it will return an error.
    /// Use `maybe_deploy` if you want to deploy the app only if it is not already deployed.
    fn deploy(
        &self,
        version: Version,
        strategy: DeployStrategy,
    ) -> Result<(), crate::AbstractInterfaceError> {
        // retrieve the deployment
        let abstr = Abstract::<Chain>::load_from(self.get_chain().to_owned())?;

        // check for existing version
        let version_check = || {
            abstr
                .version_control
                .get_app_code(&self.id(), ModuleVersion::from(version.to_string()))
        };

        match strategy {
            DeployStrategy::Error => {
                if version_check().is_ok() {
                    return Err(StdErr(format!(
                        "App {} already exists with version {}",
                        self.id(),
                        version
                    ))
                    .into());
                }
            }
            DeployStrategy::Try => {
                if version_check().is_ok() {
                    return Ok(());
                }
            }
            _ => {}
        }

        if self.upload_if_needed()?.is_some() {
            abstr
                .version_control
                .register_apps(vec![(self.as_instance(), version.to_string())])?;
        }

        Ok(())
    }
}
