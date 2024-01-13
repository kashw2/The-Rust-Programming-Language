fn main() {
    let mut x: i32 = 10;
    println!("The value of x is {x}");
    x = 20;
    println!("The value of x is now {x}");

    {
        let x: i32 = 5;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The final value of x is {x}");
}
