use github_team_visualizer::{chunk::teams::print_team_tree, CmdArgs};

#[tokio::main]
async fn main() {
  let args = CmdArgs::opts();

  if args.debug {
    eprintln!("Debug mode activated. Args: {:#?}", args);
  }

  print_team_tree(&args).await;
}
