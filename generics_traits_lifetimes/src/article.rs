pub mod articles;

pub trait Summary {
    fn summarize(&self) -> String;
    fn get_full_author_name(&self) -> String;
}
