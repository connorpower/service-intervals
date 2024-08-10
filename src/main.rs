use anyhow::{Context, Result};
use clap::Parser;
use service_intervals::{db::DB, garmin::activities::Activities};
use std::{process, time::Duration};

/// Simple program to calculate next service intervals based on ride time.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path for a garmin connect activity CSV for mountain biking.
    ///
    /// https://connect.garmin.com/modern/activities?activityType=cycling&activitySubType=mountain_biking&startDate=2024-01-1
    #[arg(short, long)]
    activity_file: String,

    /// File path for the service interval database. This is user-specific data
    /// and should be stored in a home directory, or similar.
    ///
    /// Defaults to `$HOME/Dropbox/.service-intervals.json`
    #[arg(short, long, default_value = "~/Dropbox/service-intervals.json")]
    db_file: String,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = run(args) {
        println!("{:?}", err);
        process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let activity_data =
        Activities::load_file(&args.activity_file).context("Failed to load activity file")?;

    let db = DB::load(&args.db_file).context("Failed to load database file")?;
    for (component, duration) in db.duration_since_last_serviced(&activity_data) {
        let component_name = component.name();
        let duration = hours(duration);
        let service_interval = hours(component.interval());
        println!(
            "{component_name}: {duration}hrs / {service_interval}hrs {status}",
            status = if duration > service_interval {
                "SERVICE NOW"
            } else {
                "OK"
            }
        );
    }

    Ok(())
}

fn hours(duration: Duration) -> u64 {
    duration.as_secs() / 60 / 60
}
