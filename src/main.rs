fn main() {
    //Variables
    // let a = 10;
    // let b = 20;
    // println!("Hello, world!,{} {}",a,b);

    // let unsigned = 10;
    // println!("unsigned:{}",unsigned);

    // let signed = -600;
    // println!("signed:{}",signed);

    // let float = 1.2;
    // println!("float:{}", float);

    // let letter: char  = 'c';
    // println!("letter:{}",letter);

    // let is_true : bool = true && false;
    // println!("is_true:{}",is_true)


//Arrays
// let discord: [&str;3] = ["shawn","key","marcus"];
// println!("All Members:{:?}",discord);
// println!("QWIK:{}",discord[2]);
// println!("Discord Length:{}",discord.len());

//Tuples
// let tuple:(&str,u8,bool) = ("Shawn",24,true);
// // println!("Shawn:{:?}",tuple);
// // println!("Name:{}, Age:{}, Bool:{}",tuple.0,tuple.1,tuple.2);

// //Destructuring tuple
// let (a,b,c) = tuple;
// println!("A:{} ,B:{}, C:{}",a,b,c);

// println!("{}",is_even(2));

// pub fn is_even(num:u8) -> bool {
// let digit:u8 = num % 2;
// digit == 0
// }

println!("{}",add(1,2));
println!("{}",sub(10, 5));
println!("{}",multiply(2,2));
println!("{}",divide(4,2));

pub fn add(num1:u8,num2:u8) -> u8 {
num1 + num2
}

pub fn sub(num1:u8,num2:u8) -> u8 {
num1 - num2
}

pub fn multiply(num1:u8,num2:u8) -> u8{
    num1 * num2
}

pub fn divide(num1:u8,num2:u8) -> u8{
    num1 / num2

}
}


//unsigned = positive num