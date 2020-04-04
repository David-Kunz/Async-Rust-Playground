use {serde::{Deserialize, Serialize}, serde_json};


#[derive(Serialize, Deserialize)]
pub struct Employee {
    pub name: String,
    pub birth_year: u16,
    pub employment_type: EmploymentType
}

#[derive(Serialize, Deserialize)]
pub enum EmploymentType {
    Permanent,
    PartTime,
    Trainee
}