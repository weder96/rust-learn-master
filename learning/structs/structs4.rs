struct Player {      
    nname: &'static str, // nickname      
    health: i32,      
    level: u8  
}

fn main(){
    struct Kilograms(u32);
    let weight = Kilograms(250);
    let Kilograms(kgm) = weight; // extracting kgm

    println!("weight is {} kilograms", kgm);
    let mut pl1 = Player{ nname: "Weder", health: 73, level: 2 };
    println!("Player {} is at level {}", pl1.nname, pl1.level); 
    pl1.level = 3;
    println!("Player {} is at level {}", pl1.nname, pl1.level);

    // Like tuples, structs can also be destructured in a let binding, for example:
    let Player{ health: ht, nname: nn, .. } = pl1; 
    println!("Player {} has health {}", nn, ht); 

    // Pointers carry out automatic dereferencing when accessing data structure elements,  as follows:    
    let ps = &Player{ nname: "John", health: 95, level: 1 };    
    println!("{} == {}", ps.nname, (*ps).nname);

    
}