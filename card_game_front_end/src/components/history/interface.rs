use instant::Instant;

#[derive(Debug, PartialEq, Clone,Eq)]
pub struct History {
    pub id: Box<usize>,
    pub date: Box<Instant>,
    pub command: String,
    pub output: String,
}
