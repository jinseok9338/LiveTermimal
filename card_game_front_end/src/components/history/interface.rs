use chrono::DateTime;
use stdweb::web::Date;

#[derive(Debug, PartialEq, Clone)]
pub struct History {
    pub id: usize,
    pub date: DateTime<chrono::Utc>,
    pub command: String,
    pub output: String,
}
