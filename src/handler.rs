use route_recognizer;

mod entity;
pub fn get_res() -> entity::Employee {

    let router = route_recognizer::Router::new();


    let employee = entity::Employee {
        name: "David Kunz".to_string(),
        birth_year: 1986,
        employment_type: entity::EmploymentType::Permanent,
    };
    employee
}