fn main() {
    let location = "Middle-Earth";
    let part = &location[7..12];
    println!("{}", part); // Earth

    let magician = "Merlin";
    let mut chars: Vec<char> = magician.chars().collect();
    chars.sort();
    for c in chars.iter() {      
        print!("{} ", c);
    }

    let v: Vec<&str> = "The wizard of Oz".split(' ').collect();
    let v: Vec<&str> = "abc1def2ghi".split(|c: char| c.is_numeric()).collect();

    
}