// @generated automatically by Diesel CLI.

diesel::table! {
    foods (id) {
        id -> Int4,
        name -> Varchar,
    }
}

diesel::table! {
    people (id) {
        id -> Int4,
        name -> Varchar,
        age -> Nullable<Int4>,
    }
}

diesel::table! {
    people_foods (foods_id, people_id) {
        foods_id -> Int4,
        people_id -> Int4,
    }
}

diesel::joinable!(people_foods -> foods (foods_id));
diesel::joinable!(people_foods -> people (people_id));

diesel::allow_tables_to_appear_in_same_query!(
    foods,
    people,
    people_foods,
);
