use apache_avro::Schema;
use domain::events::{CustomerCreatedEvent, CustomerUpdatedEvent};
use serde::Serialize;

pub trait AvroSerializable {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>>;
}

#[derive(Serialize)]
struct CustomerAvroModel {
    id: String,
    user_name: String,
    first_name: String,
    last_name: String,
}

#[derive(Serialize)]
pub struct CustomerCreatedEventAvroModel {
    id: String,
    customer: CustomerAvroModel,
    created_at: String,
}

impl AvroSerializable for CustomerCreatedEventAvroModel {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let schema_path =
            include_str!("../../../avro/schemas/customer_created_event_avro_model.avsc");

        parse_schema(schema_path, self)
    }
}

impl From<CustomerCreatedEvent> for CustomerCreatedEventAvroModel {
    fn from(event: CustomerCreatedEvent) -> Self {
        Self {
            id: event.id().into(),
            customer: CustomerAvroModel {
                id: event.customer().id().as_uuid().to_string(),
                user_name: event.customer().user_name().into(),
                first_name: event.customer().first_name().into(),
                last_name: event.customer().last_name().into(),
            },
            created_at: event.created_at().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct CustomerUpdatedEventAvroModel {
    id: String,
    customer: CustomerAvroModel,
    created_at: String,
}

impl AvroSerializable for CustomerUpdatedEventAvroModel {
    fn to_avro_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let schema_path =
            include_str!("../../../avro/schemas/customer_updated_event_avro_model.avsc");

        parse_schema(schema_path, self)
    }
}

impl From<CustomerUpdatedEvent> for CustomerUpdatedEventAvroModel {
    fn from(event: CustomerUpdatedEvent) -> Self {
        Self {
            id: event.id().into(),
            customer: CustomerAvroModel {
                id: event.customer().id().as_uuid().to_string(),
                user_name: event.customer().user_name().into(),
                first_name: event.customer().first_name().into(),
                last_name: event.customer().last_name().into(),
            },
            created_at: event.created_at().to_string(),
        }
    }
}

fn parse_schema<T: Serialize>(schema_path: &str, value: T) -> anyhow::Result<Vec<u8>> {
    let schema = Schema::parse_str(schema_path)?;
    let value = apache_avro::to_value(value)?;
    let mut writer = apache_avro::Writer::new(&schema, Vec::new());

    writer.append(value)?;
    writer.flush()?;

    Ok(writer.into_inner()?)
}
