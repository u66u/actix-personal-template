use crate::schema::users;
use crate::schema::wordpairs;
use diesel::prelude::*;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::wordpairs)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct WordPair {
    pub id: i32,
    pub spanish_word: String,
    pub english_word: String,
}

#[derive(Insertable)]
#[diesel(table_name = wordpairs)]
pub struct NewWordPair<'a> {
    pub spanish_word: &'a str,
    pub english_word: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String, // This is really a serialized version of the Argon2 PasswordHash struct
                          // which contains information about the algorithm, salt, and hash.
}

#[derive(Insertable)]
#[diesel(table_name = users)]
pub struct NewUser<'a> {
    pub email: &'a str,
    pub password: &'a str,
}
