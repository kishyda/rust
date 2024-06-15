use rand::Rng;
fn take_input(string: &mut String) -> &mut String {
    std::io::stdin()
        .read_line(string)
        .expect("Error reading string");
    string
}

fn string_to_int(string: String) -> i64 {
    let result = string
        .trim()
        .parse()
        .expect("Error parsing string to integer");
    result
}

fn take_int_input() -> i64 {
    let mut n: String = String::new();
    take_input(&mut n);
    let result = string_to_int(n);
    result
}

fn random_number(lower: i64, upper: i64) -> i64 {
    let mut rng = rand::thread_rng();
    let n: i64 = rng.gen_range(lower..upper);
    n
}

fn guessing_game() {
    println!("Enter lower bound");
    let lower_bound = take_int_input();
    println!("Enter upper bound");
    let upper_bound: i64 = take_int_input();
    assert!(lower_bound < upper_bound);
    let random_number = random_number(lower_bound, upper_bound);
    let counter = 0;
    loop {
        println!("Enter guess");
        let guess = take_int_input();
        if counter >= 10 {
            println!("too many guesses. You lost.");
        }
        if guess == random_number {
            println!("Correct! You win");
            break;
        } else if guess > random_number {
            println!("try lower");
        } else if guess < random_number {
            println!("try higher");
        }
    }
}

fn palindrome() {
    println!("Enter your string:");
    let mut string1 = String::new();
    let mut string2 = String::new();

    take_input(&mut string1);
    // Create a separate mutable reference to string1 for character access
    let string_ref = &string1;
    for index in 0..string_ref.len() {
        let temp = string_ref
            .chars()
            .nth(string_ref.len() - index - 1)
            .expect("Error with converting from string to char");

        string2.push(temp);
    }
    string1.pop();
    string2.remove(0);
    println!("{}", string1.chars().nth(3).expect(""));
    println!("{}", string2.chars().nth(3).expect(""));
    if string1 == string2 {
        println!("The string is a palindrome");
    } else {
        println!("The string is not a palindrome");
    }
}

fn for_loop() {
    let mut n = String::new();
    take_input(&mut n);
    let x = string_to_int(n);
    for i in 0..x + 1 {
        println!("{}", i);
    }
}

fn sum() -> i64 {
    let mut n: String = String::new();
    take_input(&mut n);
    let x: i64 = string_to_int(n);
    let mut sum: i64 = 0;
    for index in 0..x + 1 {
        sum += index;
    }
    sum
}
fn factorial() -> i64 {
    let mut n: String = String::new();
    take_input(&mut n);
    let x: i64 = string_to_int(n);
    let mut result: i64 = 1;
    for index in 1..x + 1 {
        result *= index;
    }
    result
}

fn fibonacci() -> i64 {
    let mut n: String = String::new();
    take_input(&mut n);
    let x: i64 = string_to_int(n);
    let mut vector: Vec<i64> = Vec::new();
    vector.push(0);
    vector.push(1);
    for index in 2..x + 1 {
        let next_value: i64 = vector[index as usize - 1] + vector[index as usize - 2];
        vector.push(next_value);
    }
    vector[x as usize]
}
fn main() {
    palindrome();
}
