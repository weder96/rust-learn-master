fn main(){
    let f2 = 3.14;  
    let saved_points = f2 as u32; // truncation to value 3 occurs here
    println!("{}", saved_points);

    let mag = "Gandalf";  
    
    // error: non-scalar cast:`&str`as`u32`
    //let saved_points2 = mag as u32; 
}