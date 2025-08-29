use std::{io::{self, BufRead}};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    const MAX_STEP: i32 = 1_000_000_000;

    let t: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    for _ in 0..t {
        let n: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        let points: Vec<(i32, i32)> = (0..n)
            .map(|_| {
                let line = lines.next().unwrap().unwrap();
                let mut iter = line.split_whitespace();
                let x = iter.next().unwrap().parse().unwrap();
                let y = iter.next().unwrap().parse().unwrap();
                (x, y)
            })
            .collect();

        let min_x_plus_y = points.iter().map(|(x, y)| x + y).min().unwrap() as i64;
        let min_x_minus_y = points.iter().map(|(x, y)| x - y).min().unwrap() as i64;

        println!("? D {MAX_STEP}");
        let _: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        println!("? D {MAX_STEP}");
        let _: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        println!("? L {MAX_STEP}");
        let _: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        println!("? L {MAX_STEP}");
        let distance: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        // - x_1 + y_1 = min_x_plus_y
        // - (x_a, y_a) is the current coordinates
        // - |x_1 - x_a| + |y_1 - y_a| = distance, x_a < x_1, y_a < y_1
        // x_a = x - 2 * MAX_STEP, y_a = y - 2 * MAX_STEP
        let x_plus_y: i64 = min_x_plus_y - distance + 4 * MAX_STEP as i64;

        println!("? U {MAX_STEP}");
        let _: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        println!("? U {MAX_STEP}");
        let _: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        println!("? U {MAX_STEP}");
        let _: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        println!("? U {MAX_STEP}");
        let distance: i64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        // - x_2 + y_2 = min_x_minus_y
        // - (x_b, y_b) is the current coordinates
        // - |x_2 - x_b| + |y_2 - y_b| = distance, x_b < x_2, y_b > y_2
        // x_b = x - 2 * MAX_STEP, y_b = y + 2 * MAX_STEP
        let x_minus_y: i64 = min_x_minus_y - distance + 4 * MAX_STEP as i64;

        let x = (x_plus_y + x_minus_y) / 2_i64;
        let y = (x_plus_y - x_minus_y) / 2_i64;
        println!("! {x} {y}");
    }
}
