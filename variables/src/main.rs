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

    let str = "Dhairya Shah";
    println!("My name is {}", str);

}
