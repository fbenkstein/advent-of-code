use itertools::Itertools;
use text_io::{scan, try_scan};

type Id = u16;
type Date = (u16, u8, u8, u8, u8);
type MinutesTable = Vec<Vec<Id>>;

#[derive(Debug)]
enum Event {
    BeginShift(Id),
    FallAsleep,
    WakeUp,
}

fn parse(input: &str) -> Vec<(Date, Event)> {
    let mut events: Vec<_> = input
        .lines()
        .map(|line| {
            let y: u16;
            let m: u8;
            let d: u8;
            let h: u8;
            let min: u8;
            let event: String;
            let suffix: String;

            scan!(line.bytes() => "[{}-{}-{} {}:{}] {} {}", y, m, d, h, min, event, suffix);

            let event = match &event[..] {
                "Guard" => {
                    let id: Id;
                    scan!(suffix.bytes() => "#{}", id);
                    Event::BeginShift(id)
                }
                "falls" => Event::FallAsleep,
                "wakes" => Event::WakeUp,
                _ => panic!("unexpected"),
            };

            ((y, m, d, h, min), event)
        })
        .collect();
    events.sort_by_key(|(date, _)| *date);
    events
}

fn events_to_minutes_table(events: &[(Date, Event)]) -> MinutesTable {
    let mut minutes = Vec::with_capacity(60);
    minutes.resize(60, Vec::new()); // minute -> guard id
    events.iter().fold(
        (0, 0),
        |(last_id, last_min), ((_, _, _, _, min), event)| match event {
            Event::BeginShift(id) => (*id, *min),
            Event::FallAsleep => (last_id, *min),
            Event::WakeUp => {
                for m in last_min..*min {
                    minutes[m as usize].push(last_id);
                }
                (last_id, *min)
            }
        },
    );
    minutes
}

fn solve1(minutes: &MinutesTable) -> usize {
    let (_, max_asleep_guard) = Iterator::flatten(minutes.iter().map(|entry| entry.iter()))
        .sorted()
        .into_iter()
        .group_by(|id| *id)
        .into_iter()
        .map(|(id, group)| (group.count(), *id))
        .max()
        .unwrap();

    let (_, max_asleep_minute) = minutes
        .iter()
        .enumerate()
        .map(|(minute, entry)| {
            (
                entry.iter().filter(|id| **id == max_asleep_guard).count(),
                minute,
            )
        })
        .max()
        .unwrap();

    max_asleep_guard as usize * max_asleep_minute
}

fn solve2(minutes: MinutesTable) -> usize {
    let (_, most_frequently_asleep, minute) = minutes
        .into_iter()
        .enumerate()
        .filter_map(|(minute, mut entry)| {
            entry.sort();
            entry
                .into_iter()
                .group_by(|id| *id)
                .into_iter()
                .map(|(id, same_ids)| (same_ids.count(), id))
                .max()
                .map(|(count, id)| (count, id, minute))
        })
        .max()
        .unwrap();
    most_frequently_asleep as usize * minute
}

pub fn solve(s: &str) -> (usize, usize) {
    let minutes = events_to_minutes_table(&parse(s));
    (solve1(&minutes), solve2(minutes))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve() {
        let input = r#"[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up"#;

        assert_eq!(solve(input), (240, 4455));
    }
}
