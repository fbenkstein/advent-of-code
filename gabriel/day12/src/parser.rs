pub use nom::types::CompleteStr;
use nom::*;

use std::collections::VecDeque;

#[derive(Debug)]
pub struct Rule {
    pub pattern: Vec<bool>, // TODO: don't allocate here
    pub has_plant: bool,
}

named!(plant<CompleteStr, bool>,
    do_parse!(
        has_plant: alt!(
            map!(tag!("#"), |_| true) |
            map!(tag!("."), |_| false)
        ) >>
        (has_plant)
    )
);

named!(rule<CompleteStr, Rule>,
    do_parse!(
        pattern: count!(plant, 5) >>
        space1 >>
        tag!("=>") >>
        space1 >>
        has_plant: plant >>
        opt!(line_ending) >>
        (Rule { pattern, has_plant })
    )
);

named!(pub parse_input<CompleteStr, (VecDeque<bool>, Vec<Rule>)>,
    do_parse!(
        tag!("initial state:") >>
        space1 >>
        initial_state: map!(many0!(plant), VecDeque::from) >>
        many1!(line_ending) >>
        rules: many1!(rule) >>
        (initial_state, rules)
    )
);
