use service_intervals::garmin;
use std::{env, error::Error, ffi::OsString, fs::File, process, time::Duration};

fn main() {
    if let Err(err) = run() {
        println!("{}", err);
        process::exit(1);
    }
}

fn run() -> Result<(), Box<dyn Error>> {
    let file_path = get_first_arg()?;
    let file = File::open(file_path)?;
    let mut rdr = csv::Reader::from_reader(file);

    let mut errors = vec![];
    let total_duration = rdr.deserialize::<garmin::activity_csv::Record>().fold(
        Duration::default(),
        |acc, record| match record {
            Ok(record) => acc.saturating_add(record.time),
            Err(e) => {
                errors.push(e);
                acc
            }
        },
    );

    if let Some(e) = errors.pop() {
        Err(e)?;
    } else {
        let hours = total_duration.as_secs() / 60 / 60;
        println!("{hours} hr{s}", s = if hours == 1 { "" } else { "s" });
    }

    Ok(())
}

fn get_first_arg() -> Result<OsString, Box<dyn Error>> {
    match env::args_os().nth(1) {
        None => Err(From::from("expected 1 argument, but got none")),
        Some(file_path) => Ok(file_path),
    }
}
