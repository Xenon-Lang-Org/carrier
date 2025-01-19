use clap::Parser;

mod commands;
mod config;

use commands::Commands;

#[derive(Parser)]
#[command(
    name = "carrier",
    about = "A CLI tool for the Xenon language.",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init { name } => commands::handle_init(name)?,
        Commands::Build { source, output } => commands::handle_build(source, output)?,
        Commands::Run { files, entry } => commands::handle_run(files, entry)?,
        Commands::Vm { wasm_file, args } => commands::handle_vm(wasm_file, args)?,
        Commands::Config { key, value } => commands::handle_config(key, value)?,
    }

    Ok(())
}
