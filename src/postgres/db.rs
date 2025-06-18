use sqlx::postgres::PgPoolOptions;
#[actix_web::main]
pub async fn main() -> Result<(), sqlx::Error>
{
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://postgres:12345678@localhost/test").await?;
    
    Ok(())
}