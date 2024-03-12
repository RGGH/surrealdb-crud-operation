use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Wss, opt::auth::Root, sql::Thing, Surreal};

#[derive(Debug, Deserialize, Serialize)]
struct Division {
    name: String,
    climate: String,
    population: usize,
    international_boder: bool,
}

#[derive(Debug,Deserialize)]
struct Record {
    id: Thing,
}

#[tokio::main]
async fn main() -> surrealdb::Result<()> {
    let db = Surreal::new::<Wss>("generalpione.preciqprojects.com").await?;

    db.signin(Root {
        username: "root",
        password: "test12345",
    })
    .await?;

    db.use_ns("bd").use_db("bd").await?;
    
    let shatkhira: Vec<Record> = db
        .create("khulna")
        .content(Division {
            name: "Shatkhira".to_string(),
            climate: "Rough".to_string(),
            population: 340000,
            international_boder: true,
        })
        .await?;
    println!("{:?}", shatkhira);

    Ok(())
}
