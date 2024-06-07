use actix_web::{get, web, Error, HttpResponse};

use crate::database::user::{LoginResponse, UserDB, UserLoginSchema};

#[get("/api/login")]
async fn login(login: web::Json<UserLoginSchema>, user_db: web::Data<UserDB>) -> Result<HttpResponse, Error> {
    // When the user login is successful,
    // the user has secure login. Otherwise, not logged in.
    match user_db.login_user(login.into_inner()) {
        LoginResponse::Success(user) => {
            // User logged in successfully
            todo!()
        }, 
        LoginResponse::UnknownUsername => todo!(),
        LoginResponse::WrongPassword => todo!(), // Handle limited number of tries
    }
}