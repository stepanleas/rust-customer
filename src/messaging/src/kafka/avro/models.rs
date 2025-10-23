use apache_avro::{Schema, Writer};
use domain::CustomerCreatedEvent;
use serde::Serialize;

#[derive(Serialize)]
pub struct CustomerCreatedEventAvroModel {
    id: String,
    user_name: String,
    first_name: String,
    last_name: String,
    created_at: String,
}

impl CustomerCreatedEventAvroModel {
    pub fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let schema = Schema::parse_str(include_str!(
            "../../../avro/schemas/customer_created_event_avro_model.avsc"
        ))
        .expect("invalid Avro schema");

        let value = apache_avro::to_value(self)?;
        let mut writer = Writer::new(&schema, Vec::new());

        writer.append(value)?;
        writer.flush()?;

        Ok(writer.into_inner()?)
    }
}

impl From<CustomerCreatedEvent> for CustomerCreatedEventAvroModel {
    fn from(event: CustomerCreatedEvent) -> Self {
        Self {
            id: event.customer().id().as_uuid().to_string(),
            user_name: event.customer().user_name().into(),
            first_name: event.customer().first_name().into(),
            last_name: event.customer().last_name().into(),
            created_at: event.created_at().to_string(),
        }
    }
}
