use crate::kafka::avro::events::parse_schema;
use crate::kafka::avro::models::{AvroSerializable, CustomerAvroModel};
use domain::events::CustomerUpdatedEvent;
use serde::Serialize;

#[derive(Serialize)]
pub struct CustomerUpdatedEventAvroModel {
    id: String,
    customer: CustomerAvroModel,
    created_at: String,
}

impl AvroSerializable for CustomerUpdatedEventAvroModel {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let schema_path =
            include_str!("../../../../avro/schemas/customer_updated_event_avro_model.avsc");

        parse_schema(schema_path, self)
    }
}

impl From<CustomerUpdatedEvent> for CustomerUpdatedEventAvroModel {
    fn from(event: CustomerUpdatedEvent) -> Self {
        Self {
            id: event.id().into(),
            customer: CustomerAvroModel::new(
                event.customer().id().as_uuid().to_string(),
                event.customer().user_name().into(),
                event.customer().first_name().into(),
                event.customer().last_name().into(),
            ),
            created_at: event.created_at().to_string(),
        }
    }
}
