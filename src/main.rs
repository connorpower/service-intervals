use anyhow::{Context, Result};
use clap::Parser;
use service_intervals::garmin;
use std::{process, time::Duration};

/// Simple program to calculate next service intervals based on ride time.
#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// File path for a garmin connect activity CSV for mountain biking.
    ///
    /// https://connect.garmin.com/modern/activities?activityType=cycling&activitySubType=mountain_biking&startDate=2024-01-1
    #[arg(short, long)]
    file_path: String,
}

fn main() {
    let args = Args::parse();

    if let Err(err) = run(args) {
        println!("{:?}", err);
        process::exit(1);
    }
}

fn run(args: Args) -> Result<()> {
    let mut records =
        garmin::activities::load_file(&args.file_path).context("Failed to load activity file")?;

    let total_duration: Duration = records
        .try_fold(Duration::default(), |acc, result| {
            result.map(|record| acc.saturating_add(record.time))
        })
        .context("Failed to process activity records")?;
    let hours = total_duration.as_secs() / 60 / 60;
    println!("{hours} hr{s}", s = if hours == 1 { "" } else { "s" });

    Ok(())
}
