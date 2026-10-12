use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(info(title = "Intern API", version = "1.0", description = "API documentation for the Intern API"))]
pub struct ApiDoc;
