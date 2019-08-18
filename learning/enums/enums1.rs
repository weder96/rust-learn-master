#[derive(Debug)]
enum Compass {  North, South, East, West}
//And then use it as shown in main() or another function:let martian = PlanetaryMonster::MarsMonster("Chela", 42);

// The enum's values can also be of other types or structs, as in this example:
type Species = &'static str;

enum PlanetaryMonster {  
    VenusMonster(Species, i32),  
    MarsMonster(Species, i32)
}

use PlanetaryMonster::MarsMonster;
//Then, the type can be shortened, as follows:

fn main(){
    let direction = Compass::West;
    let martian = PlanetaryMonster::MarsMonster("Chela", 42);

    println!(direction);

    let martian2 = MarsMonster("Chela", 42);

    println!("{:?}", martian2);

}