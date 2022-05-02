use std::io;

fn main() {
    println!("******************");
    println!("    Rusty Fib");
    println!("******************");

    loop {
        println!("nth fibonacci, please enter n:");

        let mut n = String::new();

        io::stdin()
            .read_line(&mut n)
            .expect("Failed to read line");

        let n: u64 = match n.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        if n == 1 {
            println!("1st fibonacci is 0");
            break;
        }

        let mut fib: u64 = 1;
        let mut last_fib: u64 = 0;


        for _i in 2..n {
            let next_fib = fib + last_fib;
            last_fib = fib;
            fib = next_fib;
        }

        let suffix = get_suffix(n);
        println!("{}{} fibonacci is {}", n, suffix, fib);
        break;
    }
}

fn get_suffix(num: u64) -> &'static str {
    let remainder = num % 10;
    let teen_remainder = num % 100 - remainder;

    if remainder == 1 && teen_remainder != 10 {
        return "st";
    } else if remainder == 2 && teen_remainder != 10 {
        return "nd";
    } else if remainder == 3 && teen_remainder != 10 {
        return "rd";
    } else {
        return "th";
    }
}
