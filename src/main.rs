use battery::Manager;
use notify_rust::{Notification, NotificationHint, NotificationUrgency, Timeout};
use std::num::NonZeroU32;
use std::thread::sleep;
use std::time::Duration;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(about, author)]
struct Opts {
    /// The battery percentage at which to warn the user about low battery
    #[structopt(long = "low")]
    low_percent: Option<u8>,

    /// The battery percentage at which to warn the user about critical battery
    #[structopt(long = "critical")]
    critical_percent: Option<u8>,

    /// Check battery level at this interval
    #[structopt(long, name = "seconds", default_value = "60")]
    interval: NonZeroU32,
}

#[derive(Default)]
struct WarningState {
    low_warned: bool,
    critical_warned: bool,
}

fn handle_warning<E>(
    percent: f32,
    warned: &mut bool,
    threshold: f32,
    warn: impl Fn() -> Result<(), E>,
) -> Result<(), E> {
    if percent <= threshold && !*warned {
        *warned = true;
        warn()
    } else {
        if percent > threshold && *warned {
            *warned = false;
        }
        Ok(())
    }
}

fn main() {
    let opts: Opts = StructOpt::from_args();

    if let (Some(critical), Some(low)) = (opts.critical_percent, opts.low_percent) {
        if critical >= low {
            eprintln!("Critical battery percentage should be less than low battery percentage");
            return;
        }
    }

    let manager = Manager::new().expect("getting battery manager");
    let batteries = manager
        .batteries()
        .expect("getting batteries")
        .collect::<Result<Vec<_>, _>>()
        .expect("getting battery");

    let mut batteries = batteries
        .into_iter()
        .map(|b| (b, WarningState::default()))
        .collect::<Vec<_>>();

    let critical_percent = opts.critical_percent.map(|v| f32::from(v) / 100.);
    let low_percent = opts.low_percent.map(|v| f32::from(v) / 100.);

    let notification_base = Notification::new()
        .timeout(Timeout::Never)
        .urgency(NotificationUrgency::Critical)
        .sound_name("battery-low")
        .hint(NotificationHint::Transient(true))
        .finalize();

    loop {
        for (battery, state) in &mut batteries {
            manager.refresh(battery).expect("refreshing battery state");

            let percent: f32 = (battery.energy() / battery.energy_full()).value;

            if let Some(low) = low_percent {
                handle_warning(percent, &mut state.low_warned, low, || {
                    notification_base
                        .clone()
                        .body(&format!("Battery is low: {:.1}%", percent * 100.))
                        .show()
                        .map(|_| ())
                })
                .expect("show notification");
            }

            if let Some(crit) = critical_percent {
                handle_warning(percent, &mut state.critical_warned, crit, || {
                    notification_base
                        .clone()
                        .summary("Battery critical")
                        .body(&format!("Battery is critical: {:.1}%", percent * 100.))
                        .show()
                        .map(|_| ())
                })
                .expect("show notification");
            }
        }

        sleep(Duration::from_secs(opts.interval.get().into()));
    }
}
