#[derive(Debug)]
struct User {
    name: String,
    age: i8,
    email: String,
}

#[derive(Debug)]
struct Rectangle {
    length: i32,
    width: i32,
}

impl Rectangle {
    fn area(&self) -> i32 { self.length * self.width }
    fn can_hold(&self, other: &Rectangle) -> bool { other.length < self.length && other.width < self.length }
    fn new(length: i32, width: i32) -> Self {
        Self {
            length,
            width,
        }
    }
}

fn main() {
    let user1: User = User {
        name: String::from("Keanu"),
        age: 25,
        email: String::from("keanu.ashwell@email.com"),
    };

    let user2: User = User {
        email: String::from("john.doe@email.com"),
        ..user1
    };

    println!("{}", user2.email);

    #[derive(Debug)]
    struct Color(i16, i16, i16);

    let color1: Color = Color(255, 0, 0);

    println!("{:#?}", user2);

    dbg!(&color1);

    let rectangle1: Rectangle = Rectangle { length: 50, width: 50 };
    let rectangle2: Rectangle = Rectangle::new(20, 10);

    dbg!(rectangle1.can_hold(&rectangle2));

}
