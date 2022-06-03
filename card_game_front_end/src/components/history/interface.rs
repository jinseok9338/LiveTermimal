use stdweb::web::Date;

#[derive(Debug, PartialEq, Clone)]
pub struct History {
    pub id: usize,
    pub date: Date,
    pub command: String,
    pub output: String,
}
