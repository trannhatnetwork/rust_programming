fn main() {
    // You can optionally experiment here.
    indexing_tuple();
}

fn indexing_tuple() {
    let numbers = (1, 2, 3);
    let second = numbers.1; 
    println!("This is not the 2nd number in the tuple: {}", second);
}

#[cfg(test)]
mod tests {
    #[test]
    fn indexing_tuple() {
        let numbers = (1, 2, 3);

        // TODO: Use a tuple index to access the second element of `numbers`
        // and assign it to a variable called `second`.
        // let second = ???;

        assert_eq!(second, 2, "This is not the 2nd number in the tuple!");
    }
}
