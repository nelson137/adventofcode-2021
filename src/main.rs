use std::error::Error;

use structopt::StructOpt;

mod days;
use days::Cli;

mod util;

fn main() -> Result<(), Box<dyn Error>> {
    Cli::from_args().run()
}
