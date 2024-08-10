//! Utilities for reading and updating the service history database.

use crate::{
    errors::Error,
    garmin::activities::{Activities, Activity},
};
use chrono::{DateTime, Utc};
use resolve_path::PathResolveExt;
use serde::{Deserialize, Serialize};
use std::{collections::BTreeSet, fs::File, time::Duration};

/// Service interval database - serializable to/from a flat file.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DB {
    /// List of components and service history.
    components: Vec<Component>,
}

impl DB {
    /// Load the service DB. The DB file must exist or an IOError will be returned.
    pub fn load(file_path: &str) -> Result<DB, Error> {
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

        serde_json::from_reader(&file).map_err(|e| Error::DBFormatError(e.into()))
    }

    /// Returns an iterator over the individual components.
    pub fn components(&self) -> impl Iterator<Item = &Component> + '_ {
        self.components.iter()
    }

    /// Returns an iterator over all components and their durations since last serviced.
    pub fn duration_since_last_serviced<'a>(
        &'a self,
        activity_data: &'a Activities,
    ) -> impl Iterator<Item = (&Component, Duration)> + 'a {
        self.components().map(|component| {
            (
                component,
                component.duration_since_last_serviced(activity_data.iter()),
            )
        })
    }
}

/// Service data for a single component.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Component {
    /// The component's service name (e.g. "Rockshox Lyrik 50hr")
    name: String,
    /// The service duration for this component.
    #[serde(with = "humantime_serde")]
    interval: Duration,
    /// List of dates at which the component was serviced.
    serviced: BTreeSet<DateTime<Utc>>,
}

impl Component {
    /// Name of the component, as defined in the database file.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Service interval for the component, as defined in the database file.
    pub fn interval(&self) -> Duration {
        self.interval
    }

    /// Date that the component was last serviced. If `None`, the component has never
    /// been serviced.
    ///
    /// There should generally always be a value, even if merely the purchase date as
    /// the component's service interval is effectively starting from zero at this point.
    pub fn last_serviced(&self) -> Option<&DateTime<Utc>> {
        self.serviced.last()
    }

    /// Duration of activity since the component was last serviced.
    ///
    /// If the component was never serviced, the duration will be equal to the total
    /// duration of all activities.
    pub fn duration_since_last_serviced<'a, A>(&self, activities: A) -> Duration
    where
        A: Iterator<Item = &'a Activity> + 'a,
    {
        activities
            .filter(|activity| {
                activity.date > *self.last_serviced().unwrap_or(&DateTime::UNIX_EPOCH)
            })
            .fold(Duration::default(), |acc, activity| {
                acc.saturating_add(activity.duration)
            })
    }
}
