enum Operation {
  Add(f64, f64),
  Subtract(f64, f64),
  Multiply(f64, f64),
  Divide(f64, f64),
}

fn main() {
    println!("Enter the expression (e.g 2.0 + 3.0):");
    let mut expr = String::new();

    std::io::stdin()
      .read_line(&mut expr)
      .expect("Failed to read line!");

    let expr_arr: Vec<&str> = expr.split(" ").collect();

    let a: f64 = expr_arr[0].trim().parse().expect("Failed to convert to f64");
    let op = expr_arr[1];
    let b: f64 = expr_arr[2].trim().parse().expect("Failed to convert to f64");

    match op {
      "+" => println!("The result of {} + {} is {}", a, b, calculate(Operation::Add(a, b))),
      "-" => println!("The result of {} - {} is {}", a, b, calculate(Operation::Subtract(a, b))),
      "*" => println!("The result of {} * {} is {}", a, b, calculate(Operation::Multiply(a, b))),
      "/" => println!("The result of {} / {} is {}", a, b, calculate(Operation::Divide(a, b))),
      _ => panic!("Invalid symbol!")
    }
}

fn calculate(operation: Operation) -> f64 {
  match operation {
    Operation::Add(a, b) => a + b,
    Operation::Subtract(a, b) => a - b,
    Operation::Multiply(a, b) => a * b,
    Operation::Divide(a, b) => a / b,
  }
}
