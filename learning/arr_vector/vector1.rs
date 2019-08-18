fn main(){
    let mut numbers: Vec<i32> = Vec::new();
    let mut magic_numbers = vec![7i32, 42, 47, 45, 54];
    let mut ids: Vec<i32> = Vec::with_capacity(25);

    // A vector can also be constructed from an iterator through the collect() 
    //method with a range, such as in this example:
    let rgvec: Vec<u32> = (0..7).collect();
    println!("Collected the range into: {:?}", rgvec); 

    let values = vec![1, 2, 3];
    for n in values {      
        println!("{}", n);
    }

    numbers.push(magic_numbers[1]);
    numbers.push(magic_numbers[4]);
    println!("{:?}", numbers); // [42, 54]
    let fifty_four = numbers.pop();// fifty_four now contains 54
    println!("{:?}", numbers); // [42]

    //slices
    println!("{:?}", &magic_numbers);
    let slc = &magic_numbers[1..4]; // only the items 42, 47 and 45
    println!("{:?}", slc);

    
}