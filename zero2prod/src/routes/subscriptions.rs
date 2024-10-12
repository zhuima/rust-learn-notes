use actix_web::{web, HttpResponse};
use sqlx::Pool;
use log;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


pub async fn subscribe(form: web::Form<FormData>, _connection: web::Data<Pool>) -> HttpResponse {
    log::info!("Subscribing a new user with email: {}", form.email);
    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        uuid::Uuid::new_v4(),
        form.email,
        form.name,
        chrono::Utc::now()
    )
    .execute(_connection.get_ref())
    .await
    {
        Ok(_) => {
            log::info!("User {} subscribed successfully", form.email);
            HttpResponse::Ok().finish()
        },
        Err(e) => {
            log::error!("Failed to execute query: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}
