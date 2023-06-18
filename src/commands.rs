use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct CLI {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Launches given SSH profile
    Run (RunArgs),
    // Runs a test ping to given URL
    Test (TestArgs)
}

// -- Command Arguments --
#[derive(Args)]
pub struct RunArgs {
    /// Profile to launch into
    profile: Option<String>
}

#[derive(Args)]
pub struct TestArgs {
    /// URL to test ping
    url: Option<String>
}