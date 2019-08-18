struct Score(i32, u8);

fn main() {
    let score1 = Score(73, 2);
    // These are called tuple structs because they resemble tuples very much. 
    //The values contained in them can be extracted as follows:  
    let Score(point, level) = score1; // destructure the tuple  
    println!("Health {} - Level {}", point, level)
}