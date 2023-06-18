use snowball::snowman::*;
use snowball::commands::*;
use clap::{Parser};

fn main() {
    let cli = CLI::parse();
    
    match &cli.command {
        Commands::Test(url) => {
            
        }
        Commands::Run(_) => {

        }
    }
    // let _output = cmd_win();

}
