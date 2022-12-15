use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use std::env;
use self::models::*;


pub mod models;
pub mod schema;

fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

fn main() {
    use self::schema::people::dsl::*;
    use self::schema::foods::dsl::*;
    use self::schema::people_foods::dsl::*;
    dotenv().ok();

    //establish connection
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = &mut establish_connection();

    
    println!("Hello, world!");

    let user = people.find(1).first::<People>(connection).expect("Error finding random dude");
    //not sure why this doesnt work
    let results = <PeopleFood as BelongingToDsl<&People>>::belonging_to(&user);//PeopleFood::belonging_to(&user).first(connection);


    println!("Displaying {} people", results.len());
    for p in results {
        println!("{}", p.name);
    }

}
