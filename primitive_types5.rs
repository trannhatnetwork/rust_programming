fn main() {
    let cat: (String, f64) = ("Furry McFurson".to_string(), 3.5);

    let (name, age) = cat;
    
    println!("{name} is {age} years old");
}
