use crate::api::docs::customer_api_doc::CustomerApiDoc;
use crate::api::docs::health_check_api_doc::HealthCheckApiDoc;
use utoipa::OpenApi;
use utoipa::openapi::OpenApi as OpenApiStruct;

pub fn open_api_docs() -> OpenApiStruct {
    let mut openapi = CustomerApiDoc::openapi();

    openapi.merge(HealthCheckApiDoc::openapi());

    openapi
}
