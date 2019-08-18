fn main() {
    show();
    lenghtArray();
    viewLocate();
    defaulArray();
}

fn show() {
    let mut shopping_list = [1,3,4,7,8,12];
    println!("{:?}",shopping_list);
    shopping_list[2] = 500;
    println!("{:?}",shopping_list);
}

fn lenghtArray() {
   let z = [1,2,3];
    println!("{:?}",z.len());
}

fn viewLocate() {
 let m_array = [55,66,27];
    println!("{:?}",m_array.len());
    println!("{:?}",m_array[0]);
    print!("{:?}",m_array[2]);
}

fn defaulArray(){
    let my_array = [5,5,5,5];
    let my_array_other = [5;4];
    println!("{:?}",my_array);
    println!("{:?}",my_array_other );
} 