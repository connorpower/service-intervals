//! Utilities for reading and working with Garmin activity files of the kind
//! retreived from https://connect.garmin.com/modern/activities.

use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::{fs::File, time::Duration};

#[derive(Debug, Clone, Deserialize)]
pub struct Record {
    #[serde(rename = "Time")]
    #[serde(deserialize_with = "deserialize_garmin_duration")]
    pub time: Duration,
}

///// Load the garmin mountain bike activity file from which ride time will be summed.
/////
///// https://connect.garmin.com/modern/activities?activityType=cycling&activitySubType=mountain_biking&startDate=2023-01-1
//pub fn load_file(file_path: &str) -> Result<Vec<Record>, Box<dyn Error>> {
//    let file = File::open(file_path)?;
//    let mut rdr = csv::Reader::from_reader(file);
//
//    let foo = rdr.deserialize::<Record>();
//
//    // .collect()?
//    unimplemented!()
//}

/// Load the garmin mountain bike activity file from which ride time will be summed.
///
/// https://connect.garmin.com/modern/activities?activityType=cycling&activitySubType=mountain_biking&startDate=2023-01-1
pub fn load_file(
    file_path: &str,
) -> Result<impl Iterator<Item = Result<Record, csv::Error>>, csv::Error> {
    let file = File::open(file_path)?;
    let rdr = csv::Reader::from_reader(file);

    Ok(rdr.into_deserialize::<Record>())
}

/// Deserialize a garmin duration string of the format "HH:MM:SS" and
/// return result as a native rust `Duration`.
fn deserialize_garmin_duration<'de, D>(deserializer: D) -> Result<Duration, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    let mut it = s.split(':');
    let (Some(hours), Some(mins), Some(secs)) = (it.next(), it.next(), it.next()) else {
        panic!("TODO: proper error");
    };

    let (Ok(hours), Ok(mins), Ok(secs)) = (
        u64::from_str(hours),
        u64::from_str(mins),
        u64::from_str(secs),
    ) else {
        panic!("TODO: proper error");
    };

    let hours = Duration::from_secs(hours * 60 * 60);
    let mins = Duration::from_secs(mins * 60);
    let secs = Duration::from_secs(secs);

    Ok(hours.saturating_add(mins).saturating_add(secs))
}
