use std::error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prev_depth = None;
    let mut num_increase = 0;
    let depth_sums : Vec<_> = io::stdin().lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|x| x.trim().parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?
        .windows(3)
        .map(|window| window.iter().fold(0, |x, y| x + y))
        .collect();

    for depth in depth_sums {
        if let Some(x) = prev_depth {
            if depth > x {
                num_increase += 1;
            }
        }
        prev_depth = Some(depth);
    }
    println!("Increased depth {} times", num_increase);
    Ok(())
}
