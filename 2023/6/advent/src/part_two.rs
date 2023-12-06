fn solve_quadratic(a: f64, b: f64, c: f64) -> Result<(f64, f64), String> {
    let discriminant = b * b - 4.0 * a * c;

    if discriminant > 0.0 {
        let sqrt_discriminant = discriminant.sqrt();
        let root1 = (-b + sqrt_discriminant) / (2.0 * a);
        let root2 = (-b - sqrt_discriminant) / (2.0 * a);
        Ok((root1, root2))
    } else if discriminant == 0.0 {
        let root = -b / (2.0 * a);
        Ok((root, root))
    } else {
        Err("No real roots".to_string())
    }
}

const NUDGE: f64 = 0.000000000001;

fn main() {
    let input: Vec<(f64, f64)> = vec![
        (58819676.0, 434104122191218.0),
    ];

    // time_held**2 - total_time * time_held + distance = 0

    let mut total = 1.0;
    for pair in input {
        match solve_quadratic(1.0, -pair.0, pair.1) {
            Ok((root1, root2)) => {
                total *= (root1 - NUDGE).floor() - (root2 + NUDGE).ceil() + 1.0;
            }
            Err(error) => {
                eprintln!("Error: {}", error);
                continue;
            }
        }
    }
    println!("Total: {}", total);
}
