use serde::Serialize;

pub trait AvroSerializable {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>>;
}

#[derive(Serialize)]
pub(crate) struct CustomerAvroModel {
    id: String,
    user_name: String,
    first_name: String,
    last_name: String,
}

impl CustomerAvroModel {
    pub fn new(id: String, user_name: String, first_name: String, last_name: String) -> Self {
        Self {
            id,
            user_name,
            first_name,
            last_name,
        }
    }
}
