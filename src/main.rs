use std::net;

fn main() {

    // floats 
    // integers
    //  string 
    // Characters

    // Signed and unsigned Integers
    // const number1: i8 = 3;
    // const number2: u8 = 20;

    // const number3: i16 = 12;
    // const number4: u16 = 13;

    // const number5: i32 = 14;
    // const number6: u32 = 230;

    // const number7: i64 = 12;
    // const number8: u64 = 13;

    // const number9: i128 = 14;
    // const number10: u128 = 230;

    // const number11: isize = 12;
    // const number12: usize = 13;

    // const my_age: &str = "Welcome to programming ";

    // let text2:String  = String::from("Hello, World");
    
    // const text3: char = 'A';

    // println!("My Age {}", my_age);
    // println!("Hello, {}", text2);

    // sum(2, 3);
    // println!("Hello, ...... {}", number1);


    // annotate
    // let a:i32 = 323;
    // let b = a;

    // println!("number {}", a);

    // string
    // let c = String::from("Hello");
    // hello(c);
    // println!("print {}", c);

    // greater
    // equal to
    // less than

    let a = 31;
    let b = 4;

    if a > b {
        println!("{} is greater than {}", a, b);
    } else {
        println!("block executed ");
    }

    if a == b {
        println!("{} is equal to {}", a, b);
    }

    if a < b {
        println!("{} is less than {}", a, b);
    }

    if a > b {
        print!("a is greater than b");

    } else if a == b {
        print!("a is equal to b");
    } else {
        print!("a is less than b");
    }

    // Code gets input from

    let network_response = 200;

    let is_success = if network_response == 200 {true} else {false};

    if is_success {
        println!("The request was successful");
    }


    let mut counter = 0;

    loop {
        counter += 1;
        println!("Counter: {}", counter);

        if counter == 1 {
            println!("Starting the loop");
            continue;
        }

        if counter == 10 {
            break;
        }
    }

    let mut number = 3;

    while number != 0 {
        println!("{}!", number);
        number -= 1;
        
    }

    println!("LIFTOFF!!!");


}

// fn sum(a: i32, b: i32){
//     let c = a + b;
//     println!("The sum of {} and {} is {}", a, b, c);
// }

// fn hello (param_1:String) -> String{
//     return param_1;
// }