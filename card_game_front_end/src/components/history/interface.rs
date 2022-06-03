use stdweb::web::Date;

pub struct History {
    pub id: usize,
    pub date: Date,
    pub command: String,
    pub output: String,
}

impl Copy for History {}

impl Clone for History {
    fn clone(&self) -> History {
        *self
    }
}
