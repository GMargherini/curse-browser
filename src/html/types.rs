
pub enum Type {
    A,
    P,
    SPAN
}

pub struct Node {
    pub content: String,
    pub t: Type,
    pub text: String
}