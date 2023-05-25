mod conversions;
mod graph;
fn main() {
    let mut binary_number: Vec<u32> = conversions::get_input();
    let hex_number: Vec<String> = conversions::binary_hex(&mut binary_number);
    println!("Your number in hexadecimal is: ");
    for i in hex_number {
        print!("{i}");
    }
}