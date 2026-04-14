use clap::Parser;

fn main() {
    let cli = binstall_extra_fixture::Cli::parse();
    binstall_extra_fixture::run(cli);
}
