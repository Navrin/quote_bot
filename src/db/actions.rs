use diesel;
use diesel::prelude::*;
use db::models::*;
use rand::{thread_rng, Rng};
use db::schema::*;
use diesel::result::Error as DieselError;

/// Creates a new quote and saves it into the database.
pub fn create_quote(conn: &PgConnection, quote: &NewQuote) -> Result<Quote, diesel::result::Error> {
    find_author(conn, quote.quoted_by_id)?;
    find_author(conn, quote.created_by_id)?;
    diesel::insert(quote).into(quotes::table).get_result(conn)
}

/// Finds an author in the database, if they do not exist, create a new one.
pub fn find_author(conn: &PgConnection, author_id: &str) -> Result<Author, diesel::result::Error> {
    use db::schema::authors;
    use db::schema::authors::dsl::*;

    let author = authors.find(author_id.to_string()).first::<Author>(conn);

    match author {
        Ok(u) => Ok(u),
        Err(ref e) if *e == DieselError::NotFound => diesel::insert(&NewAuthor { id: author_id })
            .into(authors::table)
            .get_result(conn),
        Err(e) => return Err(e),
    }
}

/// Finds a random quote from the user.
pub fn find_rand_quote(conn: &PgConnection, author_id: &str) -> Result<Option<Quote>, DieselError> {
    let mut rng = thread_rng();

    let author = find_author(conn, author_id)?;
    let mut quotes = Quote::belonging_to(&author).load::<Quote>(conn)?;

    let mut quotes = quotes.as_mut_slice();

    rng.shuffle(&mut quotes);

    let first = quotes.first();
    match first {
        Some(q) => Ok(Some(q.clone())),
        None => Ok(None),
    }
}

/// Searches for a max of 5 quotes that contain the given string query.
pub fn find_contains_quotes(
    conn: &PgConnection,
    author_id: &str,
    query: &str,
) -> Result<Vec<Quote>, DieselError> {
    let author = find_author(conn, author_id)?;
    Quote::belonging_to(&author)
        .filter(quotes::quote.eq(format!("%{}%", query)))
        .limit(5)
        .get_results::<Quote>(conn)
}

pub struct ListParams {
    pub amount: Option<i64>,
    /// a **1** indexed page number
    pub page: Option<i64>,
}

pub fn find_listed_quotes(
    conn: &PgConnection,
    author_id: &str,
    params: ListParams,
) -> Result<Vec<Quote>, DieselError> {
    let author = find_author(conn, author_id)?;
    let limit = params.amount.unwrap_or(5);

    Quote::belonging_to(&author)
        .limit(limit)
        .offset((params.page.unwrap_or(1) * limit) - limit)
        .get_results::<Quote>(conn)
}

#[cfg(test)]
mod tests {
    use super::*;
    use db::*;

    #[test]
    fn it_can_make_quotes() {
        let conn = create_connection();

        conn.begin_test_transaction().unwrap();

        let user_1 = "1";
        let user_2 = "2";

        create_quote(
            &conn,
            &NewQuote {
                created_by_id: user_1,
                quoted_by_id: user_2,
                quote: "Hello world!",
                message_id: "12345",
            },
        ).unwrap();

        let quote = find_rand_quote(&conn, user_1).unwrap();

        assert_eq!(quote.unwrap().quote, "Hello world!");
    }

    #[test]
    fn it_can_list_quotes() {
        let conn = create_connection();
        conn.begin_test_transaction().unwrap();

        let user_1 = "1";
        let user_2 = "2";

        let (quote_1, quote_2) = ("Hello world", "How fantastic!");

        create_quote(
            &conn,
            &NewQuote {
                created_by_id: user_1,
                quoted_by_id: user_2,
                quote: quote_1,
                message_id: "12345",
            },
        ).unwrap();

        create_quote(
            &conn,
            &NewQuote {
                created_by_id: user_1,
                quoted_by_id: user_2,
                quote: quote_2,
                message_id: "56789",
            },
        ).unwrap();

        let quotes = find_listed_quotes(
            &conn,
            user_1,
            ListParams {
                amount: Some(2),
                page: Some(1),
            },
        ).unwrap();

        let quotes_flat = quotes
            .iter()
            .map(|q| q.quote.to_string())
            .collect::<Vec<String>>();

        for quote in vec![quote_1, quote_2] {
            assert!(quotes_flat.contains(&quote.to_string()))
        }
    }
}
