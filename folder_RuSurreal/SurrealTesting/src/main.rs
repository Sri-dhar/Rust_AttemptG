use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::{Error, Surreal};
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    // Connect to the SurrealDB server
    let db = Surreal::new::<Ws>("127.0.0.1:8000").await?;

    // Sign in as a root user
    db.signin(Root {
        username: "root",
        password: "root",
    })
    .await?;

    // Select a specific namespace / database
    db.use_ns("test").use_db("test").await?;

    // Define the table
    db.query("DEFINE TABLE commands SCHEMAFULL;").await?;
    db.query("DEFINE FIELD id ON TABLE commands TYPE string;").await?;
    db.query("DEFINE FIELD cmd ON TABLE commands TYPE string;").await?;

    // Create a new command
    let created: Option<Command> = db
        .create("commands")
        .content(Command {
            id: "cmd1".to_string(),
            cmd: "ls -la".to_string(),
        })
        .await?;

    println!("Created: {:?}", created);

    // Retrieve a command
    let retrieved: Option<Command> = db.select(("commands", "cmd1")).await?;
    println!("Retrieved: {:?}", retrieved);

    // Retrieve all commands
    let all_commands: Vec<Command> = db.select("commands").await?;
    println!("All commands: {:?}", all_commands);

    Ok(())
}

#[derive(Debug, Serialize, Deserialize)]
struct Command {
    id: String,
    cmd: String,
}