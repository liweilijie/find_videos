use uuid::Uuid;

pub fn uuid_v4() -> String {
    Uuid::new_v4().as_simple().to_string()
}