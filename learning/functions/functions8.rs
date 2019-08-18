fn increment_power(power: i32) -> i32 {  
    println!("My power is going to increase:");  
    power + 1
}

fn main() {  
    let power = increment_power(1); 
    // function is called  
    println!("My power level is now: {}", power);
    println!("{:?}",abs(5));
}

fn abs(x: i32) -> i32 {  
    if x > 0 {  x  } else {  -x  }
}