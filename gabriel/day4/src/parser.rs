pub use nom::types::CompleteStr;
use nom::*;

#[derive(Debug, PartialEq, Default, PartialOrd, Eq, Ord)]
pub struct DateTime {
    pub year: u32,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub enum Action {
    BeginsShift(u32),
    FallsAsleep,
    WakesUp,
}

#[derive(Debug, PartialEq, PartialOrd, Eq, Ord)]
pub struct SpyingEvent {
    pub date: DateTime,
    pub action: Action,
}

named!(parse_u8(CompleteStr) -> u8,
    map_res!(recognize!(nom::digit), |input: CompleteStr| u8::from_str_radix(input.0, 10))
);

named!(parse_u32(CompleteStr) -> u32,
    map_res!(recognize!(nom::digit), |input: CompleteStr| u32::from_str_radix(input.0, 10))
);

named!(parse_claim<CompleteStr, SpyingEvent>,
    do_parse!(
        char!('[') >>
        year: parse_u32 >>
        char!('-') >>
        month: parse_u8 >>
        char!('-') >>
        day: parse_u8 >>
        space1 >>
        hour: parse_u8 >>
        char!(':') >>
        minute: parse_u8 >>
        char!(']') >>
        space1 >>
        action: alt!(
            do_parse!(
                tag!("Guard #") >>
                id: parse_u32 >>
                tag!(" begins shift") >>
                (Action::BeginsShift(id))
            ) |
            do_parse!(
                tag!("falls asleep") >>
                (Action::FallsAsleep)
            ) |
            do_parse!(
                tag!("wakes up") >>
                (Action::WakesUp)
            )

        ) >>
        opt!(line_ending) >>
        (SpyingEvent { date: DateTime { year, month, day, hour, minute }, action })
    )
);

named!(pub parse_claims<CompleteStr, Vec<SpyingEvent>>,
    do_parse!(
        claims: many0!(parse_claim) >>
        (claims)
    )
);
