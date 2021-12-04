use std::error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let result = io::stdin()
        .lock()
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let split = line.trim().split_whitespace().collect::<Vec<_>>();
            Some((split[0].to_lowercase(), split[1].parse::<i32>().ok()?))
        })
        .fold((0, 0), |acc, (command, argument)| match command.as_str() {
            "forward" => (acc.0 + argument, acc.1),
            "down" => (acc.0, acc.1 + argument),
            "up" => (acc.0, acc.1 - argument),
            _ => acc,
        });

    println!("Total is {}", result.0 * result.1);

    Ok(())
}
