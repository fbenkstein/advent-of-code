pub use nom::types::CompleteStr;
use nom::*;

#[derive(Debug, Clone)]
pub struct Point {
    pub x: u64,
    pub y: u64,
}

#[derive(Debug, Clone)]
pub struct Claim {
    pub id: u64,
    pub area: u64,
    pub top_left: Point,
    pub bottom_right: Point,
}

impl Claim {
    fn new(id: u64, left: u64, top: u64, width: u64, height: u64) -> Claim {
        Claim {
            id: id,
            area: width * height,
            top_left: Point { x: left, y: top },
            bottom_right: Point {
                x: left + width,
                y: top + height,
            },
        }
    }
}

named!(parse_u64(CompleteStr) -> u64,
    map_res!(recognize!(nom::digit), |input: CompleteStr| u64::from_str_radix(input.0, 10))
);

named!(parse_claim<CompleteStr, Claim>,
    do_parse!(
        char!('#') >>
        id: parse_u64 >>
        space1 >>
        char!('@') >>
        space1 >>
        left: parse_u64 >>
        char!(',') >>
        top: parse_u64 >>
        char!(':') >>
        space1 >>
        width: parse_u64 >>
        char!('x') >>
        height: parse_u64 >>
        space0 >>
        line_ending >>
        (Claim::new( id, left, top, width, height ))
    )
);

named!(pub parse_claims<CompleteStr, Vec<Claim>>,
    do_parse!(
        claims: many0!(parse_claim) >>
        (claims)
    )
);

#[test]
fn parse_intersect() {
    let input = r#"#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
"#;
    let claims = parse_claims(CompleteStr(input)).unwrap().1;
    // assert_eq!(claims.len(), 3);
}
