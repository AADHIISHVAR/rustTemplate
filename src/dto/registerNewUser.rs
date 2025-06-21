use crate::models::registerNewUser::RegisterUser;
use actix_web::{post, web, HttpResponse, Responder};
use sea_orm::{ActiveModelTrait, DatabaseConnection, NotSet, Set};
use crate::entitys::newUserEntity;

use argon2::{
    password_hash::{
        rand_core::OsRng,
        PasswordHash, PasswordHasher, PasswordVerifier, SaltString
    },
    Argon2
};


pub async fn register_users(db: web::Data<DatabaseConnection>, new_user: web::Json<RegisterUser>) -> impl Responder
{
    println!("entered registering users state");
    let org_pass = &new_user.password;
    let mut salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let passhashed = match argon2.hash_password(org_pass.as_bytes(), &salt)
    {
        Ok(done) => done.to_string(),
        Err(e) =>
            {
                eprintln!("password hashing error {:?}",e);
                return HttpResponse::InternalServerError().body("hashing failed");
            }
    };

    let user = newUserEntity::ActiveModel
    {
        id: NotSet,
        name: Set(new_user.name.clone()),
        username: Set(new_user.username.clone()),
        email: Set(new_user.email.clone()),
        password:Set(passhashed)
    };

   match user.insert(db.get_ref()).await
   {
       Ok(_) =>HttpResponse::Ok().body("success"),
       Err(e) =>
           {
               eprintln!("failed to insert user state {:?}",e);
               HttpResponse::InternalServerError().body(format!("failed to insert user state {:?}",e))
           }
   }
}
