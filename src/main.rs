use core::num;
use std::io::{self, IoSliceMut};



fn main(){

    println!("Calculator");
    println!("İşlemler:");
    println!("1. Add");
    println!("2. Subtract");
    println!("3. Multiply");
    println!("4. Bölme");
    println!("5. Divide");

loop {
    println!("Enter Transaction :");

    let mut choice = String::new();

    io::stdin()
    .read_line(&mut choice)
    .expect("read error");

    let choice:i32= match choice.trim().parse() {
        Ok(num)=>num,
        Err(_)=>{
            println!("invalid");
            continue;
        }
        
    };

    if choice < 1 || choice > 5 {
        println!("invalid transaction");
        continue;
    }

    if choice==5{
        println!("exiting....");
        break;
    }


    println!("enter first number :");

    let mut num1=String::new();

    io::stdin()
    .read_line(&mut num1)
    .expect("number not entered");

   let num1:f64= match num1.trim().parse() {

    Ok(num)=>num,
    Err(_)=>{
        println!("invalid");
        continue;
    }
       
   };


   println!("enter second number :");

   let mut num2=String::new();

   io::stdin()
   .read_line(&mut num2)
   .expect("number not entered");

   let num2:f64=match num2.trim().parse() {

    Ok(num)=>num,
    Err(_)=>{
        println!("invalid");
        continue;
    }
       
   };

   match choice {

    1 => {
        let result=add(num1,num2);
        println!("Result : {}", result);
    }


     2 => {
        let result=subtract(num1,num2);
        println!("Result : {}", result);
    }


     3 => {
        let result=multiply(num1,num2);
        println!("Result: {}", result);
    }


    4 => {
                if num2 == 0.0 {
                    println!("division by zero error!");
                } else {
                    let result = divide(num1, num2);
                    println!("Result: {}", result);
                }
            }

    _=>{
        println!("invalid");
        continue;
    }

       
   }
}

}


fn add(a:f64, b:f64)->f64{

    a+b

}


fn subtract(a:f64,b:f64)->f64{
    a-b
}

fn multiply(a:f64, b:f64)->f64{

    a*b

}

fn divide(a:f64, b:f64)->f64{
    a/b
}







