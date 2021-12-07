#![feature(control_flow_enum)]

use std::error;
use std::io;
use std::io::BufRead;
use std::ops::ControlFlow;

fn main() -> Result<(), Box<dyn error::Error>> {
    let numbers = io::stdin()
        .lock()
        .lines()
        .collect::<Result<Vec<_>, _>>()?
        .into_iter()
        .map(|x| x.trim().to_owned())
        .collect::<Vec<_>>();

    let word_length = numbers[0].len();
    let oxygen = get_rating(numbers.clone(), word_length, true);
    let co2 = get_rating(numbers, word_length, false);
    println!("Total is {}", oxygen * co2);

    Ok(())
}

fn get_rating(numbers: Vec<String>, word_length: usize, is_oxygen: bool) -> u32 {
    u32::from_str_radix(
        &(0..word_length)
            .try_fold(numbers, |numbers, position| {
                let most_common = get_character_for_criteria(numbers.clone(), position, is_oxygen);
                let numbers: Vec<_> = numbers
                    .iter()
                    .filter(|&x| x.chars().nth(position) == Some(most_common))
                    .map(|x| x.clone())
                    .collect();
                if numbers.len() <= 1 {
                    return ControlFlow::Break(numbers);
                }
                ControlFlow::Continue(numbers)
            })
            .break_value()
            .unwrap()[0],
        2,
    )
    .unwrap()
}

fn get_character_for_criteria(numbers: Vec<String>, position: usize, get_common: bool) -> char {
    if numbers.iter().fold(0, |acc, number| {
        acc + match number.chars().nth(position) {
            Some('1') => 1,
            Some('0') => -1,
            _ => 0,
        }
    }) >= 0
    {
        if get_common {
            '1'
        } else {
            '0'
        }
    } else {
        if get_common {
            '0'
        } else {
            '1'
        }
    }
}
