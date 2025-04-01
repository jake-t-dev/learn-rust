fn main() {
    loop {
        println!("Simple Calculator");
        println!("1. Add");
        println!("2. Subtract");
        println!("3. Multiply");
        println!("4. Divide");
        println!("5. Exit");
    }

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read line");
    let choice: u32 = choice.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            continue;
        }
    };

    if choice == 5 {
        println!("Exiting...");
        break;
    }

    println!("Enter first number:");
    let mut firstNum = String::new();
    io::stdin().read_line(&mut firstNum).expect("Failed to read line");
    let num1: f64 = firstNum.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a number.");
            continue;
        }
    };

    println!("Enter second number:");
    let mut secondNum = String::new();
    io::stdin().read_line(&mut secondNum).expect("Failed to read line");
    let num2: f64 = secondNum.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Invalid input, please enter a number.");
            continue;
        }
    };

    

}