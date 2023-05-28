use std::io::stdin;

mod conversions;
mod graph;
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
        "1" => println!("Binary to decimal."),
        "2" => {
            println!("Binary to hexadecimal.");
            let mut binary_number: Vec<u32> = conversions::get_input();
            let hex_number: Vec<String> = conversions::binary_hex(&mut binary_number);
            println!("Your number in hexadecimal is: ");
            for i in hex_number {
                print!("{i}");
            }
        }
        "3" => println!("decimal to binary."),
        "4" => println!("decimal to hexadecimal."),
        "5" => println!("hexadecimal to binary."),
        "6" => println!("hexadecimal to decimal."),
        &_ => println!("Invalid option."),
    }
}