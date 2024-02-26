use axum::{
    http::StatusCode,
    extract
};
use tracing::info;

// struct UpdateInfo {

// }
// pub async fn account_updated(extract::Json(payload): extract::Json<UpdateInfo>) -> StatusCode {
//     info!("account_updated");

//     StatusCode::OK
// }

pub async fn account_updated() -> StatusCode {
    info!("account_updated");
    StatusCode::OK
}
