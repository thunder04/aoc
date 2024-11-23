use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

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

#[macro_export]
macro_rules! export_day {
    ($($id: literal: $name: literal, $day_mod: ident)*) => {
        $(mod $day_mod;)*

        pub type Runner = (Option<Solution>, Option<Solution>);
        pub type Solution = fn() -> u32;

        pub fn run() -> eyre::Result<()> {
            const AVAILABLE_DAYS: &[&'static str] = &[$($name),*];

            let Some(day) = args().nth(1) else {
                bail!("I expected a day argument")
            };

            match &*day {
                $(stringify!($id) => return _run($id, $day_mod::run()),)*

                _ => {
                    return Err(
                        eyre::Report::msg("this day hasn't been implemented").with_suggestion(|| {
                            format!(
                                "the available days are: {}",
                                AVAILABLE_DAYS.join(", ")
                            )
                        }),
                    )
                }
            }

            pub fn _run(id: u8, (part_1, part_2): Runner) -> eyre::Result<()> {
                if let Some(part_1) = part_1 {
                    info!("Day {id}, part 1's answer is \"{}\".", (part_1)());
                }

                if let Some(part_2) = part_2 {
                    info!("Day {id}, part 2's answer is \"{}\".", (part_2)());
                }

                Ok(())
            }
        }
    };
}
