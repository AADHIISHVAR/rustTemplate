mod dto;
use crate::dto::registerNewUser::register_users;
use crate::dto::login_users::login_users;
mod entitys;
mod models;
mod postgres;
use actix_web::*;
use sea_orm::Database;

#[get{"/greet"}]
 async fn greet() ->impl Responder
{
    "welcome to karigalan magic show"
}

#[actix_web::main ]
async fn main() -> std::io::Result<()> {
    let db = Database::connect("postgres://postgres:12345678@localhost:5432/postgres").await.unwrap();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(db.clone()))
            // .route("/register-user", web::post().to(create_user))
            .service(greet)
            .route("/register-newuser",web::post().to(register_users))
            .route("/user-login",web::post().to(login_users))
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
