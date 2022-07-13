// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints

// you can modify anything EXCEPT for this function's signature
fn print_number(maybe_number: u16) {
    println!("printing: {}", maybe_number);
}

fn main() {
    print_number(13);
    print_number(99);

    let mut numbers: [Option<u16>; 5] = [None; 5];
    for iter in 0..5 {
        let number_to_add: u16 = { ((iter * 1235) + 2) / (4 * 16) };

        numbers[iter as usize] = Some(number_to_add);
    }
}
