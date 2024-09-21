mod days;

use aoc_common::install_helpers;

#[macro_use]
extern crate tracing;

fn main() -> eyre::Result<()> {
    install_helpers()?;
    days::run()
}
