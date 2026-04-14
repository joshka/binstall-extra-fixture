use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use clap::Parser;

#[derive(Debug, Parser)]
#[command(about = "Build a release archive with man page and shell completions")]
struct Args {
    /// Rust target triple used to name the output archive.
    #[arg(long)]
    target: String,

    /// Version string without the leading v.
    #[arg(long)]
    version: String,

    /// Path to the already-built release binary for the selected target.
    #[arg(long)]
    binary: PathBuf,

    /// Output directory for generated artifacts and final archives.
    #[arg(long, default_value = "dist")]
    out_dir: PathBuf,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let crate_root = PathBuf::from(env::var("CARGO_MANIFEST_DIR")?);
    let stage_dir = args.out_dir.join(format!(
        "binstall-extra-fixture-{}-v{}",
        args.target, args.version
    ));

    if stage_dir.exists() {
        fs::remove_dir_all(&stage_dir)?;
    }

    fs::create_dir_all(stage_dir.join("man/man1"))?;
    fs::create_dir_all(stage_dir.join("completions/bash"))?;
    fs::create_dir_all(stage_dir.join("completions/fish"))?;
    fs::create_dir_all(stage_dir.join("completions/zsh"))?;

    fs::copy(&args.binary, stage_dir.join("binstall-extra-fixture"))?;

    let cmd = binstall_extra_fixture::command();
    let man = clap_mangen::Man::new(cmd.clone());
    let mut man_buffer = Vec::new();
    man.render(&mut man_buffer)?;
    fs::write(
        stage_dir.join("man/man1/binstall-extra-fixture.1"),
        man_buffer,
    )?;

    write_completion(
        clap_complete::Shell::Bash,
        "binstall-extra-fixture",
        &stage_dir.join("completions/bash/binstall-extra-fixture"),
    )?;
    write_completion(
        clap_complete::Shell::Fish,
        "binstall-extra-fixture",
        &stage_dir.join("completions/fish/binstall-extra-fixture.fish"),
    )?;
    write_completion(
        clap_complete::Shell::Zsh,
        "binstall-extra-fixture",
        &stage_dir.join("completions/zsh/_binstall-extra-fixture"),
    )?;

    let archive = args.out_dir.join(format!(
        "binstall-extra-fixture-{}-v{}.tar.gz",
        args.target, args.version
    ));

    if archive.exists() {
        fs::remove_file(&archive)?;
    }

    create_archive(&crate_root, &args.out_dir, &archive, &stage_dir)?;
    println!("{}", archive.display());
    Ok(())
}

fn write_completion(
    shell: clap_complete::Shell,
    bin_name: &str,
    out_file: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = binstall_extra_fixture::command();
    let mut output = Vec::new();
    clap_complete::generate(shell, &mut cmd, bin_name, &mut output);
    fs::write(out_file, output)?;
    Ok(())
}

fn create_archive(
    crate_root: &Path,
    out_dir: &Path,
    archive: &Path,
    stage_dir: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let status = Command::new("tar")
        .current_dir(crate_root)
        .arg("-czf")
        .arg(archive)
        .arg("-C")
        .arg(out_dir)
        .arg(
            stage_dir
                .file_name()
                .ok_or("missing stage directory name")?,
        )
        .status()?;

    if status.success() {
        Ok(())
    } else {
        Err("tar command failed".into())
    }
}
