use clap::Parser;
mod args;
mod comid;
mod corim;
mod coswid;
mod cots;
mod eat;
mod utils;

use crate::comid::comid_main;
use crate::corim::corim_main;
use crate::coswid::coswid_main;
use crate::cots::cots_main;
use crate::eat::eat_main;
use args::*;
use clap::CommandFactory;
use std::env;

fn main() {
    let e = env::args_os();
    if 1 == e.len() {
        let mut a = CfcliArgs::command();
        if let Err(_e) = a.print_help() {
            println!("Error printing help. Try again with -h parameter.")
        }
        return;
    }
    let args: CfcliArgs = CfcliArgs::parse();
    match &args.command {
        Commands::Comid(c) => comid_main(c),
        Commands::Corim(c) => corim_main(c),
        Commands::Coswid(c) => coswid_main(c),
        Commands::Cots(c) => cots_main(c),
        Commands::Eat(c) => eat_main(c),
    }
}
