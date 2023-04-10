use std::env::{args, Args};

fn main() {
    let mut args: Args = args();

    // for arg in args {
    //     println!("{}", arg);
    // }

    let first: String = args.nth(1).unwrap();

    let operator: String = args.nth(0).unwrap();

    let second: String = args.nth(0).unwrap();

    let first_number = first.parse::<i32>().unwrap();
    let second_number = second.parse::<i32>().unwrap();

    println!("{} {} {}", first, operator, second);

    println!(
        "Result {}",
        calculate(first_number, operator, second_number)
    );
}

fn calculate(first_number: i32, operator: String, second_number: i32) -> i32 {
    if operator == "+" {
        return first_number + second_number;
    } else if operator == "-" {
        return first_number - second_number;
    } else if operator == "*" {
        return first_number * second_number;
    } else if operator == "/" {
        return first_number / second_number;
    } else {
        panic!("Unknown operator");
    }
}
