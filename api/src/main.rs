mod web;

use anyhow::Result;
use env_logger::Env;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, Clone, StructOpt)]
pub struct Options {
    /// Port to listen on.
    #[structopt(short = "p", long = "server-port", default_value = "4000")]
    pub server_port: u16,

    // The default 127.0.0.1 is for development, so you don't expose the port on your
    // local network.
    /// Address to bind to.  This can be an IPv4 (e.g., "127.0.0.1") or IPv6 address
    /// (e.g., "::1"). To listen on all addresses, use "--bind-addr 0.0.0.0".
    #[structopt(short = "b", long = "bind", default_value = "127.0.0.1")]
    pub bind_addr: String,

    /// Directory to serve assets from. If no directory is specified, the server only serves the
    /// API.
    #[structopt(short = "d", long = "assets-dir")]
    pub assets_dir: Option<PathBuf>,
}

fn main() -> Result<()> {
    let opts = Options::from_args();
    env_logger::from_env(Env::default().default_filter_or("info")).init();
    web::serve(opts)
}
