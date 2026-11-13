use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DoctorFieldsDto {
    Name,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MaterialFieldsDto {
    Name,
}
