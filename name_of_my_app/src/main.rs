#[macro_use]
extern crate tracing;

fn main() -> eyre::Result<()> {
    install_helpers()?;

    info!("A: Hello.");
    panic!("B: Hi!");
}

fn install_helpers() -> eyre::Result<()> {
    use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

    let (panic_hook, eyre_hook) = color_eyre::config::HookBuilder::default().into_hooks();
    eyre_hook.install()?;

    let stderr_logs = tracing_subscriber::fmt::layer()
        .with_writer(std::io::stderr)
        .with_ansi(true); // The developers (me) view these logs, and I want to have colors 😊

    tracing_subscriber::registry().with(stderr_logs).init();

    // Install our panic hook before any others, to perform stuff first.
    let default_panic = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        error!("{}", panic_hook.panic_report(info));

        default_panic(info);
    }));

    Ok(())
}
