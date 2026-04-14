use std::path::PathBuf;

use clap::{ArgAction, Args, CommandFactory, Parser, Subcommand, ValueHint};

#[derive(Debug, Parser)]
#[command(
    author,
    version,
    about = "Exercise cargo-binstall extra-file installation paths",
    long_about = "A small CLI fixture used to publish release archives that include a binary, a \
generated man page, and generated shell completions in cargo-binstall's default extra-file \
layout."
)]
pub struct Cli {
    /// Increase output verbosity.
    #[arg(short, long, action = ArgAction::Count, global = true)]
    pub verbose: u8,

    /// Disable ANSI colors in terminal output.
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Optional configuration file to inspect.
    #[arg(long, value_name = "PATH", global = true, value_hint = ValueHint::FilePath)]
    pub config: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Inspect a fixture name and describe how it would be processed.
    Inspect(InspectArgs),
}

#[derive(Debug, Args)]
pub struct InspectArgs {
    /// Fixture name to inspect.
    pub name: String,

    /// Emit the result as a compact machine-readable line.
    #[arg(long)]
    pub json: bool,

    /// Include a canned extra-file summary in the output.
    #[arg(long)]
    pub include_extras: bool,
}

pub fn command() -> clap::Command {
    Cli::command()
}

pub fn run(cli: Cli) {
    let Cli {
        verbose,
        no_color,
        config,
        command,
    } = cli;
    let base_cli = Cli {
        verbose,
        no_color,
        config,
        command: None,
    };

    match command {
        Some(Commands::Inspect(args)) => run_inspect(base_cli, args),
        None => print_overview(base_cli),
    }
}

fn print_overview(cli: Cli) {
    println!("fixture: binstall-extra-fixture");
    println!("verbose-level: {}", cli.verbose);
    println!("color: {}", if cli.no_color { "disabled" } else { "auto" });
    println!(
        "config: {}",
        cli.config
            .as_deref()
            .map(|path| path.display().to_string())
            .unwrap_or_else(|| "none".to_owned())
    );
}

fn run_inspect(cli: Cli, args: InspectArgs) {
    let config = cli
        .config
        .as_deref()
        .map(|path| path.display().to_string())
        .unwrap_or_else(|| "none".to_owned());

    if args.json {
        println!(
            "{{\"name\":\"{}\",\"verbose\":{},\"color\":\"{}\",\"config\":\"{}\",\"include_extras\":{}}}",
            args.name,
            cli.verbose,
            if cli.no_color { "disabled" } else { "auto" },
            config,
            args.include_extras
        );
        return;
    }

    println!("fixture-name: {}", args.name);
    println!("verbose-level: {}", cli.verbose);
    println!("color: {}", if cli.no_color { "disabled" } else { "auto" });
    println!("config: {}", config);

    if args.include_extras {
        println!("extras:");
        println!("- man/man1/binstall-extra-fixture.1");
        println!("- completions/bash/binstall-extra-fixture");
        println!("- completions/fish/binstall-extra-fixture.fish");
        println!("- completions/zsh/_binstall-extra-fixture");
    }
}
