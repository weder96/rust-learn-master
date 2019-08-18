fn main(){ 
    let mut aliens = ["Cherfer", "Fynock", "Shirack", "Zuxu"]; 
    println!("{:?}", aliens);

    let mut aliens2: [&str; 4] = ["Cherfer", "Fynock", "Shirack", "Zuxu"];
    println!("{:?}", aliens2);

    //If we want to initialize an array with three Zuxus, that's easy too:
    let zuxus = ["Zuxu"; 3];
    //How would you then make an empty array? This is shown as follows:
    let mut empty: [i32; 0] = [];println!("{:?}", empty); 
    // []We can also access individual items with their index, starting from 0:
    println!("The first item is: {}", aliens[0]); // Cherfer
    println!("The third item is: {}", aliens[2]); // Shirack

    let pa = &aliens;
    println!("Third item via pointer: {}", pa[2]);

    aliens[2] = "Facehugger";
    // runtime error:
    //println!("This item does not exist: {}", aliens[10]);  

    for ix in 0..aliens.len() { println!("Alien no {} is {}", ix, aliens[ix]);}

    for a in aliens.iter() { println!("The next alien is {}", a); }
    
    //The for loop can be written even shorter as follows:
    for b in &aliens  { 
        println!("{:?}", b);
    }

} 