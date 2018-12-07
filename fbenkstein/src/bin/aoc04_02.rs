#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate itertools;
extern crate regex;

use std::collections::BTreeMap;
use std::str::FromStr;

use chrono::prelude::*;
use chrono::Duration;

use itertools::Itertools;

use regex::Regex;

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq)]
enum Event {
    BeginsShift { id: u32 },
    FallsAsleep,
    WakesUp,
}

impl FromStr for Event {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref BEGINS_SHIFT_RE: Regex =
                Regex::new("^Guard #(?P<id>.*) begins shift$").expect("invalid regex");
        }

        if s == "wakes up" {
            Ok(Event::WakesUp)
        } else if s == "falls asleep" {
            Ok(Event::FallsAsleep)
        } else if let Some(id_str) = BEGINS_SHIFT_RE
            .captures(s)
            .and_then(|c| c.name("id"))
            .map(|m| m.as_str())
        {
            let id: u32 = id_str.parse().map_err(|_| "invalid id")?;
            Ok(Event::BeginsShift { id })
        } else {
            Err("invalid event".to_string())
        }
    }
}

#[derive(Debug, PartialOrd, Ord, Eq, PartialEq)]
struct Record {
    timestamp: NaiveDateTime,
    event: Event,
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE: Regex =
                Regex::new(r"^\[(?P<timestamp>.*)\] (?P<event>.*)$").expect("invalid regex");
        }
        let captures = RE
            .captures(s)
            .ok_or_else(|| "invalid record string".to_string())?;
        let timestamp_str = captures.name("timestamp").unwrap().as_str();
        let timestamp = NaiveDateTime::parse_from_str(timestamp_str, "%F %R")
            .map_err(|e| format!("invalid timestamp: {}", e))?;
        let event_str = captures.name("event").unwrap().as_str();
        let event: Event = event_str.parse()?;
        Ok(Record { timestamp, event })
    }
}

fn main() {
    let data = include_str!("input_04.txt");
    let records: Vec<Record> = data.lines().map(|s| s.parse().unwrap()).sorted();

    #[derive(Debug)]
    struct SleepRecord {
        start: NaiveDateTime,
        end: NaiveDateTime,
    }

    let mut guard_sleep_records = BTreeMap::new();
    let mut guard_on_shift = None;
    let mut sleep_start = None;

    for record in records {
        match record.event {
            Event::BeginsShift { id } => guard_on_shift = Some(id),
            Event::FallsAsleep => sleep_start = Some(record.timestamp),
            Event::WakesUp => guard_sleep_records
                .entry(guard_on_shift.unwrap())
                .or_insert_with(Vec::new)
                .push(SleepRecord {
                    start: sleep_start.unwrap(),
                    end: record.timestamp,
                }),
        }
    }

    let mut guards_asleep_each_minute_counts = BTreeMap::new();

    for (id, sleep_records) in guard_sleep_records.iter() {
        for SleepRecord {
            start: sleep_start,
            end: sleep_end,
        } in sleep_records
        {
            let mut t = *sleep_start;

            while t != *sleep_end {
                *guards_asleep_each_minute_counts
                    .entry(t.minute())
                    .or_insert_with(BTreeMap::new)
                    .entry(id)
                    .or_insert(0u32) += 1;
                t += Duration::minutes(1);
            }
        }
    }

    guards_asleep_each_minute_counts
        .iter()
        .map(|(minute, guard_asleep_counts)| {
            (
                minute,
                guard_asleep_counts
                    .iter()
                    .minmax_by_key(|(_guard, count)| *count)
                    .into_option()
                    .map(|(_min, max)| max)
                    .unwrap(),
            )
        })
        .minmax_by_key(|(_minute, (_most_asleep_guard_id, count))| *count)
        .into_option()
        .map(|(_min, max)| max)
        .map(|(minute, (most_asleep_guard_id, _count))| {
            println!("{:?}", *minute * *most_asleep_guard_id)
        });
}
