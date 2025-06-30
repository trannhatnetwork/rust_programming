fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    let a = [0; 100]; // An array of 100 elements, all initialized to 0

    println!("Array a: {:?}", a);
    println!("Value of Array at index 10: {}", a[10]);

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
