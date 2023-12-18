use instant::Instant;

#[derive(Debug, PartialEq, Clone)]
pub struct History {
    pub id: Box<usize>,
    pub date: Box<Instant>,
    pub command: String,
    pub output: String,
}
