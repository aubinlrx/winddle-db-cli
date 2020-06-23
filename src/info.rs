use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {}

pub fn call(opts: Opts) {
    println!("Call from info: {:?}", opts)
}
