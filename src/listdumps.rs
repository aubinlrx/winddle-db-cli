use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Opts {
    environment: Option<String>
}

pub fn call(opts: Opts) {
    println!("Call from list-dumps: {:?}", opts.environment)
}
