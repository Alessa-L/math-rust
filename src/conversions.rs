#![allow(dead_code)]
use std::{
    collections::HashMap,
    io::{self, Write},
    str::Chars
};

// Binary to decimal: Vec[digit * 2.pow(n)]
// bit shift could also be used in this instance: Vec[digit << n]

pub fn get_input() -> Vec<u32> {
    let mut user_input: String = String::new();
    print!("Please input a number: ");
    io::stdout().flush().unwrap(); // The stdout needs to be flushed to ensure subsequent output is immediate.
    match io::stdin().read_line(&mut user_input) {
        Ok(_) => { user_input.pop(); }
        Err(error) => {
            eprintln!("Error: {error}");
            println!("Let's try this again.");
            return get_input()
        }
    }

    let user_input: Chars = user_input.chars(); // Create an iterator of characters out of the string.
    let mut binary_number: Vec<u32> = Vec::new();
    for digit in user_input {
        if digit.is_whitespace() {
            continue
        }
        match digit.to_digit(2) { // Try to transform the character into a integer digit of the specified radix.
            Some(digit) => { binary_number.push(digit) }
            None => {
                eprintln!("Invalid number!");
                break
            }
        }
    }

    return binary_number
}

pub fn parse_input(char_digit: char, radix: u32) -> usize { 
    // TODO: I'll try to replicate the functionality of char::to_digit.
    let digit: usize = 0;
    return digit
}

pub fn binary_hex(binary_number: &mut Vec<u32>) -> Vec<String> {
    // While the array isn't divisible by 4, keep updating it with more leading zeroes.
    while binary_number.len() % 4 != 0 {
        binary_number.push(0);
    }
    binary_number.reverse();
    
    let mut hex_calc: Vec<String> = Vec::new();
    for digit in 0 .. ( binary_number.len() / 4 ) {
        let mut calc: u32 = 0;
        for pos_shift in 0 ..= 3 {
            let binary_radix: u32 = 2; // Need to set this in order to perform exponentiation in a more readable format.
            // try_into() used in order to convert usize to u32.
            calc += binary_number[ (digit * 4) + pos_shift ] * binary_radix.pow(pos_shift.try_into().unwrap()); 
        }
        hex_calc.push(calc.to_string());
    }
    hex_calc.reverse();

    let hex_digits: HashMap<usize, &str> = HashMap::from(
        [(10, "A"), 
         (11, "B"), 
         (12, "C"), 
         (13, "D"), 
         (14, "E"), 
         (15, "F")]
    );

    for (i, digit) in hex_calc.clone().into_iter().enumerate() {
        let digit: usize = digit.parse().unwrap();
        if digit > 9 {
            hex_calc[i] = hex_digits[&digit].try_into().unwrap();
        }
        else {
            hex_calc[i] = digit.to_string();
        }
    }

    return hex_calc
}