use clap::Parser;

mod args;

use args::Args;

pub const LOR_ID: usize = 1256670;

fn main() {
    let _ = Args::parse();
}
