use std::io::stdin;

pub fn run() {
    let mut buf = String::new();
    let mut numbers: Vec<u32> = Vec::new();

    println!("Please enter limit:");
    if let Ok(_) = stdin().read_line(&mut buf) {
        if let Ok(val) = buf.trim().parse::<u32>() {
            if val < 2 {
                panic!("Limit is too low");
            }
            for i in 2..=val {
                let mut is_prime = true;
                for num in &numbers {
                    if *num > i / 2 {
                        break;
                    }
                    if i % num == 0 {
                        is_prime = false
                    }
                }
                if is_prime {
                    numbers.push(i);
                }
            }
        }
    }
    println!("Primes: {:?}", numbers);
}
