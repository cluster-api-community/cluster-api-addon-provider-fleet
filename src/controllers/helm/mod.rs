use std::io;

use thiserror::Error;

pub type FleetInstallResult<T> = std::result::Result<T, FleetInstallError>;

#[derive(Error, Debug)]
pub enum FleetInstallError {
    #[error("Fleet install error: {0}")]
    FleetInstall(#[from] helm_r2g::install::InstallError),

    #[error("Fleet upgrade error: {0}")]
    FleetUpgrade(#[from] helm_r2g::upgrade::UpgradeError),

    #[error("Deserialize install error: {0}")]
    DeserializeInstallError(#[from] serde_json::Error),
}

pub type FleetPatchResult<T> = std::result::Result<T, FleetPatchError>;

#[derive(Error, Debug)]
pub enum FleetPatchError {
    #[error("Fleet chart patch error: {0}")]
    FleetPatch(#[from] io::Error),
}

pub type FleetCRDInstallResult<T> = std::result::Result<T, FleetCRDInstallError>;

#[derive(Error, Debug)]
pub enum FleetCRDInstallError {
    #[error("CRD install error: {0}")]
    CRDInstall(#[from] helm_r2g::install::InstallError),

    #[error("CRD upgrade error: {0}")]
    CRDUpgrade(#[from] helm_r2g::upgrade::UpgradeError),

    #[error("Deserialize install response error: {0}")]
    DeserializeInstallError(#[from] serde_json::Error),
}

pub type RepoAddResult<T> = std::result::Result<T, RepoAddError>;

#[derive(Error, Debug)]
pub enum RepoAddError {
    #[error("Fleet repo add error: {0}")]
    RepoAdd(#[from] helm_r2g::repo_add::RepoAddError),
}

pub type RepoSearchResult<T> = std::result::Result<T, RepoSearchError>;

#[derive(Error, Debug)]
pub enum RepoSearchError {
    #[error("Fleet repo search error: {0}")]
    RepoSearch(#[from] helm_r2g::repo_search::RepoSearchError),

    #[error("Deserialize search error: {0}")]
    DeserializeInfoError(#[from] serde_json::Error),
}

pub type MetadataGetResult<T> = std::result::Result<T, MetadataGetError>;

#[derive(Error, Debug)]
pub enum MetadataGetError {
    #[error("Metadata get error: {0}")]
    MetadataGet(#[from] helm_r2g::list::ListError),

    #[error("Deserialize info error: {0}")]
    DeserializeInfoError(#[from] serde_json::Error),
}

pub mod install;
