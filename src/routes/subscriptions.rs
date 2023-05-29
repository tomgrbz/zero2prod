use actix_web::{web, HttpResponse};
use chrono::Utc;
use sqlx::PgPool;
use tracing::Instrument;
use uuid::Uuid;


#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String,
}


#[tracing::instrument(
    name = "Adding a new subscriber",
    skip(form, pool),
    fields(
        request_id = %Uuid::new_v4(),
        subscriber_email = %form.email,
        subscriber_name = %form.name
    )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
    // Spans, like logs, have an associated level, this case we are using 'info'
    // let request_span = tracing::info_span!(
    //     "Saving new subscriber details in the database"
    // );

    // // Using 'enter' in an async function is a recipe for disaster. for now we will use it
    // let _request_span_guard = request_span.enter();
    // Request span is dropped after the end of the subscribe function, that is when we 'exit' the span
    let query_span = tracing::info_span!("Saving new subscriber details in the database",);

    match sqlx::query!(
        r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        "#,
        Uuid::new_v4(),
        form.email,
        form.name,
        Utc::now()
    )
    .execute(pool.get_ref())
    .instrument(query_span)
    .await
    {
        Ok(_) => {
            tracing::info!(
                "request_id {} - New subscriber details have been saved.",
                request_id
            );
            HttpResponse::Ok().finish()
        }
        Err(e) => {
            tracing::error!(
                "request_id {} - Failed to execute query: {:?}",
                request_id,
                e
            );
            HttpResponse::InternalServerError().finish()
        }
    }
}
