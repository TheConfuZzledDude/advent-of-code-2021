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
        .fold(
            (0, 0, 0),
            |acc @ (position, depth, aim), (command, argument)| match command.as_str() {
                "forward" => (position + argument, depth + aim * argument, aim),
                "down" => (position, depth, aim + argument),
                "up" => (position, depth, aim - argument),
                _ => acc,
            },
        );

    println!("Total is {}", result.0 * result.1);

    Ok(())
}
