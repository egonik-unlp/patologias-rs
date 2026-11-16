use diesel::{associations::Identifiable, deserialize::Queryable, prelude::Insertable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, Identifiable, Queryable)]
#[diesel(table_name= crate::core::database::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    id: i32,
    name: String,
    // privi
}
