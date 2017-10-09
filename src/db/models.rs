use db::schema::{authors, quotes};

#[derive(Identifiable, Queryable)]
pub struct Author {
    pub id: String,
}

#[derive(Identifiable, Queryable, Associations)]
#[belongs_to(Author, foreign_key = "created_by_id")]
pub struct Quote {
    /// Autogen `SERIAL PRIMARY KEY`
    pub id: i32,
    /// The discord message snowflake id, `UNIQUE`
    pub message_id: String,
    /// Quote body, with formatting.
    pub quote: String,
    /// Who created the original message, the person who was quoted.
    pub created_by_id: String,
    /// The person who starred the message, the person who *quoted*.
    pub quoted_by_id: String,
}

impl Clone for Quote {
    fn clone(&self) -> Quote {
        Quote {
            id: self.id,
            message_id: self.message_id.to_string(),
            quote: self.quote.to_string(),
            created_by_id: self.created_by_id.to_string(),
            quoted_by_id: self.quoted_by_id.to_string(),
        }
    }
}

#[derive(Insertable)]
#[table_name = "quotes"]
pub struct NewQuote<'a> {
    pub message_id: &'a str,
    pub quote: &'a str,
    pub created_by_id: &'a str,
    pub quoted_by_id: &'a str,
}

#[derive(Insertable)]
#[table_name = "authors"]
pub struct NewAuthor<'a> {
    pub id: &'a str,
}
