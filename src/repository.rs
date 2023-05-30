use dotenvy::dotenv;
use dotenvy_macro::dotenv;

use diesel::prelude::*;
use diesel::result::Error;

use crate::schema::posts::dsl::*;
use crate::models::{NewPost, Post};

pub struct PostRepository {
  pub conn: SqliteConnection,
}

impl PostRepository {

  pub fn new() -> Self {
    dotenv().expect(".env file not found");
    let database_url = dotenv!("DATABASE_URL");

    PostRepository {
      conn: SqliteConnection::establish(database_url).expect(&format!("Error connecting to {}", database_url))
    }
  }

  pub fn find_all(&mut self) -> Result<Vec<Post>, Error> {
    posts.load::<Post>(&mut self.conn)
  }

  pub fn create(&mut self, new_post: &NewPost) -> Result<Post, Error>  {
    diesel::insert_into(posts)
      .values(new_post)
      .execute(&mut self.conn)
      .expect("Error saving new post");

    posts.order(id.desc()).first(&mut self.conn)

  }
  
}