use crate::services::auth;
use crate::services::categories;
use utoipa::OpenApi;
#[derive(OpenApi)]
#[openapi(paths(categories::create_category, auth::register,))]
pub struct ApiDoc;
use std::fs::File;
use std::io::Write;
#[cfg(debug_assertions)]
pub fn write_to_file() {
    let openapi_json = ApiDoc::openapi().to_pretty_json().unwrap();
    let mut file = File::create("openapi.json").unwrap();
    writeln!(file, "{}", openapi_json).unwrap();
    log::info!("{}112112312", openapi_json);
}
