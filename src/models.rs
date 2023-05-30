use diesel::{Queryable, Insertable, Selectable};

use crate::schema::posts;

#[derive(Debug, Queryable, Selectable)]
#[diesel(table_name = posts)]
pub struct Post {
  pub id: i32,
  pub title: String,
  pub body: String,
}

#[derive(Debug, Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost {
  pub title: String,
  pub body: String,
}