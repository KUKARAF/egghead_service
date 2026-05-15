use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use utoipa::{
    openapi::security::{ApiKey, ApiKeyValue, Http, HttpAuthScheme, SecurityScheme},
    Modify, OpenApi,
};

use crate::api::{
    me::{ApiTokenResponse, GenerateTokenResponse},
    tasks::{CreateTaskRequest, CreateTaskResponse, GetTaskResponse},
};
use crate::error::ErrorResponse;
use crate::models::task::TaskView;

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::tasks::create_task,
        crate::api::tasks::get_task,
        crate::api::me::list_tasks,
        crate::api::me::get_task_detail,
        crate::api::me::approve_task,
        crate::api::me::reject_task,
        crate::api::me::get_api_token,
        crate::api::me::regenerate_api_token,
    ),
    components(schemas(
        CreateTaskRequest,
        CreateTaskResponse,
        GetTaskResponse,
        TaskView,
        ApiTokenResponse,
        GenerateTokenResponse,
        ErrorResponse,
    )),
    modifiers(&SecurityAddon),
    info(
        title = "egghead_service API",
        version = "1.0.0",
        description = "Userscript generation backend"
    )
)]
pub struct ApiDoc;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        let components = openapi.components.get_or_insert_with(Default::default);
        components.add_security_scheme(
            "ApiToken",
            SecurityScheme::Http(Http::new(HttpAuthScheme::Bearer)),
        );
        components.add_security_scheme(
            "SessionCookie",
            SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("session_token"))),
        );
    }
}

pub async fn get_openapi_json() -> Response {
    match ApiDoc::openapi().to_json() {
        Ok(json) => (
            StatusCode::OK,
            [("content-type", "application/json")],
            json,
        )
            .into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
