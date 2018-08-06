#![allow(unused_variables)]

extern crate libc;
extern crate chrono;
extern crate users;
extern crate ansi_term;
#[macro_use]
extern crate structopt;
#[macro_use]
extern crate clap;
extern crate term_grid;
extern crate term_size;

mod entry;
mod error;
mod opts;
mod view;
mod filter;

pub use entry::Entry;
pub use error::*;
pub use opts::Opts;

use std::env;
use structopt::StructOpt;

fn main() -> Result<(), Error> {
    let opts: Opts = Opts::from_args();
    if opts.debug { eprintln!("{:?}", opts) }

    // TODO: add proper handling for other dirs
    let dir = opts.files.get(0).cloned().unwrap_or(env::current_dir()?);

    let res: Result<Vec<_>,_> = dir.read_dir()?.map(|e: Result<_,_>| {
        e.map_err(|err| err.into())
         .and_then(|dir_entry| Entry::from_dir_entry(&dir_entry))
    }).collect();
    let res: Vec<Entry> = res?;
    let filtered: Vec<_> = filter(res, &opts);
    let sorted: Vec<_> = sort(filtered, &opts);
    view::view(sorted, &opts);

    Ok(())
}

fn filter(data: Vec<Entry>, opts: &Opts) -> Vec<Entry> {
    // TODO:
    data
}
fn sort(data: Vec<Entry>, opts: &Opts) -> Vec<Entry> {
    // TODO:
    data
}