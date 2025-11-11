use crate::kafka::avro::models::{
    AvroSerializable, CustomerCreatedEventAvroModel, CustomerUpdatedEventAvroModel,
};
use crate::kafka::producer::KafkaProducer;
use anyhow::anyhow;
use application::ports::output::publishers::CustomerMessagePublisher;
use domain::events::{CustomerCreatedEvent, CustomerUpdatedEvent};
use std::sync::Mutex;

pub struct CustomerKafkaPublisher {
    producer: Mutex<KafkaProducer>,
}

impl CustomerKafkaPublisher {
    pub fn new(producer: KafkaProducer) -> Self {
        Self {
            producer: Mutex::new(producer),
        }
    }

    fn publish_event<T, E>(&self, topic: &str, event: T) -> anyhow::Result<()>
    where
        T: Into<E>,
        E: AvroSerializable,
    {
        let mut producer = self
            .producer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let avro_model: E = event.into();
        producer.send(topic, &avro_model.to_avro_bytes()?)
    }
}

impl CustomerMessagePublisher for CustomerKafkaPublisher {
    fn publish_created(&self, event: CustomerCreatedEvent) -> anyhow::Result<()> {
        let customer_id = &event.customer().id().as_uuid().to_string();
        tracing::info!(
            "Received CustomerCreatedEvent for customer with id: {}",
            customer_id,
        );

        match self.publish_event::<CustomerCreatedEvent, CustomerCreatedEventAvroModel>(
            "customer-created",
            event,
        ) {
            Ok(_) => {
                tracing::info!(
                    "CustomerCreatedEvent published successfully for customer with id: {}",
                    customer_id,
                );

                Ok(())
            }
            Err(error) => {
                tracing::error!(
                    "Error while sending CustomerCreatedEvent to kafka for customer id: {}. {}",
                    customer_id,
                    error.to_string(),
                );

                Err(anyhow!(error))
            }
        }
    }

    fn publish_updated(&self, event: CustomerUpdatedEvent) -> anyhow::Result<()> {
        let customer_id = &event.customer().id().as_uuid().to_string();
        tracing::info!(
            "Received CustomerUpdatedEvent for customer with id: {}",
            customer_id,
        );

        match self.publish_event::<CustomerUpdatedEvent, CustomerUpdatedEventAvroModel>(
            "customer-updated",
            event,
        ) {
            Ok(_) => {
                tracing::info!(
                    "CustomerUpdatedEvent published successfully for customer with id: {}",
                    customer_id,
                );

                Ok(())
            }
            Err(error) => {
                tracing::error!(
                    "Error while sending CustomerUpdatedEvent to kafka for customer id: {}. {}",
                    customer_id,
                    error.to_string(),
                );

                Err(anyhow!(error))
            }
        }
    }
}
