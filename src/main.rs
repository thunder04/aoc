#![feature(iterator_try_collect)]

macro_rules! days {
    ($year: ident, $_0th: expr, $args: expr) => {
        match $args.peek().map(|x| &**x) {
            Some("all") => aoc::$year::ALL_DAYS.to_vec(),
            Some(_) => $args.map(|x| x.parse::<u8>()).try_collect::<Vec<_>>()?,
            None => eyre::bail!("Usage: {} <year> <...days>", $_0th.unwrap()),
        }
    };
}

fn main() -> eyre::Result<()> {
    install_helpers()?;

    let mut args = std::env::args().peekable();
    let _0th = args.next();

    match args.next().as_deref() {
        Some("2024") => aoc::_2024::run(days!(_2024, _0th, args)),
        Some("2023") => aoc::_2023::run(days!(_2023, _0th, args)),
        _ => eyre::bail!("The year doesn't exist"),
    }
}

pub fn install_helpers() -> eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt as _, util::SubscriberInitExt as _};

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
