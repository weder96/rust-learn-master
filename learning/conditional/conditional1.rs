fn main(){
    let health = 20;
    let active =
    if health >= 50 { 
        true
    } else {      
        false
    };
    println!("Am I active? {}", active);

    let adult = true;
    let age = if adult { "+18" } else { "-18" };
    println!("Age is {}", age);  // Age is +18
    println!("{:?}",verbose(10));

}

fn verbose(x: i32) -> &'static str {  
    let mut result: &'static str;  
    if x < 10 {    
        result = "less than 10";  
    } else {    
        result = "10 or more";  
    } 
    return result;
}