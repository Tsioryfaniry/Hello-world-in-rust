// use std::io;
// use rand::Rng;
// use std::cmp::Ordering;
// mod guesstosecret;

fn main() { 
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("{} et {}",s2,s1);
        // let s = String::from("hello");  // s comes into scope
        // takes_ownership(s); // s's value moves into the function...
        // // ... and so is no longer valid here
        // let x = 5; // x comes into scope
        // makes_copy(x); // x would move into the function,
        // // but i32 is Copy, so it's okay to
        // // still use x afterward
        // } // Here, x goes out of scope, then s. But because s's value was moved,
        // // nothing special happens.
        // fn takes_ownership(some_string: String) { // some_string comes into scope
        // println!("{}", some_string);
        // } // Here, some_string goes out of scope and `drop` is called. The backing
        // // memory is freed.
        // fn makes_copy(some_integer: i32) { // some_integer comes into scope
        // println!("{}", some_integer);
        // } // Here, some_integer goes out of scope. Nothing special happens.
            // {
            //         let s1 = String::from("hello"); 
            //         let s2 = s1.clone();
            //     println!("{s1}");
            //     println!("clone: {s2}");
            // }
            // let s = "Hello!";
            // let ss = s;
            // println!("{}, {}",s,ss);

            // let array = [10,20,30,40, 50];
    // for element in array.iter().rev(){
    //     println!("The value is:{}", element)
    // }

    // for element in array.iter(){
    //     println!("The value is:{}", element)
    // }
    // let mut index = 0;

//    while index < 5 {
//        println!("The value is: {}", array[index]);
//        index = index + 1;
//    }
//    let x = {
//         let y = 3;
//         y + 2
//    };
//    println!("La valeur {}", x);
//    let mut counter = 0;

//    let result = loop{
//     counter +=1;

//     if counter == 10 {
//         break counter ;
//     }  
//     println!("{}", counter)      
// };
// println!("The result is: {}", result);
// let mut number = 3;

// while number != 0 {
//     println!("{}", number);
//     number -=1;
// }
//  println!("LIFTOFF!!!")
//argument in println!
// let x = 5_888_999;
// let y = 10_7667_788;
// println!("x = {} and y = {}", x, y);

//import d'une fonction random
// guesstosecret::types();
// let secret_number = rand::thread_rng().gen_range(1, 100);

// loop {
//     println!("Please input your guess");
// let mut guess = String::new();
// io::stdin().read_line(&mut guess)
// .expect("Failed to read line");

// //trim(): remove space start and end
// //u32 Contain only numerical characters
// let guess:u32 = match guess.trim().parse(){
//     Ok(num) => num,
//     Err(_) => continue,
// };
// println!("You guessed:{}", guess);
// // .expect("Type a number");


//     match guess.cmp(&secret_number){
//     Ordering::Less => println!("To small"),
//     Ordering::Greater => println!("To big"),
//     Ordering::Equal => {
//         println!("You win!");
//         break;
//     },

//     }

// }
//Les opérateur logic
// let x= 5;//statement
// let y= {
//     let x = 3;
//     x + 1
// };
// println!("The value of y is: {:?}", y);
// let condition:bool = x > 0;
// if x > 0 { println!("Le nombre est positif"); }
// println!("{}", condition); 
// another_function(5);
// for x in 1..34{
//     println!("{}",x)
// }
}

// fn another_function(x:u8){
//     println!("The value of x:{}",x);
// }