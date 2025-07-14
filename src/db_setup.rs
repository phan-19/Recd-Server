use sqlx::{query, SqlitePool};

pub async fn db_setup(pool: &SqlitePool) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let qry = "
        PRAGMA foreign_keys = ON; 

        CREATE TABLE IF NOT EXSIST users (
            user_id INTEGER PRIMARY KEY NOT NULL,
            username STRING NOT NULL,
            password STRING NOT NULL,
            bio STRING,
            profile_pic BLOB
        );

        CREATE TABLE IF NOT EXSIST media (
            media_id INTEGER PRIMARY KEY NOT NULL,
            media_name STRING NOT NULL,
            medium STRING NOT NULL,
            description STRING NOT NULL
            image BLOB
        );

        CREATE TABLE IF NOT EXSIST reviews (
            review_id INTEGER PRIMARY KEY NOT NULL,
            user_id INTEGER,
            media_id INTEGER,
            rating INTEGER NOT NULL,
            review_txt STRING,
            posted_at STRING NOT NULL DEFAULT current_timestamp,
            FOREIGN KEY(user_id) REFERENCES users(user_id),
            FOREIGN KEY(media_id) REFERENCES media(media_id)
        );

        CREATE TABLE IF NOT EXSIST following_user (
            follower_id INTEGER,
            followed_id INTEGER,
            FOREIGN KEY(follower_id) REFERENCES users(user_id),
            FOREIGN KEY(followed_id) REFERENCES users(user_id),
            UNIQUE(follower_id, followed_id)
        );

        CREATE TABLE IF NOT EXSIST following_media (
            follower_id INTEGER,
            media_idINTEGER,
            FOREIGN KEY(follower_id) REFERENCES users(user_id),
            FOREIGN KEY(media_id) REFERENCES users(media_id),
            UNIQUE(follower_id, media_id)
        );
    ";
    query(&qry).execute(pool).await
}
