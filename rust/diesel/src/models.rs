use diesel::prelude::*;
use super::schema::*;

#[derive(Queryable, Identifiable, Associations)]
#[table_name="people"]
#[belongs_to(PeopleFood)]
pub struct People {
    pub id: i32,
    pub name: String,
    pub age: Option<i32>,
}

#[derive(Queryable, Identifiable, Associations)]
#[table_name="foods"]
#[belongs_to(PeopleFood)]
pub struct Foods {
    pub id: i32,
    pub name: String,
}

#[derive(Queryable, Associations)]
#[belongs_to(People, foreign_key = "people_id")]
#[belongs_to(Foods, foreign_key = "foods_id")]
#[table_name="people_foods"]
pub struct PeopleFood {
    pub foods_id: i32,
    pub people_id: i32,
}

