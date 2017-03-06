use std::time::Instant;

fn main() {
    println!("Results for problem 1.");
    let (result, time) = time_problem(&problem1a);
    println!("Result for problem 1a is: {}. \nIt took {} ms.", result, time);

    let (result, time) = time_problem(&problem1b);
    println!("Result for problem 1b is: {}. \nIt took {} ms.", result, time);

    println!("Results for problem 2.");
    let (result, time) = time_problem(&problem2a);
    println!("Result for problem 2a is: {}. \nIt took {} ms.", result, time);

    let (result, time) = time_problem(&problem2b);
    println!("Result for problem 2b is: {}. \nIt took {} ms.", result, time);
}

fn time_problem(f: &Fn() -> i32) -> (i32, u64) {
    let start = Instant::now();
    let result = f();
    let elapsed = start.elapsed();
    return (result, elapsed.as_secs() * 1000000 as u64 + (elapsed.subsec_nanos() / 1000) as u64);
}

fn problem1a() -> i32 {
    return problem1a_help(&vec![3, 5], 1000);
}

fn problem1a_help(prime_factors: &Vec<i32>, max: i32) -> i32 {
    let mut accumulator = 0;
    for i in 0..max {
        if prime_factors.iter().any(|p|  i % p == 0) {
            accumulator += i;
        }
    }

    return accumulator;
}

fn problem1b() -> i32 {
    return problem1b_help(&vec![3,5], 1000);
}

fn problem1b_help(prime_factors: &Vec<i32>, max: i32) -> i32 {
    let mut accumulator = 0;
    let mut past_factors = Vec::new();

    for value in prime_factors {
        for i in 1..max {
            if i % value == 0 && past_factors.iter().all(|x| i % x != 0) {
                accumulator += i;
            }
        }

        past_factors.push(*value);
    }

    return accumulator;
}


fn fibonacci (index: i32) -> i32 {
    let result = ((1f64 + 5f64.sqrt()).powi(index) + (1f64 - 5f64.sqrt()).powi(index) / (2f64.powi(index) * 5f64.sqrt())) as i32;
    println!("The {} fibo number is {}", index, result);
    return ((1f64 + 5f64.sqrt()).powi(index) + (1f64 - 5f64.sqrt()).powi(index)/(2f64.powi(index) * 5f64.sqrt())) as i32;
}

fn problem2a() -> i32 {
    let mut index = 0;
    let mut accumulator = 0;
    let mut value = fibonacci(index);

    while value < 4000000 {
        if value % 2 == 0 {
            accumulator += value;
        }
        index += 1;
        value = fibonacci(index);
    }

    return accumulator;
}

fn fib(index: i32) -> i32 {
    if index == 0 {
        return 1;
    }

    if index == 1 {
        return 2;
    }

    return fib(index - 1) + fib(index - 2);
}

fn problem2b() -> i32 {
    let mut i = 0;
    let mut value = fib(i);
    let mut accumulator = 0;

    while value < 4000000 {
        if value % 2 == 0 {
            accumulator += value;
        }

        i += 1;
        value = fib(i)
    }

    return accumulator;
}