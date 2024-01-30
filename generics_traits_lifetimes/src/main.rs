pub mod article;

use std::fmt::{Display, Formatter, Pointer};
use article::articles::{Article, Tweet};
use article::Summary;

fn main() {
    let vec_1: Vec<i32> = Vec::from([1, 10, 25, 20, 19, 27, 100, 40, 33, 81, 55, 18, 21, 42]);
    let vec_2: Vec<char> = Vec::from(['a', 'b', 'd', 'z', 'y']);

    println!("The largest number is {}", find_largest(&vec_1).unwrap_or(&-1));
    println!("The largest char is {}", find_largest(&vec_2).unwrap_or(&'a'));
    println!("The largest char using LargestFinder is {}", vec_2.find().unwrap_or(&'a'));

    let tweet: Tweet = Tweet { username: String::from("Johnny"), content: String::from("Hello World") };
    let article: Article = Article { name: String::from("Hello World"), description: String::from("An article about Hello World"), content: String::from("Hello World, this is me"), author: String::from("John Doe") };

    println!("Tweet summary: {}", tweet.summarize());
    println!("Article summary: {}", article.summarize());

    println!("Tweet author: {}", tweet.get_full_author_name());
    println!("Article author: {}", article.get_full_author_name());

    println!("Article author == Tweet author: {}", is_same_author(&tweet, &article));

    let short_string: String = String::from("Short string");
    let long_string: String = String::from("This is a long string");

    println!("Longest string: {}", String::longest(&short_string, &long_string));
}

fn find_largest_number(vec: &Vec<i32>) -> Option<&i32> {
    let mut largest: Option<&i32> = None;

    for i in vec {
        if i > &largest.unwrap_or(&0) {
            largest = Some(i);
        };
        continue;
    }

    return largest;
}

fn find_largest<T: PartialOrd>(vec: &Vec<T>) -> Option<&T> {
    let mut largest: Option<&T> = None;

    match largest {
        Some(current) => for i in vec {
            if i > current {
                largest = Some(i)
            }
        }
        _ => {
            largest = vec.get(0);
            for i in vec {
                if i > largest.unwrap() {
                    largest = Some(i)
                }
            }
        }
    }

    return largest;
}

trait LargestFinder<T: PartialOrd> {
    fn find(&self) -> Option<&T>;
}

impl LargestFinder<char> for Vec<char> {
    fn find(&self) -> Option<&char> {
        if self.capacity() < 1 {
            return None;
        }
        let mut largest: Option<&char> = self.get(0);
        for i in self {
            if i > largest.unwrap() {
                largest = Some(i)
            }
        }
        return largest;
    }
}

fn find_author(item: &impl Summary) -> String {
    item.get_full_author_name()
}

impl Display for Tweet {
    // Not actually required, just performed to make the where A: Summary + Display work
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        return f.pad(&self.content);
    }
}

fn is_same_author<A, B>(item_1: &A, item_2: &B) -> bool
    where A: Summary + Display, B: Summary {
    if item_1.get_full_author_name().contains(&item_2.get_full_author_name()) {
        return true;
    }
    return false;
}

trait LongestString {
    fn longest<'a>(first: &'a str, second: &'a str) -> &'a str;
}

impl LongestString for String {
    fn longest<'a>(first: &'a str, second: &'a str) -> &'a str {
        if first.len() > second.len() {
            return first;
        }
        return second;
    }
}
