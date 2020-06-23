use structopt::StructOpt;

mod switch;
mod reset;
mod info;
mod listdumps;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "db",
    about = "Work seamlessly with Winddle development database from the command line."
)]
pub struct Opts {
    // SUBCOMMANDS
    #[structopt(subcommand)]
    commands: Option<Db>

}

#[derive(Debug, StructOpt)]
enum Db {
    #[structopt(name="info")]
    Info(info::Opts),

    #[structopt(name="switch")]
    Switch(switch::Opts),

    #[structopt(name="reset")]
    Reset(reset::Opts),

    #[structopt(name="list-dumps")]
    ListDumps(listdumps::Opts),
}

fn main() {
    let opts = Opts::from_args();

    // handle subcommand
    if let Some(subcommand) = opts.commands{
        match subcommand {
            Db::Info(opts) => info::call(opts),
            Db::Switch(opts) => switch::call(opts),
            Db::Reset(opts) => reset::call(opts),
            Db::ListDumps(opts) => listdumps::call(opts),
        }
    }
}
