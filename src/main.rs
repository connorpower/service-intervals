use clap::Parser;
use service_intervals::garmin;
use std::{error::Error, process, time::Duration};

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
        println!("{}", err);
        process::exit(1);
    }
}

fn run(args: Args) -> Result<(), Box<dyn Error>> {
    let records = garmin::activities::load_file(&args.file_path)?;

    let total_duration: Duration = records
        .into_iter()
        .fold(Duration::default(), |acc, result| {
            acc.saturating_add(result.expect("TODO - handle error").time)
        });
    let hours = total_duration.as_secs() / 60 / 60;
    println!("{hours} hr{s}", s = if hours == 1 { "" } else { "s" });

    Ok(())
}
