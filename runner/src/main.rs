use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

pub fn install_helpers() -> eyre::Result<()> {
    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();
    eyre_hook.install()?;

    let stderr_logs = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(true); // The developers (me) view these logs, and I want to have colors 😊

    tracing_subscriber::registry().with(stderr_logs).init();

    // Install our panic hook before any others, to perform stuff first.
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        tracing::error!("{}", panic_hook.panic_report(info));

        default_panic(info);
    }));

    Ok(())
}
