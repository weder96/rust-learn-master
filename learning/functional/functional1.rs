fn main() {
    func_calculate();
    func_range();
}

pub fn func_calculate() {
println!("I'm going to calculate a sum, hang on a sec");
    let mut total = 0;
    for i in 1 .. 101 {
        total += i;
    }
    println!("The total is {}", total);
}

pub fn func_range() {
    println!("{}", (1..101).fold(0, |x, y| x + y));
}
