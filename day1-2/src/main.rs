use std::error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let depth_increases = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .iter()
        .map(|x| x.trim().parse::<usize>())
        .collect::<Result<Vec<_>, _>>()?
        .windows(3)
        .map(|window| window.iter().fold(0, |x, y| x + y))
        .fold((0, None), |(result, prev), current| match prev {
            Some(prev) if current > prev => (result + 1, Some(current)),
            _ => (result, Some(current)),
        })
        .0;

    println!("Increased depth {} times", depth_increases);
    Ok(())
}
