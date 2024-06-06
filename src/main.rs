/*
Port of "Secret weapon" BASIC game
from "Compute Battlegames" book 1982
for ZX Spectrum and other microcomputers
*/

use std::io;
use rand::Rng;

fn main() {
    clearscreen::clear().expect("Couldn't clear the screen");
    println!("SECRET WEAPON");

    let difficulty: usize = input_difficulty();

    let target_x = rand::thread_rng()
        .gen_range(1..=difficulty) as f64;
    let target_y = rand::thread_rng()
        .gen_range(1..=difficulty) as f64;

    for goes in 1..=difficulty+5 {
        println!("Guesses for X and Y");

        let guess_x = match input_int() {
            Some(value) => value as f64,
            None => {
                println!("Invalid input - Enter an integer");
                continue;
            }
        };
        let guess_y = match input_int() {
            Some(value) => value as f64,
            None => {
                println!("Invalid input - Enter an integer");
                continue;
            }
        };

        let distance = f64::sqrt(
            (target_x - guess_x).powi(2) + (target_y - guess_y).powi(2)
        );

        if distance == 0.0 {
            println!("You destroyed it in {} goes", goes);
            return;
        }
        else if distance <= 3.0 {
            println!("Close");
        }
        else {
            println!("Not even close");
        }
    }

    println!("The robots have seen\nyou - agghhhhh.....");
}

fn input_difficulty() -> usize {
    loop {
        println!("Enter difficulty");
        match input_int() {
            Some(d) if d >= 4 => return d,
            _ => {
                println!("Enter a difficulty of almost 4");
            }
        }
    }
}

fn input_int() -> Option<usize> {
    let mut value = String::new();

    io::stdin()
        .read_line(&mut value)
        .expect("Can't read input.");

    match value.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None
    }
}
