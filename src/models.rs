use crate::schema::posts;

use diesel::{Queryable, Insertable};
use diesel::result::QueryResult;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::error::CustomError;

pub type Posts = Result<Json<Vec<Post>>, CustomError>;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable)]
pub struct Post {
    id: i32,
    title: String,
    sub_title: String
}

pub fn fetch(conn:&mut diesel::PgConnection) -> QueryResult<Vec<Post>> {
    use crate::schema::posts::dsl::*;    

    posts.load::<Post>(conn)
}
