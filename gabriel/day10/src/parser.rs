pub use nom::types::CompleteStr;
use nom::*;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct LightPoint {
    pub x: i32,
    pub y: i32,
    velocity_x: i32,
    velocity_y: i32,
}

impl LightPoint {
    pub fn translate(&mut self) {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
    }

    pub fn has_neighbor(&self, points: &Vec<LightPoint>) -> bool {
        for point in points.iter() {
            if self.is_neighbour_of(point) {
                return true;
            }
        }
        return false;
    }

    pub fn is_neighbour_of(&self, point: &LightPoint) -> bool {
        self != point
            && point.x >= self.x - 1
            && point.x <= self.x + 1
            && point.y >= self.y - 1
            && point.y <= self.y + 1
    }
}

named!(parse_i32(CompleteStr) -> i32,
    map_res!(
        recognize!(many1!(alt!(tag!("-") | tag!("+") | digit))), |input: CompleteStr| i32::from_str_radix(&input, 10)
    )
);

named!(comma_separated(CompleteStr) -> (i32, i32),
    do_parse!(
        char!('<') >>
        space0 >>
        x: parse_i32 >>
        space0 >>
        char!(',') >>
        space0 >>
        y: parse_i32 >>
        space0 >>
        char!('>') >>
        (x, y)
    )
);

named!(parse_light_point<CompleteStr, LightPoint>,
    do_parse!(
        tag!("position=") >>
        position: comma_separated >>
        space0 >>
        tag!("velocity=") >>
        velocity: comma_separated >>
        space0 >>
        opt!(line_ending) >>
        (LightPoint { x: position.0, y: position.1, velocity_x: velocity.0, velocity_y: velocity.1})
    )
);

named!(pub parse_light_points<CompleteStr, Vec<LightPoint>>,
    do_parse!(
        light_points: many0!(parse_light_point) >>
        (light_points)
    )
);
