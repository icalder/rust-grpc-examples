use std::path::PathBuf;

use clap::{Parser, Subcommand};

// Nested subcommands example
// see: https://github.com/clap-rs/clap/blob/3ef784b516b2c9fbf6adb1c3603261b085561be7/examples/git-derive.rs

#[derive(Parser)]
// NB: version from cargo.toml will be used by default
#[command(version, about="Tests Clap", long_about = None)]
pub struct Cli {
    /// Optional name to operate on
    pub name: Option<String>,

    /// Sets a custom config file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::Count)]
    pub debug: u8,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// admin subcommands
    Admin(super::admin::AdminArgs),
    /// run a client
    Client(super::client::ClientArgs),
    /// run a server
    Server(super::server::ServerArgs),
}
