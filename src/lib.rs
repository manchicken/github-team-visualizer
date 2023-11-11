pub mod chunk;

/// Struct for command line arguments
use clap::Parser;
#[derive(Parser, Debug)]
#[command(name = "github-team-visualizer")]
pub struct CmdArgs {
  /// Activate debug mode
  #[arg(short='d', long)]
  pub debug: bool,

  /// Set the organization
  #[arg(short='o', long, required=true)]
  pub organization: String,
}

impl CmdArgs {
  pub fn opts() -> Self {
    CmdArgs::parse()
  }
}