fn main() {
    // numbers
    let x: i8 = 5; // range 2 ** n: 2 ** 8 - 1 = 127 to -128
    let y: u8 = 107;
    let z: f32 = 1000.01;


    // for i in 0..1000 {
    //     x = x + 100; // > 127
    // }

    println!("x : {}, y: {}, z: {}", x,y,z);


    // Booleans
    let is_male: bool = true;
    let age: i8 = 16;
    let is_above_18: bool = age >= 18;

    if is_male{
        println!("You are a male");
    }else {
        println!("You are not a male");
    }

    if is_male && is_above_18{
        println!("You are a legal adult male");
    }else{
        println!("You are a not legal adult male");
    }

    // Strings

    let str: &str = "Dhairya Shah";
    let str2: String = String::from("Code");
    println!("My name is {} and I like to {}.", str, str2);
    
    let char1: Option<char> = str.chars().nth(0);
    
    // Check safety by rust
    // It will only print if the character exists or the error will show
    // println!("char1: {}", char1.unwrap()); this also works but not recommended
    match char1 {
        Some(c) => println!("{}", c),
        None => println!("No character at index 1000"),
    }
    // // This is invalid
    println!("char1: {}", char1);

}
