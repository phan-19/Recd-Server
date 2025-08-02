use sqlx::{query, query_scalar, Row, SqlitePool};

pub async fn add_rating(pool: &SqlitePool, media_id: i64, rating: i64) -> Result<(), sqlx::Error> {
    let sql = "SELECT reivew_count, average_rating FROM media WHERE media_id = $1";
    match query(&sql).bind(media_id).fetch_one(pool).await {
        Ok(result) => {
            let review_count: i64 = result.get("review_count");
            let average_rating: i64 = result.get("average_rating");

            let new_avg: i64 = (average_rating * review_count + rating) / (review_count + 1);
            let sql = "UPDATE media SET review_count = $1, average_rating = $2 WHERE media_id = $3";
            match query(&sql)
                .bind(review_count + 1)
                .bind(new_avg)
                .bind(media_id)
                .execute(pool)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

pub async fn recalc_media(pool: &SqlitePool, media_id: i64) -> Result<(), sqlx::Error> {
    let sql = "SELECT rating FROM reviews WHERE media_id = $1";
    match query_scalar::<_, i64>(&sql)
        .bind(media_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => {
            let review_count: i64 = result.iter().count() as i64;
            let average_rating: f64 = result.iter().sum::<i64>() as f64 / review_count as f64;

            let sql = "UPDATE media SET review_count = $1, average_rating = $2 WHERE media_id = $3";
            match query(&sql)
                .bind(review_count)
                .bind(average_rating)
                .bind(media_id)
                .execute(pool)
                .await
            {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}
