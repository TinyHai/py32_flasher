use crate::cli::Cli;

pub mod cli;
pub mod flash;

fn main() -> anyhow::Result<()> {
    Cli::run()
}
