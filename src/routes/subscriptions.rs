use actix_web::{web, HttpResponse};
use sqlx::PgPool;
use chrono::Utc;
use uuid::Uuid;
use unicode_segmentation::UnicodeSegmentation;
use crate::domain::{NewSubscriber, SubscriberName, SubscriberEmail};
use std::convert::TryFrom;
use std::convert::TryInto;

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}


impl TryFrom<FormData> for NewSubscriber {
  type Error = String;

  fn try_from(value: FormData) -> Result<Self, Self::Error> {
    let name = SubscriberName::parse(value.name)?;
    let email = SubscriberEmail::parse(value.email)?;
    Ok(Self { email, name })
  }
}

#[tracing::instrument(
  name = "Adding a new subscriber",
  skip(form, pool),
  fields(
    subscriber_email = %form.email,
    subscriber_name = %form.name
  )
)]
pub async fn subscribe(form: web::Form<FormData>, pool: web::Data<PgPool>) -> HttpResponse {
  let new_subscriber = match form.0.try_into() {
    Ok(subscriber) => subscriber,
    Err(_) => return HttpResponse::BadRequest().finish(),
  };

  match insert_subscriber(&pool, &new_subscriber).await {
    Ok(_) => HttpResponse::Ok().finish(),
    Err(_) => HttpResponse::InternalServerError().finish()
  }
}

pub fn is_valid_name(s: &str) -> bool {
  let is_empty_or_whitespace = s.trim().is_empty();
  let is_too_long = s.graphemes(true).count() > 256;
  let forbidden_characters = ['/', '(', ')', '"', '<', '>', '\\', '{', '}'];
  let contains_forbidden_characters = s.chars().any(|g| forbidden_characters.contains(&g));

  !(is_empty_or_whitespace || is_too_long || contains_forbidden_characters)
}

#[tracing::instrument(
  name = "Saving new subscriber details"
  skip(new_subscriber, pool)
)]
pub async fn insert_subscriber(pool: &PgPool, new_subscriber: &NewSubscriber) -> Result<(), sqlx::Error> {
  sqlx::query!(
    r#"
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
    "#,
    Uuid::new_v4(),
    new_subscriber.email.as_ref(),
    new_subscriber.name.as_ref(),
    Utc::now()
  )
  .execute(pool)
  .await
  .map_err(|e| {
    tracing::error!("Failed to execute query: {:?}", e);
    e
  })?;
  Ok(())
}