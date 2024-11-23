use aoc_common::install_helpers;

fn main() -> eyre::Result<()> {
    install_helpers()?;

    let mut args = std::env::args();

    match (args.next(), args.next().as_deref(), args.next()) {
        (_, Some("2023"), Some(day)) => aoc_solutions::_2023::run(day.parse()?),

        (_, Some(_), Some(_)) => eyre::bail!("This year doesn't exist"),
        (Some(zeroth), _, _) => eyre::bail!("Usage: {zeroth} <year> <day>"),
        (None, _, _) => unreachable!(),
    }
}
