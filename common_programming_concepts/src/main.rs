fn main() {
    // Chapter 3.1
    let mut x: i32 = 10;
    println!("The value of x is {x}");
    x = 20;
    println!("The value of x is now {x}");

    {
        let x: i32 = 5;
        println!("The value of x in the inner scope is {x}");
    }

    println!("The final value of x is {x}");

    // Chapter 3.2
    let _u8: u8 = 255;
    let _i8: i8 = -128;
    let _u16: u16 = 65535;
    let _i32: i32 = 999999999;
    let _f64: f64 = 9.51923;

    let tuple: (u8, f32, u16) = (123, 42.42, 9001);

    let (_a, _b, _c) = tuple;

    let _one_two_three = tuple.0;
    let _meaning_of_life = tuple.1;
    let _over_9000 = tuple.2;

    let _days: [&str; 7] = ["Monday", "Tuesday", "Wednesday", "Thursday", "Friday", "Saturday", "Sunday"];
    let _monday: &str = _days[0];

    // Chapter 3.3
    another_function(5);
    println!("{}", five());

    // Chapter 3.5
    let number: i32 = 3;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    for number in 0..3 {
        println!("The number is {number}");
    }
}

fn another_function(x: u32) {
    println!("Just another function");
    println!("{x} was passed to the function");
}

fn five() -> u8 {
    5
}
