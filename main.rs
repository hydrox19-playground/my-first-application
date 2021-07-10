#[path = "functions/numbers.rs"] mod numbers;
use std::io;

fn main() {
  let mut count = 0;
  let mut numbers: [u32; 2] = [0, 0];

  println!("Enter two numbers to add");

  loop {
    if count == 2 {
      break;
    }
    
    let mut n = String::new();
    
    io::stdin()
      .read_line(&mut n)
      .expect("failed to read input.");
    
    let n = n.trim().parse::<u32>().expect("invalid input");

    numbers[count] = n;
    
    count += 1;
  }

  let number = numbers::add_numbers(numbers[0], numbers[1]);

  println!("The sum of {} + {} is {}", numbers[0], numbers[1], number);
}

/*
use std::io::stdin;

fn main() {

  let mut input_value = String::new();
  stdin().read_line(&mut input_string).ok().expect("Failed to read line");
  
  let

  let number = add_numbers::run(32, 23);
  println!("The number is {}", number);
}
*/