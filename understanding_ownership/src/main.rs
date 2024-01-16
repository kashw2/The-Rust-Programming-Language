fn main() {
    let mut string: String = String::from("hello");

    string.push_str(", world!");

    println!("{}", string);

    {
        // New scope so we can create a new instance of string from the original in the parent scope
        let string: &mut String = &mut string;

        string.push_str("\nx2");

        println!("{string}")
    }

    // As string is on the heap, original string is moved to string2
    // so the underlying string data structure now consists of a *char[] with the contents of the string, length and capacity
    let string2: String = string;

    println!("{string2}");
}
