use std::{io::{stdin, self, Read}, process::exit};

mod conversions;
mod computer;
mod arithmetic;
fn main() {
    let options: [&str; 6] = [
        "binary to decimal conversions.",
        "binary to hexadecimal conversions.",
        "decimal to binary conversions.",
        "decimal to hexadecimal conversions.",
        "hexadecimal to binary conversions.",
        "hexadecimal to decimal conversions."
    ];

    for (i, option) in options.into_iter().enumerate() {
        let num: usize = i + 1;
        println!("Type {num} to access {option}");
    }

    let mut input: String = String::new();
    match stdin().read_line(&mut input) {
        Ok(_) => { input.pop(); }
        Err(error) => { eprintln!("{error}"); }
    }
    match input.as_str() {
        "1" => {
            println!("Binary to decimal.");
            let mut binary_number: Vec<u8> = vec![];
            binary_number = conversions::get_input(2);
            let dec_number: u32 = conversions::binary_dec(&mut binary_number);
            println!("Your number in hexadecimal is: {}", dec_number);
        }
        "2" => {
            println!("Binary to hexadecimal.");
            let mut binary_number: Vec<u8> = vec![];
            while binary_number.len() == 0 {
                binary_number = conversions::get_input(2);
            }
            let hex_number: Vec<String> = conversions::binary_hex(&mut binary_number);
            println!("Your number in hexadecimal is: ");
            for i in hex_number {
                print!("{i}");
            }
        }
        "3" => {
            println!("decimal to binary.");
            let decimal_vector: Vec<u8> = conversions::get_input(10);
            // We "unpack" the vector with the appropriate decimal place value
            let mut decimal_number: f32  = decimal_vector.iter().fold(0.0, |accum, n| (accum * 10.0) + *n as f32);
            let binary_number: Vec<u8> = conversions::decimal_bin(&mut decimal_number);
            println!("Your number in binary is: ");
            for i in binary_number {
                print!("{}", i);
            }
        }
        "4" => println!("decimal to hexadecimal."),
        "5" => println!("hexadecimal to binary."),
        "6" => println!("hexadecimal to decimal."),
        "7" => {
            computer::bubble_sort();
        }
        "8" => {
            let mut input: String = String::new();
            let mut a: String = String::new();
            let mut b: String = String::new();
            for i in 0 .. 2 {
                match io::stdin().read_line(&mut input) {
                    Ok(_) => { input.pop(); },
                    Err(error) => {
                        eprintln!("Error: {}", error)
                    }
                }
                match i {
                    0 => {
                        a = input.clone();
                        input.clear();
                    },
                    1 => b = input.clone(),
                    _ => {}
                }
            }
            arithmetic::add(a, b);
        }
        &_ => println!("Invalid option.")
    }
    
    exit(0)
}