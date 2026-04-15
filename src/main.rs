use std::io;

fn input_and_validate_number_input() -> i32 {

    loop {

        let mut number = String::new();
        io::stdin()
                .read_line(&mut number)
                .expect("Failed to read line");
    
        match number.trim().parse::<i32>() {
            Ok(num) => {
                return num;
            },
            Err(_) => {
                println!("Please type a number!");
                continue;
            }
        };

     }
  

}

fn input_and_validate_operator() -> String {
    loop{
        let mut operator = String::new();
        io::stdin()
            .read_line(&mut operator)
            .expect("Failed to read line");

        match operator.trim() {
          "+"=>{ return "Add".to_string();},
          "-"=>{return "Substract".to_string();},
          "*"=>{return "Multiply".to_string()},
          "/"=>{return "Divide".to_string()},
          _=>{println!("Please Enter a valid operator from the list!");}
        }
    }
    
}
fn main() {

    println!("This Is a simple CLI calculator, input 2 numbers and an operator: ");


    println!("Enter the first number: ");
    let arg1 = input_and_validate_number_input();
    

    println!("Enter operator: ");
    let operator = input_and_validate_operator();
    

    println!("Enter the second number: ");

    let arg2 = input_and_validate_number_input();

    

    match operator.as_str(){
      "Add"=>println!("{} + {} = {}",arg1, arg2, arg1 + arg2),
      "Substract"=>println!("{} - {} = {}", arg1, arg2, arg1 - arg2),
      "Multiply"=>println!("{} * {} = {}", arg1, arg2, arg1 * arg2),
      "Divide" => {
            if arg2 == 0 {
                println!("Cannot divide by zero!");
            } else {
                println!("{} / {} = {}", arg1, arg2, arg1 / arg2);
            }
        },
      _ => unreachable!()
     }


    
}