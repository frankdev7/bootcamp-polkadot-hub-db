use service::PostService;

mod schema;
mod models;
mod repository;
mod service;

fn main() {

  let new_title = "Ejemplo de Post";
  let new_body = "Este es un ejemplo funcional parte almacenar un POST";

  let mut post_service = PostService::new();
  let new_post = post_service.create_post(new_title, new_body);
  println!("{:?}", new_post);
}