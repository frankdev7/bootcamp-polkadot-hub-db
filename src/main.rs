use postgres::{Client, NoTls};
use dotenvy::dotenv;
use std::error::Error;
use dotenvy_macro::dotenv;

fn main() -> Result<(), Box<dyn Error>> {
  dotenv().expect(".env file not found");

  let database_url = dotenv!("DATABASE_URL");
  let client = Client::connect(database_url, NoTls);
  
  match client {
    Ok(mut db) => {
      db.batch_execute(
        "CREATE TABLE IF NOT EXISTS usuarios (
          id SERIAL PRIMARY KEY,
          nombre TEXT NOT NULL,
          edad INTEGER NOT NULL
        )"
      )?;

      let name = "Frank";
      let age = 26;

      db.execute("INSERT INTO usuarios (nombre, edad) VALUES ($1, $2)", &[&name, &age])?;

      for row in db.query("SELECT id, nombre, edad FROM usuarios", &[])? {
        let id: i32 = row.get(0);
        let name: String = row.get(1);
        let age: i32 = row.get(2);

        println!("ID: {}, Nombre: {}, Edad: {}", id, name, age);
      }

    }
    Err(e) => {
      println!("Can't connect to the database {}", e);
    }
  }

  Ok(())
}