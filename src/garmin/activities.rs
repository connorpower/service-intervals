//! Utilities for reading and working with Garmin activity files of the kind
//! retreived from https://connect.garmin.com/modern/activities.

use crate::errors::Error;
use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use resolve_path::PathResolveExt;
use serde::{de::Error as SerdeError, Deserialize, Deserializer};
use std::{fs::File, str::FromStr, time::Duration};

/// The in-memory representation of a Garmin activity file such as those downloaded from Garmin Connect:
/// https://connect.garmin.com/modern/activities?activityType=cycling&activitySubType=mountain_biking
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(transparent)]
pub struct Activities {
    /// List of activities in the activity file.
    activities: Vec<Activity>,
}

impl Activities {
    /// Load the garmin mountain bike activity file from which ride time will be summed.
    ///
    /// https://connect.garmin.com/modern/activities?activityType=cycling&activitySubType=mountain_biking
    pub fn load_file(file_path: &str) -> Result<Self, Error> {
        let path = file_path
            .resolve()
            .canonicalize()
            .map_err(|e| Error::IOError {
                source: e,
                file_path: file_path.to_string(),
            })?;

        let file = File::open(&path).map_err(|e| Error::IOError {
            source: e,
            file_path: path.display().to_string(),
        })?;
        let mut rdr = csv::Reader::from_reader(file);

        let activities = rdr
            .deserialize::<Activity>()
            .map(|result| result.map_err(Error::ActivityFormatError))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self { activities })
    }

    /// Returns an iterator over the individual activities.
    pub fn iter(&self) -> impl Iterator<Item = &Activity> + '_ {
        self.activities.iter()
    }

    /// Returns the total duration of all activities summed together.
    pub fn total_duration(&self) -> Duration {
        self.iter().fold(Duration::default(), |acc, activity| {
            acc.saturating_add(activity.duration)
        })
    }

    /// Returns the total duration that has been ridden since the provided `since` date.
    pub fn total_duration_since(&self, since: DateTime<Utc>) -> Duration {
        self.iter()
            .filter(|activity| activity.date > since)
            .fold(Duration::default(), |acc, activity| {
                acc.saturating_add(activity.duration)
            })
    }
}

/// A single activity entry in a Garmin activity file.
#[derive(Debug, Clone, Default, Deserialize)]
pub struct Activity {
    /// Garmin Activity Date
    #[serde(rename = "Date")]
    #[serde(deserialize_with = "Activity::deserialize_garmin_date")]
    pub date: DateTime<Utc>,

    /// Garmin Activity Duration
    #[serde(rename = "Time")]
    #[serde(deserialize_with = "Activity::deserialize_garmin_duration")]
    pub duration: Duration,
}

impl Activity {
    /// Deserialize a garmin activity date string of the format "YYYY-MM-DD HH:MM:SS" and
    /// return result as chrono `DateTime`.
    fn deserialize_garmin_date<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s: &str = Deserialize::deserialize(deserializer)?;
        let Ok(datetime) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") else {
            Err(D::Error::custom(
                "Date was not in YYYY-MM-DD HH:MM:SS format",
            ))?
        };

        Ok(Utc.from_local_datetime(&datetime).unwrap())
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
            Err(D::Error::custom("Duration was not in HH:MM:SS format"))?
        };

        let (Ok(hours), Ok(mins), Ok(secs)) = (
            u64::from_str(hours),
            u64::from_str(mins),
            u64::from_str(secs),
        ) else {
            Err(D::Error::custom("Duration was not in HH:MM:SS format"))?
        };

        let hours = Duration::from_secs(hours * 60 * 60);
        let mins = Duration::from_secs(mins * 60);
        let secs = Duration::from_secs(secs);

        Ok(hours.saturating_add(mins).saturating_add(secs))
    }
}
