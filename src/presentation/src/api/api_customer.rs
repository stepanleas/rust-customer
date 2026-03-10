use crate::app_state::AppState;
use crate::error::ApiError;
use crate::requests::{CreateCustomerRequest, UpdateCustomerRequest};
use crate::responses::CustomerResponse;
use crate::validation::ValidatedJson;
use actix_web::web::Path;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, Responder, post, put, web};
use anyhow::anyhow;
use application::commands::{CreateCustomerCommand, UpdateCustomerCommand};
use application::handlers::{CreateCustomerCommandHandler, UpdateCustomerCommandHandler};
use serde_json::json;
use uuid::Uuid;

const CUSTOMERS: &str = "Customers";

#[tracing::instrument(skip(req))]
#[utoipa::path(
    tag = CUSTOMERS,
    operation_id = "create_customer",
    responses(
        (status = 201, description = "Create a customer", body = [CustomerResponse])
    ),
    request_body = CreateCustomerRequest,
)]
#[post("")]
pub async fn create(
    req: HttpRequest,
    request: ValidatedJson<CreateCustomerRequest>,
) -> Result<impl Responder, ApiError> {
    let correlation_id = req
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or("unknown".to_string());

    tracing::info!(%correlation_id, "Handling customer create");

    let payload = request.into_inner();

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = CreateCustomerCommandHandler::new(
        state.customer_repository.clone(),
        state.customer_message_publisher.clone(),
    );

    let command = CreateCustomerCommand::new(
        payload.user_name.clone(),
        payload.first_name.clone(),
        payload.last_name.clone(),
    );

    let customer = handler.execute(command).await?;

    Ok(HttpResponse::Created().json(json!({ "data": CustomerResponse::from(customer) })))
}

#[tracing::instrument(skip(req))]
#[utoipa::path(
    tag = CUSTOMERS,
    operation_id = "update_customer",
    responses(
        (status = 200, description = "Update a customer by id", body = [CustomerResponse])
    ),
    params(
        ("id", description = "Id of the customer to update")
    ),
    request_body = UpdateCustomerRequest,
)]
#[put("/{id}")]
pub async fn update(
    req: HttpRequest,
    id: Path<Uuid>,
    request: ValidatedJson<UpdateCustomerRequest>,
) -> Result<impl Responder, ApiError> {
    let correlation_id = req
        .extensions()
        .get::<String>()
        .cloned()
        .unwrap_or("unknown".to_string());

    tracing::info!(%correlation_id, "Handling customer update");

    let payload = request.into_inner();

    let state = req
        .app_data::<web::Data<AppState>>()
        .ok_or_else(|| ApiError::internal(anyhow!("Missing app state")))?;

    let handler = UpdateCustomerCommandHandler::new(
        state.customer_repository.clone(),
        state.customer_message_publisher.clone(),
    );

    let command = UpdateCustomerCommand::new(
        id.into_inner(),
        payload.user_name.clone(),
        payload.first_name.clone(),
        payload.last_name.clone(),
    );

    let customer = handler.execute(command).await?;

    Ok(HttpResponse::Ok().json(json!({ "data": CustomerResponse::from(customer) })))
}
