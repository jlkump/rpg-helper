use actix_web::{get, web, Error, HttpResponse};

use crate::database::user::{UserDB, UserLoginSchema};

#[get("/api/login")]
async fn login(login: web::Json<UserLoginSchema>) -> Result<HttpResponse, Error> {
    // TODO: Where to put DB since we have async?
    todo!()
}