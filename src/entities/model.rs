use diesel::{associations::Identifiable, deserialize::Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::doctors)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Doctor {
    id: i32,
    name: String,
}

#[derive(Debug, Serialize, Deserialize, Queryable, Selectable, Identifiable)]
#[diesel(table_name= crate::core::database::schema::materials)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Material {
    id: i32,
    name: String,
}
