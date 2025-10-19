use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[readonly::make]
#[derive(Deserialize, Validate, ToSchema)]
pub struct CreateCustomerRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Username must be between 1 and 255 characters"
    ))]
    pub user_name: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "First name must be between 1 and 255 characters"
    ))]
    pub first_name: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Last name must be between 1 and 255 characters"
    ))]
    pub last_name: String,
}

#[readonly::make]
#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateCustomerRequest {
    #[validate(length(
        min = 1,
        max = 255,
        message = "Username must be between 1 and 255 characters"
    ))]
    pub user_name: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "First name must be between 1 and 255 characters"
    ))]
    pub first_name: String,

    #[validate(length(
        min = 1,
        max = 255,
        message = "Last name must be between 1 and 255 characters"
    ))]
    pub last_name: String,
}
