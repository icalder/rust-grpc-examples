mod cmd;

use cmd::prelude::*;
use cmd::{admin::AdminCommands, cli::Commands};

const DEFAULT_PORT: u16 = 8100;

/*
TODO:
clap commands and subcommands
grpc client and server
 - rpc to get time
 - stream to provide time updates
 */

fn main() {
    let cli = cmd::cli::Cli::parse();

    println!("debug level = {}", cli.debug);

    match &cli.command {
        Commands::Admin(admin_args) => match &admin_args.command {
            AdminCommands::Setup { replicas } => {
                println!("Admin Setup with {} replicas", replicas)
            }
            AdminCommands::Teardown => println!("Admin Teardown"),
        },
        Commands::Client { server, port } => {
            let port = port.unwrap_or(DEFAULT_PORT);
            println!("TODO Client should connect to {server} on port {port}")
        }
        Commands::Server { port } => {
            println!("TODO Server should listen on port {port}")
        }
    }
}
