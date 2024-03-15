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
    police_station: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct Record {
    id: Thing,
    number_of_thana: usize,
    population: usize,
    police_station: Vec<String>
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
            police_station: vec!["mithapukur".to_string()],
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
    let mithapukur: Vec<Record> = db
        .update("District")
        .content(District {
            number_of_thana: 1,
            population: 100002873,
            police_station: vec!["mithapukur".to_string(), "Jaigirhat".to_string()],
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

    // update a police_station filed to add another one as well as update number_of_thena
    // x3we856001c2ibzweexz
    let update_thana = r#"
                     UPDATE District
                     SET police_station += ['pirgacha','pirgoang']
                     WHERE id="District:x3we856001c2ibzweexz"
             
    "#;
    db.query(update_thana).await?;

    //    update number_of_thana according to lenght of police_station lenth
    let query = r#"
   UPDATE District
   SET number_of_thana = array::len(police_station)
"#;
    db.query(query).await?;


    // Print every entry of the struct that select from database 
    // to extract the every filed of struct you have to need Record struct above.
    let entries: Vec<Record> = db.select("District").await?;
    entries.iter().for_each(|entry| {
        println!("{}", entry.id);
        println!("{}", entry.number_of_thana);
        println!("{}", entry.population);
        println!("{:?}", entry.police_station);
    });

    // delete COLUMN
    // let query = r#"
    //     UPDATE  District
    //        SET population = REMOVE 
    //     WHERE id="District:x3we856001c2ibzweexz"
    // "#;
    // db.query(query).await?;


    // Read records

    let select : Vec<Record>= db.select("District").await?;
    // println!("SELECT : {:?}", select);
        for record in select.iter(){
            // println!("record {:?}", record.id.id);
            println!("{:?}", record.id.tb.find("number_of_thana").unwrap())
        }


    // Delete all records in a table
    // db.delete("khulna").await?;

    Ok(())
}
