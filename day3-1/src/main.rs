use std::error;
use std::io;
use std::io::BufRead;

fn main() -> Result<(), Box<dyn error::Error>> {
    let result = io::stdin()
        .lock()
        .lines()
        .fold(None, |acc, number| {
            if let Ok(number) = number {
                let number = number.trim();
                let mut acc = match acc {
                    None => vec![0; number.len()],
                    Some(acc) => acc,
                };
                for (i, v) in number.chars().enumerate() {
                    acc[i] += match v {
                        '1' => 1,
                        '0' => -1,
                        _ => 0,
                    }
                }
                Some(acc)
            } else {
                acc
            }
        })
        .unwrap()
        .iter()
        .map(|&x| if x > 0 { '1' } else { '0' })
        .collect::<String>();

    let gamma = u32::from_str_radix(&result, 2)?;
    let epsilon = !gamma & (1 << result.len()) - 1 ;

    println!("Total is {}", gamma * epsilon);

    Ok(())
}
