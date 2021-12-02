use std::io;
use std::error;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let mut prev_depth = None;
    let mut num_increase = 0;
        for line in io::stdin().lock().lines() {
            let depth = line?.trim().parse::<usize>()?;
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
