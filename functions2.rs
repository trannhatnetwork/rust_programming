// TODO: Add the missing type of the argument `num` after the colon `:`.
fn call_me(num: i32) {
    for i in 0..num {
        println!("Ring! Call number {}", i + 1);
    }
}

fn info_student(name: &str, age: i32){
    println!("My name is {name} and I am {age} years old.");
}

fn main() {
    call_me(3);
    info_student("Nhat", 20);
}
