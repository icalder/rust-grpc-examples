use clap::Args;

#[derive(Args)]
pub struct ServerArgs {
    #[arg(short, long, default_value_t = 8100)]
    pub port: u16,
}

pub fn run(args: &ServerArgs) {
    println!("TODO Server should listen on port {}", args.port)
}
