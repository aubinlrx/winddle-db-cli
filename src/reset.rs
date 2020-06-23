use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    #[structopt(short="d")]
    download: Option<PathBuf>,
    environment: Option<String>
}

pub fn call(opts: Opts) {
    println!("Call from reset: {:?}", opts)
}
