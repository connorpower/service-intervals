use serde::{Deserialize, Deserializer};
use std::str::FromStr;
use std::time::Duration;

#[allow(dead_code)]
#[derive(Debug, Clone, Deserialize)]
pub struct Record {
    #[serde(rename = "Time")]
    #[serde(deserialize_with = "deserialize_time")]
    pub time: Duration,
}

fn deserialize_time<'de, D>(deserializer: D) -> Result<Duration, D::Error>
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
