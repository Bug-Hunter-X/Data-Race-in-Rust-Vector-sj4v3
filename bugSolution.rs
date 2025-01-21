use std::sync::{Arc, Mutex};

fn main() {
    let vec = Arc::new(Mutex::new(Vec::new()));
    vec.lock().unwrap().push(1);
    vec.lock().unwrap().push(2);
    let x = vec.lock().unwrap()[0];
    vec.lock().unwrap().push(3);
    println!("{:?}", x);
}