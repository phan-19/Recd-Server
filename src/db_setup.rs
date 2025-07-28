use sqlx::{query, SqlitePool};

pub async fn db_setup(pool: &SqlitePool) -> Result<sqlx::sqlite::SqliteQueryResult, sqlx::Error> {
    let qry = "
        PRAGMA foreign_keys = ON; 

        CREATE TABLE IF NOT EXISTS users (
            user_id INTEGER PRIMARY KEY NOT NULL,
            username STRING NOT NULL,
            password STRING NOT NULL,
            bio STRING,
            profile_pic BLOB,
            UNIQUE(username)
        );

        CREATE TABLE IF NOT EXISTS media (
            media_id INTEGER PRIMARY KEY NOT NULL,
            media_name STRING NOT NULL,
            medium STRING NOT NULL,
            description STRING NOT NULL,
            image BLOB,
            UNIQUE(media_name)
        );

        CREATE TABLE IF NOT EXISTS reviews (
            review_id INTEGER PRIMARY KEY NOT NULL,
            user_id INTEGER,
            media_id INTEGER,
            rating INTEGER NOT NULL,
            review_txt STRING,
            posted_at STRING NOT NULL DEFAULT current_timestamp,
            FOREIGN KEY(user_id) REFERENCES users(user_id),
            FOREIGN KEY(media_id) REFERENCES media(media_id)
        );

        CREATE TABLE IF NOT EXISTS following_user (
            follower_id INTEGER,
            followed_id INTEGER,
            FOREIGN KEY(follower_id) REFERENCES users(user_id),
            FOREIGN KEY(followed_id) REFERENCES users(user_id),
            UNIQUE(follower_id, followed_id)
        );

        CREATE TABLE IF NOT EXISTS following_media (
            user_id INTEGER,
            media_id INTEGER,
            FOREIGN KEY(user_id) REFERENCES users(user_id),
            FOREIGN KEY(media_id) REFERENCES media(media_id),
            UNIQUE(user_id, media_id)
        );

        CREATE TABLE IF NOT EXISTS tags (
            media_id INTEGER,
            tag STRING NOT NULL,
            FOREIGN KEY(media_id) REFERENCES media(media_id),
            UNIQUE(media_id, tag)
        );

        CREATE TABLE IF NOT EXISTS todo (
            user_id INTEGER,
            media_id INTEGER,
            status STRING NOT NULL DEFAULT 'todo',
            FOREIGN KEY(user_id) REFERENCES users(user_id),
            FOREIGN KEY(media_id) REFERENCES media(media_id),
            UNIQUE(user_id, media_id)
        );
    ";
    query(&qry).execute(pool).await
}
