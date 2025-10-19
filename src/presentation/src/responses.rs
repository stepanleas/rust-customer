use crate::validation::ValidationFieldError;
use application::{CustomerDto, Settings};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

const OK_STATUS: &str = "Ok";

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub code: u16,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationFieldError>>,
}

#[readonly::make]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct AppInfoResponse {
    environment: String,
}

impl crate::responses::AppInfoResponse {
    pub fn new(settings: Settings) -> Self {
        Self {
            environment: settings.environment,
        }
    }
}

#[readonly::make]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct HealthCheckResponse {
    pub status: String,
}

impl HealthCheckResponse {
    pub fn new() -> Self {
        Self {
            status: OK_STATUS.to_string(),
        }
    }
}

#[readonly::make]
#[derive(Serialize, Deserialize, ToSchema)]
pub struct CustomerResponse {
    id: Uuid,
    user_name: String,
    first_name: String,
    last_name: String,
}

impl From<CustomerDto> for CustomerResponse {
    fn from(customer: CustomerDto) -> Self {
        Self {
            id: customer.id(),
            user_name: customer.user_name().into(),
            first_name: customer.first_name().into(),
            last_name: customer.last_name().into(),
        }
    }
}
