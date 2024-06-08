fn main() {
    // conditional
    let num = 2;
    if num % 2 == 0 {
        println!("The number is even.");
    } else {
        println!("The number is odd.");
    }

    // loops
    // loops can be used to iterate over arrays, maps, strings
    for i in 0..10 {
        println!("{}", i);
    }

    let sentence: String = String::from("dhairya");
    let first_word = get_first_word(sentence);
    println!("{}", first_word);
}

fn get_first_word(sentence: String) -> String {
    let mut ans = String::new(); // Equivalent to String::from("")
    for char in sentence.chars() {
        ans.push_str(char.to_string().as_str());
        if char == ' '{
            break;
        }
    }
    return ans;
}
