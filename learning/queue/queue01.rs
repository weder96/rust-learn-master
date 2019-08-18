//! import collections
use std::collections::VecDeque;

fn main(){
    // queue define capacity
    let vector: VecDeque<u32> = VecDeque::with_capacity(10);
    println!("{:?}", vector);

    let mut buf = VecDeque::new();
    buf.push_back(3);
    buf.push_back(4);
    buf.push_back(5);
    assert_eq!(buf.get(1), Some(&4));
    println!("{:?}", buf.get(1));

    // change position 3 0 with position 2 5 
    buf.swap(0, 2);
    assert_eq!(buf, [5, 4, 3]);  


    if let Some(elem) = buf.get_mut(1) {
        *elem = 7;
    }
    assert_eq!(buf[1], 7);
    println!("{:?}", buf);
    capacity_check();
}

pub fn capacity_check() {
    let buf2: VecDeque<i32> = VecDeque::with_capacity(10);
    println!("capacity: {:?}", buf2.capacity());
    assert!(buf2.capacity() >= 10);
}
