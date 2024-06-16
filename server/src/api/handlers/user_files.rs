use actix_web::{get, post, web, HttpMessage, HttpRequest, HttpResponse, Responder};
use serde_json::json;

use crate::{api::{jwt_auth, types::UserDataError}, database::user::UserDB};

#[post("/upload/{file_type}")]
async fn file_upload(
    req: HttpRequest,
    user_db: web::Data<UserDB>,
    _: jwt_auth::JwtMiddleware,
) -> impl Responder {
    let ext = req.extensions();
    let user_id = ext.get::<uuid::Uuid>().unwrap();

    if let Some(user_data) = user_db.get_data(user_id.into()) { 
        HttpResponse::Ok().json(json!({"status": "success"}))
    } else {
        HttpResponse::InternalServerError().json(UserDataError::UserNotFound(*user_id))
    }
}