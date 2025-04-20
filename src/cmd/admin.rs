use clap::{Args, Subcommand};

#[derive(Args)]
#[command()]
pub struct AdminArgs {
    #[command(subcommand)]
    pub command: AdminCommands,
}

#[derive(Subcommand)]
pub enum AdminCommands {
    Setup {
        #[arg(short, long, default_value_t = 1)]
        replicas: u8,
    },
    Teardown,
}
