use clap::{Args, Parser, Subcommand};
use serde_json::Result;

mod preset;

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add(Add),
    New { name: String },
    // Test { name: String },
}

#[derive(Debug, Args)]
#[command(args_conflicts_with_subcommands = true)]
pub struct Add {
    #[command(subcommand)]
    command: AddCommand,
}

#[derive(Debug, Subcommand)]
pub enum AddCommand {
    Configure { name: String },
    //     // Build {
    //     //     name: String,
    //     // },
    //     // Test {
    //     //     name: String,
    //     // },
    //     // Install {
    //     //     name: String,
    //     // },
}

fn main() {
    env_logger::init();

    let args = match Cli::try_parse() {
        Ok(args) => args,
        Err(e) => {
            let _ = e.print();
            std::process::exit(2);
        }
    };

    log::trace!("Parsed arguments: {:#?}", args);

    match args.command {
        // Commands::Test { name } => {
        //     log::info!("TEST: {:?}", name);
        //     parse_example().unwrap();
        // },
        Commands::New { name: _ } => {
            new_example().unwrap();
        }
        Commands::Add(add) => {
            let add_cmd = add.command;
            match add_cmd {
                AddCommand::Configure { name } => {
                    println!("Configuring {:?}", name);
                }
            }
        }
    }
}

// fn parse_example() -> Result<()> {
//     let data = r#"
//         {
//             "version": 3,
//             "cmakeMinimumRequired": {
//                 "major": 3,
//                 "minor": 23,
//                 "patch": 0
//             },
//             "configurePresets": [
//                 {
//                     "name": "ninja-multi-config",
//                     "displayName": "Ninja multi config",
//                     "description": "configure with ninja",
//                     "toolchainFile": "$env{VCPKG_ROOT}/scripts/buildsystems/vcpkg.cmake",
//                     "generator": "Ninja Multi-Config",
//                     "binaryDir": "${sourceDir}/build/${presetName}",
//                     "hidden": true,
//                     "cacheVariables": {
//                         "CMAKE_BUILD_TYPE": "Debug"
//                     }
//                 }
//             ],
//             "workflowPresets": [
//                 {
//                     "name": "default",
//                     "description": "Default workflow",
//                     "steps": [
//                         {
//                             "type": "configure",
//                             "name": "ninja-multi-config"
//                         }
//                     ]
//                 }
//             ]
//         }"#;
//
//     let p: preset::CMakePreset = serde_json::from_str(data)?;
//     println!("CMakePreset: {:?}", p);
//
//     let j = serde_json::to_string_pretty(&p)?;
//     println!("{}", j);
//
//     Ok(())
// }

fn new_example() -> Result<()> {
    let mut preset = preset::WorkflowPreset::new("default");
    preset.set_description("default workflow");
    preset.add_step("configure", "default");
    preset.add_step("build", "default");

    let j = serde_json::to_string_pretty(&preset)?;
    println!("{}", j);

    Ok(())
}
