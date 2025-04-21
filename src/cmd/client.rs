use clap::Args;

const DEFAULT_PORT: u16 = 8100;

#[derive(Args)]
pub struct ClientArgs {
    // This is a positional arg
    pub server: String,

    // N.B. cargo add clap with env feature: cargo add clap --features derive,env
    #[arg(short, long, env = "CLIENT_PORT")]
    pub port: Option<u16>,
}

pub fn run(args: &ClientArgs) {
    let port = args.port.unwrap_or(DEFAULT_PORT);
    let server = &args.server;
    println!("TODO Client should connect to {server} on port {port}")
}
