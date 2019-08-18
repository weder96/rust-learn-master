fn main() {
    let magician1 = "Merlin";  
    let greeting = "Hello, 世界!";
    let mut str1 = String::new();
    let mut str2 = String::with_capacity(25);
    let mut str3 = magician1.to_string();
    let sl1 = &str3;
    if &str3 == magician1 {    println!("We got the same magician alright!")  }

    let c1 = 'q';  
    // character c1
    str1.push(c1);
    println!("{}", str1); 
    
    // q
    str1.push_str(" Level 1 is finished - ");
    println!("{}", str1); 
    // q Level 1 is finished - 
    str1.push_str("Rise up to Level 2");
    println!("{}", str1); 
    
    // q Level 1 is finished - Rise up to Level 2
    for c in magician1.chars() {    print!("{} - ", c); }
    // Which prints out: M - e - r - l - i - n -.

    println!(" ");
    for word in str1.split(" ") {     print!("{} / ", word); }

    let str5 = str1.replace("Level", "Floor");

    println!("Length of str1: {}", how_long(&str1));
    println!("Length of str1: {}", how_long(&str1[..]));

}

 fn how_long(s: &str) -> usize { 
     s.len()
}

/*String
mutable – heap memory allocation
module: std::string


String slice (&str)
fixed size – view on String – reference(&)
module: std::str
*/
