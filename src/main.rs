mod cmd;

use cmd::cli::Commands;
use cmd::prelude::*;

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
        Commands::Admin(admin_args) => cmd::admin::run(admin_args),
        Commands::Client(client_args) => cmd::client::run(client_args),
        Commands::Server(server_args) => cmd::server::run(server_args),
    }
}
