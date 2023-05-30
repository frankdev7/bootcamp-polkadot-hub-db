use diesel::result::Error;

use crate::repository::PostRepository;
use crate::models::{NewPost, Post};

pub struct PostService {
  pub repository: PostRepository,
}

impl PostService {

  pub fn new() -> Self {
    PostService {
      repository: PostRepository::new()
    }
  }

  pub fn create_post(&mut self, title: &str, body: &str) -> Result<Post, Error> {
    let new_post = NewPost {
      title: String::from(title),
      body: String::from(body),
    };
    self.repository.create(&new_post)
  }

  pub fn get_posts(&mut self) -> Result<Vec<Post>, Error> {
    self.repository.find_all()
  }
}

