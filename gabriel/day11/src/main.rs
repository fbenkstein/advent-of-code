use itertools::iproduct;
use rayon::prelude::*;

// TODO:
// * grow the rectangle instead of recomputing all the time
// * use simd because why not
fn main() {
    println!("Coordinate of the top-left fuel cell of the 3x3 square with the largest total power: {:?}", max_power(2866));
    println!("Identifier of the square with the largest total power: {:?}", max_power2(2866));
}

fn max_power(grid_serial_number: usize) -> (usize, usize) {
    iproduct!(0..300, 0..300)
        .max_by_key(|&(x, y)| FuelCell::square_power_level(x, y, 3, grid_serial_number))
        .expect("duh")
}

fn max_power2(grid_serial_number: usize) -> (usize, usize, usize) {
    iproduct!(0..300, 0..300, 1..300).collect::<Vec<_>>()
        .into_par_iter()
        .filter(|(x,y, size)| x.max(y) + size < 300)
        .max_by_key(|&(x, y, size)| FuelCell::square_power_level(x, y, size, grid_serial_number))
        .expect("duh")
}

struct FuelCell;

impl FuelCell {
    fn power_level(x: usize, y: usize, grid_serial_number: usize) -> isize {
        let rack_id = x + 10;
        let mut power_level = rack_id * y;
        power_level += grid_serial_number;
        power_level *= rack_id;
        power_level = power_level % 1000 / 100;
        power_level as isize - 5
    }

    fn square_power_level(x: usize, y: usize, size: usize, grid_serial_number: usize) -> isize {
        let mut power_level = 0;
        for x_offset in 0..size {
            for y_offset in 0..size {
                power_level += Self::power_level(x + x_offset, y + y_offset, grid_serial_number);
            }
        }
        power_level
    }
}

#[test]
fn test_power_levels() {
    assert_eq!(FuelCell::power_level(3, 5, 8), 4);
    assert_eq!(FuelCell::power_level(122, 79, 57), -5);
    assert_eq!(FuelCell::power_level(217, 196, 39), 0);
    assert_eq!(FuelCell::power_level(101, 153, 71), 4);
}

#[test]
fn test_max_power() {
    assert_eq!(max_power(18), (33, 45));
    assert_eq!(max_power(42), (21, 61));
}

#[test]
fn test_max_power2() {
    assert_eq!(max_power2(18), (90, 269, 16));
    assert_eq!(max_power2(42), (232, 251, 12));
}