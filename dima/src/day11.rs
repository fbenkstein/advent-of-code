fn make_grid(grid_serial_number: i64) -> Vec<i64> {
    let mut grid = Vec::with_capacity(300 * 300);
    for y in 1..=300 {
        for x in 1..=300 {
            let rack_id = x + 10;
            let power_level = rack_id * y + grid_serial_number;
            let power_level = power_level * rack_id;
            let power_level = power_level % 1000 / 100 - 5;
            grid.push(power_level);
        }
    }
    grid
}

fn solve1(grid: &[i64]) -> (usize, usize) {
    let mut max_x = usize::min_value();
    let mut max_y = usize::min_value();
    let mut max_power_level = i64::min_value();

    for x in 0..298 {
        for y in 0..298 {
            let mut power_level = 0;
            for (dx, dy) in [
                (0, 0),
                (1, 0),
                (2, 0),
                (0, 1),
                (1, 1),
                (2, 1),
                (0, 2),
                (1, 2),
                (2, 2),
            ]
            .iter()
            {
                let idx = (x + dx + 300 * (y + dy)) as usize;
                power_level += grid[idx];
            }

            if power_level > max_power_level {
                max_x = x;
                max_y = y;
                max_power_level = power_level;
            }
        }
    }

    (max_x + 1, max_y + 1)
}

fn solve2(grid: &[i64]) -> (usize, usize, usize) {
    let mut max_x = usize::min_value();
    let mut max_y = usize::min_value();
    let mut max_size = usize::min_value();
    let mut max_power_level = i64::min_value();

    // Don't judge, I am lazy!
    for x in 0..300 {
        for y in 0..300 {
            for size in 1..(300 - x.max(y)) {
                let mut power_level = 0;
                for dx in 0..size {
                    for dy in 0..size {
                        let idx = (x + dx + 300 * (y + dy)) as usize;
                        power_level += grid[idx];
                    }
                }

                if power_level > max_power_level {
                    max_x = x;
                    max_y = y;
                    max_size = size;
                    max_power_level = power_level;
                }
            }
        }
    }

    (max_x + 1, max_y + 1, max_size)
}

pub fn solve(input: &str) -> ((usize, usize), (usize, usize, usize)) {
    let grid_serial_number: i64 = input.parse().unwrap();
    let grid = make_grid(grid_serial_number);
    (solve1(&grid), solve2(&grid))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(&make_grid(18)), (33, 45))
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(&make_grid(18)), (90, 269, 16));
        assert_eq!(solve2(&make_grid(42)), (232, 251, 12))
    }
}
