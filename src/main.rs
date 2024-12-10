#![feature(iterator_try_collect)]

fn main() -> eyre::Result<()> {
    install_helpers()?;

    let mut args = std::env::args();
    let _0th = args.next();
    let year = args.next();
    let days = args.map(|x| x.parse::<u8>()).try_collect::<Vec<_>>()?;

    if year.is_none() || days.is_empty() {
        eyre::bail!("Usage: {} <year> <...days>", _0th.unwrap());
    } else {
        match year.as_deref() {
            Some("2024") => aoc::_2024::run(days),
            Some("2023") => aoc::_2023::run(days),
            _ => eyre::bail!("The year doesn't exist"),
        }
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
