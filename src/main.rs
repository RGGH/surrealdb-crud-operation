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
    
    let mithapukur: Vec<Record> = db
        .create("District")
        .content(District {
                number_of_thana: 2,
                population: 1000000,
                police_station: vec!["mithapukur".to_string()]
        })
        .await?;
    println!("{:?}", mithapukur);
//     [
//     {
//         "id": "District:hc7tvmarkgbj74cxjc4c",
//         "number_of_thana": 2,
//         "police_station": [
//             "mithapukur"
//         ],
//         "population": 1000000
//     }
// ]

    // Update all records in a table
   let mithapukur: Vec<Record> = db.update("District")
     .content(District {
            number_of_thana: 1,
            population: 100002873,
             police_station: vec!["mithapukur".to_string(), "Jaigirhat".to_string()]
                     })
     .await?;
    println!("{:?}", mithapukur);
    dbg!(mithapukur);

    // update only single field ok
    let query = r#"
        UPDATE District
        SET number_of_thana = 2
        WHERE id="District:2v88h9zbdx9hqc9owr5i"
    "#;
    db.query(query).await?;

    // update only vector field
    let query = r#"
        UPDATE District
        SET police_station += ['shathibari']
        WHERE id="District:2v88h9zbdx9hqc9owr5i"
    "#;
    db.query(query).await?;

    // update only single with increment
    let query = r#"
        UPDATE District
        SET number_of_thana = number_of_thana + 4
        WHERE id="District:9j4uapk2bn56ix4yy7tm"
    "#;
    db.query(query).await?;

    // update only single with decrement
    let query = r#"
        UPDATE District
        SET number_of_thana = number_of_thana - 1
        WHERE id="District:2v88h9zbdx9hqc9owr5i"
    "#;
    db.query(query).await?;

    // update only single with increment
       // update only vector field
       let query = r#"
       UPDATE District
       SET police_station += ['shathibari', 'mirjapur', 'shukurhat']
       WHERE id="District:2v88h9zbdx9hqc9owr5i"
   "#;
   db.query(query).await?;

   // update number_of_thana according to lenght of police_station lenth
//    let query = r#"
//     UPDATE District
//     SET number_of_thana = LENGTH(police_station)
// "#;
//     db.query(query).await?;
    // delete record
    let query = r#"
        DELETE FROM District
                    
        WHERE id="District:2v88h9zbdx9hqc9owr5i"
    "#;
    db.query(query).await?;

   
    

    // Delete all records in a table
    // db.delete("khulna").await?;

    Ok(())
}
