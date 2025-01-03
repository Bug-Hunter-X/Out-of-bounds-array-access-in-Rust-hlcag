fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 10;

    // Safe way to access the element, returns an Option
    match vec.get(index) {
        Some(value) => println!("Value at index {} is: {}", index, value),
        None => println!("Index {} is out of bounds", index),
    }

    //Alternative approach: check bounds first
    if index < vec.len() {
        let value = vec[index];
        println!("Value at index {} is: {}", index, value);
    } else {
        println!("Index {} is out of bounds", index);
    }
} 