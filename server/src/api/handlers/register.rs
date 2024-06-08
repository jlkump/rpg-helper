use actix_web::{post, web, HttpResponse, Responder};

use crate::database::user::{RegistrationResponse, UserDB, UserRegistrationSchema};


#[post("/auth/register")]
async fn register_user_handler(body: web::Json<UserRegistrationSchema>, db: web::Data<UserDB>) -> impl Responder {
    let registration_response = db.create_user(body.into_inner());

    match registration_response {
        RegistrationResponse::Success(user_id) => {
            // TODO: Replace with serialized structs instead for more uniform access patterns.
            let user_respose = serde_json::json!({"status": "success", "data": serde_json::json!({"user": user_id})});
            return HttpResponse::Ok().json(user_respose);
        },
        RegistrationResponse::EmailTaken => {
            return HttpResponse::InternalServerError().json(serde_json::json!({"status": "error", "message": "Email Taken"}));
        },
        RegistrationResponse::UsernameTaken => {
            return HttpResponse::InternalServerError().json(serde_json::json!({"status": "error", "message": "Username Taken"}));
        },
    }
}