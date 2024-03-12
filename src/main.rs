use serde::{Deserialize, Serialize};
use surrealdb::{engine::remote::ws::Wss, opt::auth::Root, sql::Thing, Surreal};

#[derive(Debug, Deserialize, Serialize)]
struct Division {
    name: String,
    climate: String,
    population: usize,
    international_boder: bool,
}
#[derive(Debug, Deserialize, Serialize)]
struct District {
    number_of_thana: usize,
    population: usize,
    police_station: Vec<String>
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

    db.use_ns("district").use_db("district").await?;
    
    let shatkhira: Vec<Record> = db
        .create("District")
        .content(District {
                number_of_thana: 2,
                population: 1000000,
                police_station: vec!["mithapukur".to_string()]
        })
        .await?;
    println!("{:?}", shatkhira);

    // Update all records in a table
//    let district: Vec<Record> = db.update("khulna")
//      .content(Division {
//             name: "Shatkhira".to_string(),
//             climate: "Sunny".to_string(),
//             population: 340000,
//             international_boder: true,
//         })
//      .await?;

    // Delete all records in a table
    // db.delete("khulna").await?;

    Ok(())
}
