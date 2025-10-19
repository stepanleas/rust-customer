use crate::api::api_customer::__path_create;

use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    tags(
        (name = "Customer", description = "Customer management endpoints.")
    ),
    paths(
        create,
    )
)]
pub(crate) struct CustomerApiDoc;
