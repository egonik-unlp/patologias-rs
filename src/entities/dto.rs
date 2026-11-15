use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DoctorFieldsDto {
    Name,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialFieldsDto {
    Name,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NewDoctorDto {
    name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DoctorDto {
    pub id: i32,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialsDto {
    pub id: i32,
    pub name: String,
}
