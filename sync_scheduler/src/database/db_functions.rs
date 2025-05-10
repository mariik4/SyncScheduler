use sqlx::postgres::Postgres;
use sqlx::types;
use sqlx::Pool;
use sqlx::{Connection, Error, PgConnection};
use std::env;

#[derive(sqlx::FromRow, Debug)]
pub struct Event {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub event_type: String,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub priority: i64,
    pub postpone: i64,
    pub user_id: Uuid,
}

impl Event {
    // constructor to create new static event
    // with predefined fields: id, event_type, priority, postpone
    pub fn new_static(
        name: String,
        description: Option<String>,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        user_id: Uuid,
    ) -> Self {
        Event {
            id: Uuid::new_v4(),
            name,
            description,
            event_type: "static".into(),
            start_time,
            end_time,
            priority: 5,
            postpone: 0,
            user_id,
        }
    }

    // constructor to create new dynamic event
    // with predefined fields: id, event_type, postpone
    pub fn new_dynamic(
        name: String,
        description: Option<String>,
        start_time: NaiveDateTime,
        end_time: NaiveDateTime,
        priority: i64,
        user_id: Uuid,
    ) -> Self {
        Event {
            id: Uuid::new_v4(),
            name,
            description,
            event_type: "dynamic".into(),
            start_time,
            end_time,
            priority,
            postpone: 0,
            user_id,
        }
    }
}

#[derive(sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub password: String,
}

pub async fn get_events_in_day(user_id: Uuid, date: NaiveDate) -> Result<Vec<Event>, Error> {
    let url = env::var("DB_URL").unwrap(); //6543

    println!("DB_URL: {}", url);
    let pool = Pool::<Postgres>::connect(&url).await?;

    let start_time = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();

    let dt_start = NaiveDateTime::new(date, start_time);

    let end_time = NaiveTime::from_hms_milli_opt(23, 59, 59, 999).unwrap();
    let dt_end = NaiveDateTime::new(date, end_time);

    let events = sqlx::query_as(
        "SELECT id, name, description, event_type, start_time, end_time, priority, postpone, user_id
        FROM events WHERE ( start_time <= $3 AND end_time >= $2) AND user_id = $1",
    )
    .bind(user_id)
    .bind(dt_start)
    .bind(dt_end)
    .fetch_all(&pool)
    .await?;

    Ok(events)
}

pub async fn get_events_in_range(
    user_id: Uuid,
    start_day: NaiveDate,
    end_day: NaiveDate,
) -> Result<Vec<Event>, Error> {
    let url = std::env::var("DB_URL").unwrap(); //6543

    let pool = Pool::<Postgres>::connect(&url).await?;

    let start_time = NaiveTime::from_hms_milli_opt(0, 0, 0, 0).unwrap();

    let dt_start = NaiveDateTime::new(start_day, start_time);

    let end_time = NaiveTime::from_hms_milli_opt(23, 59, 59, 999).unwrap();
    let dt_end = NaiveDateTime::new(end_day, end_time);

    let events = sqlx::query_as(
        "SELECT id, name, description, event_type, start_time, end_time, priority, postpone, user_id
        FROM events WHERE ( start_time <= $3 AND end_time >= $2) AND user_id = $1",
    )
    .bind(user_id)
    .bind(dt_start)
    .bind(dt_end)
    .fetch_all(&pool)
    .await?;

    Ok(events)
}

pub async fn create_new_user_on_db(
    username: String,
    first_name: String,
    last_name: String,
    password: String,
) -> Result<User, Error> {
    let url = std::env::var("DB_URL").unwrap();

    let pool = Pool::<Postgres>::connect(&url).await?;
    let uuid = Uuid::new_v4();
    let newUser = User {
        id: uuid,
        username: username.clone(),
        first_name: first_name.clone(),
        last_name: last_name.clone(),
        password: password.clone(),
    };

    sqlx::query(
        "INSERT INTO users (id, username, first_name, last_name, password )
        VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(uuid)
    .bind(username)
    .bind(first_name)
    .bind(last_name)
    .bind(password)
    .execute(&pool)
    .await?;

    Ok(newUser)
}

pub async fn add_event_to_db(event: &Event) -> Result<(), Error> {
    let url = std::env::var("DB_URL").expect("DB_URL must be set to connect to the database");

    let pool = Pool::<Postgres>::connect(&url).await?;

    sqlx::query(
        "INSERT INTO events (id, name, description, event_type, start_time, end_time, priority, postpone, user_id)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
    )
    .bind(event.id)
    .bind(&event.name)
    .bind(&event.description)
    .bind(&event.event_type)
    .bind(event.start_time)
    .bind(event.end_time)
    .bind(event.priority)
    .bind(event.postpone)
    .bind(event.user_id)
    .execute(&pool)
    .await?;

    // let event = sqlx::query("select (1) as id, 'Herp Derpinson' as name")
    Ok(())
}
