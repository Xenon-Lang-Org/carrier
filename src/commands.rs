use clap::{Parser, Subcommand};
use std::path::PathBuf;
use anyhow::Result;

use crate::config::{load_config, save_config, XnConfig};

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new XN project
    Init {
        /// Name of the project
        name: String,
    },
    /// Build (compile) XN sources to WASM
    Build {
        /// Path to the source file to compile
        #[arg(short, long, default_value = "main.xn")]
        source: PathBuf,
        /// Output WASM file
        #[arg(short, long, default_value = "output.wasm")]
        output: PathBuf,
    },
    /// Run (interpret) one or more XN files
    Run {
        /// List of XN files to run with the interpreter
        #[arg()]
        files: Vec<PathBuf>,
        /// Entrypoint file if needed
        #[arg(short, long)]
        entry: Option<PathBuf>,
    },
    /// Execute a compiled WASM file on the XN WASM VM
    Vm {
        /// The `.wasm` file to run
        wasm_file: PathBuf,
        /// Arguments to pass to the VM
        #[arg()]
        args: Vec<String>,
    },
    /// Read or update config file key-value pairs
    Config {
        /// The config key (e.g., "compiler_path", "vm_path")
        key: Option<String>,
        /// The value to set. Omit to get the current value of `key`.
        value: Option<String>,
    },
}

pub fn handle_init(name: String) -> Result<()> {
    // Create a new project directory
    std::fs::create_dir_all(&name)?;
    
    // Create a default config
    let default_config = XnConfig {
        compiler_path: "xcc".to_string(),
        interpreter_path: "xin".to_string(),
        vm_path: "xvm".to_string(),
        project_name: name.clone(),
        ..Default::default()
    };
    
    // Write the config to a file named "xn.toml" inside the new project
    let config_path = format!("{}/xn.toml", name);
    save_config(&default_config, &config_path)?;

    println!("Initialized a new XN project in `{}`", name);
    Ok(())
}

pub fn handle_build(source: PathBuf, output: PathBuf) -> Result<()> {
    let config = load_config("xn.toml")?;
    
    // Example: calling xcc via std::process::Command
    std::process::Command::new(&config.compiler_path)
        .arg(source)
        .arg("-o")
        .arg(output)
        .spawn()?
        .wait()?;

    println!("Build finished");
    Ok(())
}

pub fn handle_run(files: Vec<PathBuf>, entry: Option<PathBuf>) -> Result<()> {
    let config = load_config("xn.toml")?;
    
    let mut cmd = std::process::Command::new(&config.interpreter_path);
    for f in files {
        cmd.arg(f);
    }
    if let Some(entry_file) = entry {
        cmd.arg("-e").arg(entry_file);
    }
    
    cmd.spawn()?.wait()?;
    Ok(())
}

pub fn handle_vm(wasm_file: PathBuf, args: Vec<String>) -> Result<()> {
    let config = load_config("xn.toml")?;
    
    let mut cmd = std::process::Command::new(&config.vm_path);
    cmd.arg(wasm_file);
    for a in args {
        cmd.arg(a);
    }

    cmd.spawn()?.wait()?;
    Ok(())
}

pub fn handle_config(key: Option<String>, value: Option<String>) -> Result<()> {
    let mut config = load_config("xn.toml")?;
    
    match (key, value) {
        (Some(k), Some(v)) => {
            // set the key to value
            match k.as_str() {
                "compiler_path" => config.compiler_path = v.clone(),
                "interpreter_path" => config.interpreter_path = v.clone(),
                "vm_path" => config.vm_path = v.clone(),
                "project_name" => config.project_name = v.clone(),
                _ => println!("Unknown config key: {}", k),
            }
            save_config(&config, "xn.toml")?;
            println!("Updated config key `{}` to `{}`", k, v);
        }
        (Some(k), None) => {
            // read config key
            let val = match k.as_str() {
                "compiler_path" => &config.compiler_path,
                "interpreter_path" => &config.interpreter_path,
                "vm_path" => &config.vm_path,
                "project_name" => &config.project_name,
                _ => {
                    println!("Unknown config key: {}", k);
                    return Ok(());
                }
            };
            println!("{} = {}", k, val);
        }
        (None, _) => {
            // list all config
            println!("Current config:\n{:#?}", config);
        }
    }

    Ok(())
}
