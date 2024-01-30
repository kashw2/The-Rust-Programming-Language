use super::Summary;

pub struct Tweet {
    pub username: String,
    pub content: String,
}

pub struct Article {
    pub name: String,
    pub description: String,
    pub content: String,
    pub author: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}", self.content)
    }
    fn get_full_author_name(&self) -> String { format!("@{}", self.username) }
}

impl Summary for Article {
    fn summarize(&self) -> String { format!("{} - {}", self.description, self.content) }
    fn get_full_author_name(&self) -> String { format!("{}", self.author) }
}

