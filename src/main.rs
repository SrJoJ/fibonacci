use std::io;

fn main() {
    loop {
        println!("Type number of digit in fibanacci order: ");

        let mut number: String = String::new();

        io::stdin().read_line(&mut number)
            .expect("Failed to read line!");

        let number: u64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                "Type a positive number!";
                continue
            }
        };

        let fibanacci_number: u64 = get_n_fibanacci_number(number);
        println!("{fibanacci_number}")
    };
}

fn get_n_fibanacci_number(n: u64) -> u64 {
    if n > 2 {
        get_n_fibanacci_number(n-2) + get_n_fibanacci_number(n-1)
    } else if n > 1 {
        1
    } else {
        0
    }
}