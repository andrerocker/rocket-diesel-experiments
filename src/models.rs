use crate::schema::posts;
use crate::schema::fragments;

use diesel::{Queryable, Insertable};
use diesel::result::QueryResult;

use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use diesel::prelude::*;

use crate::error::CustomError;

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Identifiable)]
pub struct Post {
    id: i32,
    title: String,
    sub_title: String
}

#[derive(Debug, Clone, Deserialize, Serialize, Queryable, Insertable, Identifiable, Associations)]
#[belongs_to(Post)]
pub struct Fragment {
    id: i32,
    image_url: String,
    comment: String,
    post_id: i32
}

pub fn posts(conn:&mut diesel::PgConnection) -> QueryResult<Vec<Post>> {
    use crate::schema::posts::dsl::*;
    posts.load::<Post>(conn)
}

pub fn fragments(conn:&mut diesel::PgConnection) -> QueryResult<Vec<Fragment>> {
    use crate::schema::fragments::dsl::*;
    fragments.load::<Fragment>(conn)
}

pub type Posts = Result<Json<Vec<Post>>, CustomError>;
pub type Fragments = Result<Json<Vec<Fragment>>, CustomError>;
