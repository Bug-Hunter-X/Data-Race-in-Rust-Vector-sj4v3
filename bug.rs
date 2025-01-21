fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let x = &vec[0]; // this is the error
    vec.push(3); // this will cause an error 
    println!("{:?}