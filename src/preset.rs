use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct CMakePreset {
    version: u32,

    #[serde(rename(
        serialize = "cmakeMinimumRequired",
        deserialize = "cmakeMinimumRequired"
    ))]
    cmake_minimum_required: Option<CMakeMinimumRequired>,

    include: Option<Vec<String>>,

    // vendor
    #[serde(rename(serialize = "configurePresets", deserialize = "configurePresets"))]
    configure_presets: Option<Vec<ConfigurePreset>>,

    #[serde(rename(
        serialize = "buildPresets",
        deserialize = "buildPresets"
    ))]
    build_presets: Option<Vec<BuildPreset>>,

    #[serde(rename(
        serialize = "testPresets",
        deserialize = "testPresets"
    ))]
    test_presets: Option<Vec<TestPreset>>,

    #[serde(rename(
        serialize = "packagePresets",
        deserialize = "packagePresets"
    ))]
    package_presets: Option<Vec<PackagePreset>>,

    #[serde(rename(serialize = "workflowPresets", deserialize = "workflowPresets"))]
    workflow_presets: Option<Vec<WorkflowPreset>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CMakeMinimumRequired {
    major: u32,
    minor: u32,
    patch: u32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ConfigurePreset {
    name: String,
    hidden: Option<bool>,
    inherits: Option<Inherit>,
    // conditon: Option<>,
    // vendor: Option<>,
    #[serde(rename(serialize = "displayName", deserialize = "displayName"))]
    display_name: Option<String>,

    description: Option<String>,
    generator: Option<String>,

    #[serde(rename(serialize = "toolchainFile", deserialize = "toolchainFile"))]
    toolchain_file: Option<String>,

    #[serde(rename(serialize = "binaryDir", deserialize = "binaryDir"))]
    binary_dir: Option<String>,

    #[serde(rename(serialize = "installDir", deserialize = "installDir"))]
    install_dir: Option<String>,

    // cmake_executable,
    #[serde(rename(serialize = "cacheVariables", deserialize = "cacheVariables"))]
    cache_variables: Option<HashMap<String, String>>,
    // environment,
    // warnings,
    // errors,
    // debug,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BuildPreset {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestPreset {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InstallPreset {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PackagePreset {
    name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowPreset {
    name: String,

    #[serde(rename(serialize = "displayName", deserialize = "displayName"))]
    display_name: Option<String>,

    description: Option<String>,

    steps: Vec<WorkflowStep>,
}

// #[derive(Serialize, Deserialize, Debug)]
// pub struct Condition {
//     type: String,
// }

#[derive(Serialize, Deserialize, Debug)]
pub enum Inherit {
    Single(String),
    Multi(Vec<String>),
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ArchitectureToolset {
    Simple(String),
    Complex {
        value: Option<String>,
        strategy: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WorkflowStep {
    #[serde(rename(serialize = "type", deserialize = "type"))]
    type_name: String,
    name: String,
}
