use sea_orm::{EntityTrait, QueryFilter, ColumnTrait, DatabaseConnection, DbErr};
use actix_web::{web, HttpResponse, Responder};
use crate::models::login_user_model::LoginUser;
use crate::entitys::newUserEntity;
use argon2::{Argon2, PasswordHash, PasswordVerifier};
use sea_orm::Condition;
use crate::dto::jwt_encode::user_jwt_encoder;

pub async fn login_users(login_user: web::Json<LoginUser>, db: web::Data<DatabaseConnection>, ) -> impl Responder
{
    println!("entered login users state");

    let username_or_email = login_user.username_or_email.clone();
    let input_password = login_user.password.clone();

    //WHERE username = 'value' OR email = 'value'
    let retrive_data = Condition::any()
        .add(newUserEntity::Column::Username.eq(username_or_email.clone()))
        .add(newUserEntity::Column::Email.eq(username_or_email.clone()));

    // ✅ Step 1: Fetch user by username and email
    let user = newUserEntity::Entity::find()
        .filter(retrive_data)
        .one(db.get_ref())
        .await;

    //step2 : compare teh password in teh db and teh user entered
    match user
    {
        Ok(Some(user_model)) =>
            {
                let parsed_hash = PasswordHash::new(user_model.password.as_str());  // hash from DB

                if Argon2::default()
                    .verify_password(input_password.as_bytes(), &parsed_hash.unwrap())  // plain password as input
                    .is_ok()
                {
                    user_jwt_encoder(username_or_email, "public".to_string());
                    HttpResponse::Ok().body("user logged in")
                } 
                else {
                    HttpResponse::Unauthorized().body("invalid credentials")
                }

            },
        Err(e) => {
            HttpResponse::InternalServerError().body(format!("error: {:?}", e))
        }

        Ok(None) =>
            {
                HttpResponse::Ok().body(format!("no user found in the given param(email or username) {}", &login_user.username_or_email))
            }
    }
}
