use crate::kafka::avro::CustomerCreatedEventAvroModel;
use crate::kafka::producer::KafkaProducer;
use anyhow::anyhow;
use application::CustomerMessagePublisher;
use domain::CustomerCreatedEvent;
use log::{error, info};
use std::sync::Mutex;

pub struct CustomerCreatedEventKafkaPublisher {
    producer: Mutex<KafkaProducer>,
}

impl CustomerCreatedEventKafkaPublisher {
    pub fn new(producer: KafkaProducer) -> Self {
        Self {
            producer: Mutex::new(producer),
        }
    }
}

impl CustomerMessagePublisher for CustomerCreatedEventKafkaPublisher {
    fn publish(&self, event: CustomerCreatedEvent) -> anyhow::Result<()> {
        let customer_id = &event.customer().id().as_uuid().to_string();
        info!(
            "Received CustomerCreatedEvent for customer with id: {}",
            customer_id,
        );

        let mut producer = self
            .producer
            .lock()
            .unwrap_or_else(|poisoned| poisoned.into_inner());

        let avro_model: CustomerCreatedEventAvroModel = event.into();

        match producer.send("customer.created", &avro_model.to_avro_bytes()?) {
            Ok(_) => {
                info!(
                    "CustomerCreatedEvent published successfully for customer with id: {}",
                    customer_id,
                );

                Ok(())
            }
            Err(error) => {
                error!(
                    "Error while sending CustomerCreatedEvent to kafka for customer id: {}. {}",
                    customer_id,
                    error.to_string(),
                );

                Err(anyhow!(error))
            }
        }
    }
}
