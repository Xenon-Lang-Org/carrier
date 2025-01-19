use anyhow::Result;
use clap::Subcommand;
use std::path::{Path, PathBuf};
use walkdir::WalkDir; // For recursive directory walking

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
        /// Optional single source file (otherwise we gather from src/)
        #[arg(short, long)]
        source: Option<PathBuf>,
        /// Output WASM file
        #[arg(short, long, default_value = "out/output.wasm")]
        output: PathBuf,
    },
    /// Run (interpret) one or more XN files
    Run {
        /// If empty, we'll gather .xn files from src/
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
    std::fs::create_dir_all(&name)?;

    let src_dir = format!("{}/src", &name);
    std::fs::create_dir_all(&src_dir)?;

    let main_xn_path = format!("{}/main.xn", src_dir);
    let gcd_code = r#"// Greatest Common Divisor
fn gcd(a: mut i32, b: mut i32) -> i32
{
    let t: mut i32 = 0;
    
    while (b != 0) {
        t = b;
        b = a % b;
        a = t;
    }
    return a;
}

fn main() -> i32 {
    let a: i32 = 270;
    let b: i32 = 192;
    return gcd(a, b);
}
"#;
    std::fs::write(&main_xn_path, gcd_code)?;

    let default_config = XnConfig {
        compiler_path: "xcc".to_string(),
        interpreter_path: "xin".to_string(),
        vm_path: "xrun".to_string(),
        project_name: name.clone(),
        ..Default::default()
    };

    let config_path = format!("{}/xn.toml", &name);
    save_config(&default_config, &config_path)?;

    println!("Initialized a new XN project in `{}`", name);
    Ok(())
}

pub fn handle_build(source: Option<PathBuf>, output: PathBuf) -> Result<()> {
    let config = load_config("xn.toml")?;

    std::fs::create_dir_all("out")?;

    // if not specify a `--source`, concatenate all .xn from src/
    let source_to_compile = if let Some(src_path) = source {
        src_path
    } else {
        let concatenated_file = concatenate_xn_files("src")?;
        concatenated_file
    };

    std::process::Command::new(&config.compiler_path)
        .arg(source_to_compile)
        .arg("-o")
        .arg(&output)
        .spawn()?
        .wait()?;

    println!("Build finished -> {}", output.display());
    Ok(())
}

pub fn handle_run(mut files: Vec<PathBuf>, entry: Option<PathBuf>) -> Result<()> {
    let config = load_config("xn.toml")?;

    // If the user does not pass any files, gather everything under src/
    if files.is_empty() {
        files = gather_xn_files("src");
    }

    if files.is_empty() {
        println!("No .xn files found to run.");
        return Ok(());
    }

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
            println!("Current config:\n{:#?}", config);
        }
    }

    Ok(())
}

// recursively
fn gather_xn_files<P: AsRef<Path>>(dir: P) -> Vec<PathBuf> {
    let mut collected = Vec::new();
    for entry in WalkDir::new(dir) {
        if let Ok(e) = entry {
            if e.file_type().is_file() {
                if let Some(ext) = e.path().extension() {
                    if ext == "xn" {
                        collected.push(e.path().to_path_buf());
                    }
                }
            }
        }
    }
    collected
}

fn concatenate_xn_files<P: AsRef<Path>>(dir: P) -> Result<PathBuf> {
    let xn_files = gather_xn_files(dir);
    if xn_files.is_empty() {
        anyhow::bail!("No .xn files found in `src/` for build.");
    }

    std::fs::create_dir_all("out")?;

    let merged_path = PathBuf::from("out/output.xn");

    let mut merged_contents = String::new();
    for file in &xn_files {
        let file_contents = std::fs::read_to_string(file)?;
        merged_contents.push_str("// Start of file: ");
        merged_contents.push_str(&file.to_string_lossy());
        merged_contents.push('\n');
        merged_contents.push_str(&file_contents);
        merged_contents.push('\n');
    }

    std::fs::write(&merged_path, merged_contents)?;
    Ok(merged_path)
}
