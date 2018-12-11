#[derive(Debug, Default)]
struct Star {
    x: i32,
    y: i32,
    vel_x: i32,
    vel_y: i32,
}

pub fn solve(input: &str) -> usize {
    let mut stars: Vec<Star> = input.lines().map(|line| {
        let mut star : Star = Default::default();
        scan!(line.bytes() => "position=<{},{}> velocity=<{},{}>",star.x, star.y, star.vel_x, star.vel_y);
        star
    }).collect();

    let mut min_height: usize = 20;
    let mut min_width: usize = 0;
    let mut min_m: Vec<bool> = vec![false; 0];
    let mut min_step = 0;

    for step in 0..20000 {
        let min_x = stars.iter().map(|star| star.x).min().unwrap();
        let min_y = stars.iter().map(|star| star.y).min().unwrap();
        let width = (stars.iter().map(|star| star.x).max().unwrap() - min_x + 1) as usize;
        let height = (stars.iter().map(|star| star.y).max().unwrap() - min_y + 1) as usize;

        if height < min_height {
            let mut m = vec![false; width * height];

            for star in &stars {
                let x = (star.x - min_x) as usize;
                let y = (star.y - min_y) as usize;
                m[x + y * width] = true;
            }

            min_height = height;
            min_width = width;
            min_m = m;
            min_step = step;
        }

        for star in &mut stars {
            star.x += star.vel_x;
            star.y += star.vel_y;
        }
    }

    for y in 0..min_height {
        for x in 0..min_width {
            if min_m[x + y * min_width] {
                print!("#");
            } else {
                print!(" ");
            }
        }
        println!("");
    }
    min_step
}
