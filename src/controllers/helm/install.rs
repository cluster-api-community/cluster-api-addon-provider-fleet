use helm_r2g::{
    install::install,
    list::{List, list},
    repo_add::{RepoAdd, repo_add},
    repo_search::{RepoSearch, repo_search},
    upgrade::{Upgrade, upgrade},
};
use serde::Deserialize;
use serde_json::json;
use tracing::debug;

use crate::{
    api::fleet_addon_config::{FeatureGates, Install},
    controllers::helm::RepoAddResult,
};

use super::{FleetCRDInstallResult, FleetInstallResult, MetadataGetResult, RepoSearchResult};

#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Clone)]
pub struct FleetChart {
    pub repo: String,
    pub version: Option<Install>,
    pub namespace: String,

    pub wait: bool,
    pub update_dependency: bool,
    pub create_namespace: bool,

    pub bootstrap_local_cluster: bool,

    pub feature_gates: FeatureGates,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ChartInfo {
    pub name: String,
    pub namespace: String,
    #[serde(default)]
    pub chart: Chart,
    pub info: Info,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Chart {
    #[serde(default)]
    pub metadata: Metadata,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    #[serde(default)]
    pub app_version: String,
    pub name: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub status: String,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "PascalCase")]
pub struct ChartSearch {
    pub name: String,
    #[serde(default)]
    pub chart: EmbeddedChart,
}

#[derive(Deserialize, Debug, Default, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EmbeddedChart {
    #[serde(flatten)]
    pub metadata: Metadata,
}

impl FleetChart {
    /// Adds the fleet helm repository.
    ///
    /// # Errors
    ///
    /// This function will return an error if the helm command fails to spawn.
    pub async fn add_repo(&self) -> RepoAddResult<()> {
        let req = RepoAdd {
            name: "fleet".to_string(),
            url: self.repo.clone(),
            ..Default::default()
        };

        debug!("Adding fleet helm repository");

        Ok(repo_add(req).await?)
    }

    /// Searches the fleet helm repository for charts.
    ///
    /// # Errors
    ///
    /// This function will return an error if the helm command fails to spawn or the output cannot be parsed.
    pub async fn search_repo(&self) -> RepoSearchResult<Vec<ChartSearch>> {
        let req = RepoSearch {
            terms: vec!["fleet".to_string()],
            ..Default::default()
        };

        debug!("Searching fleet helm repository");

        let data = repo_search(req).await?;

        Ok(serde_json::from_str(&data)?)
    }

    /// Gets metadata for a specific chart.
    ///
    /// # Errors
    ///
    /// This function will return an error if the helm command fails to spawn or the output cannot be parsed.
    pub async fn get_metadata(chart: &str) -> MetadataGetResult<Option<ChartInfo>> {
        let req = List {
            all: true,
            all_namespaces: true,
            ..Default::default()
        };

        debug!("Listing helm charts metadata");

        let data = list(req).await?;

        if data.is_empty() {
            return Ok(None);
        }

        let infos: Vec<ChartInfo> = serde_json::from_str(&data)?;

        Ok(infos.into_iter().find(|i| i.name == chart))
    }

    /// Installs the fleet chart.
    pub async fn install_fleet(&self) -> FleetInstallResult<ChartInfo> {
        let req = helm_r2g::install::Install {
            release_name: "fleet".to_string(),
            chart: "fleet/fleet".to_string(),
            create_namespace: self.create_namespace,
            wait: self.wait,
            ns: self.namespace.clone(),
            version: match self.version.clone().unwrap_or_default() {
                Install::FollowLatest(_) => String::new(),
                Install::Version(version) => version,
            },
            values: serde_json::to_vec(&json!({
                "bootstrap": {
                    "enabled": self.bootstrap_local_cluster.to_string(),
                },
                "extraEnv": [
                    {
                        "name": "EXPERIMENTAL_OCI_STORAGE",
                        "value": self.feature_gates.experimental_oci_storage.to_string()
                    },
                    {
                        "name": "EXPERIMENTAL_HELM_OPS",
                        "value": self.feature_gates.experimental_helm_ops.to_string()
                    }
                ]
            }))?,
            ..Default::default()
        };

        debug!("Installing fleet chart");

        let data = install(req).await?;

        Ok(serde_json::from_str(&data)?)
    }

    /// Upgrades the fleet chart.
    pub async fn upgrade_fleet(&self) -> FleetInstallResult<ChartInfo> {
        let req = Upgrade {
            release_name: "fleet".to_string(),
            chart: "fleet/fleet".to_string(),
            wait: self.wait,
            ns: self.namespace.clone(),
            reuse_values: true,
            version: match self.version.clone().unwrap_or_default() {
                Install::FollowLatest(_) => String::new(),
                Install::Version(version) => version,
            },
            values: serde_json::to_vec(&json!({
                "bootstrap": {
                    "enabled": self.bootstrap_local_cluster.to_string(),
                },
                "extraEnv": [
                    {
                        "name": "EXPERIMENTAL_OCI_STORAGE",
                        "value": self.feature_gates.experimental_oci_storage.to_string()
                    },
                    {
                        "name": "EXPERIMENTAL_HELM_OPS",
                        "value": self.feature_gates.experimental_helm_ops.to_string()
                    }
                ]
            }))?,
            ..Default::default()
        };

        debug!("Upgrading fleet chart");

        let data = upgrade(req).await?;

        Ok(serde_json::from_str(&data)?)
    }

    /// Installs the fleet-crd chart.
    pub async fn install_fleet_crds(&self) -> FleetCRDInstallResult<ChartInfo> {
        let req = helm_r2g::install::Install {
            release_name: "fleet-crd".to_string(),
            chart: "fleet/fleet-crd".to_string(),
            create_namespace: self.create_namespace,
            wait: self.wait,
            ns: self.namespace.clone(),
            timeout: vec![300],
            version: match self.version.clone().unwrap_or_default() {
                Install::FollowLatest(_) => String::new(),
                Install::Version(version) => version,
            },
            ..Default::default()
        };

        debug!("Installing fleet-crd chart");

        let data = install(req).await?;

        Ok(serde_json::from_str(&data)?)
    }

    /// Upgrades the fleet-crd chart.
    pub async fn upgrade_fleet_crds(&self) -> FleetCRDInstallResult<ChartInfo> {
        let req = Upgrade {
            release_name: "fleet-crd".to_string(),
            chart: "fleet/fleet-crd".to_string(),
            reuse_values: true,
            wait: self.wait,
            ns: self.namespace.clone(),
            version: match self.version.clone().unwrap_or_default() {
                Install::FollowLatest(_) => String::new(),
                Install::Version(version) => version,
            },
            ..Default::default()
        };

        debug!("Upgrading fleet-crd chart");

        let data = upgrade(req).await?;

        Ok(serde_json::from_str(&data)?)
    }
}
