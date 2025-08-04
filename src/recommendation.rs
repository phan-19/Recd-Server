use sqlx::SqlitePool;

pub async fn generate_recommendations(
    pool: &SqlitePool,
    user_id: i64,
    medium: String,
) -> Result<Vec<i64>, sqlx::Error> {
    //followed users liked
    let sql = format!(
        "SELECT reviews.media_id FROM reviews 
        INNER JOIN following_user ON following_user.followed_id = reviews.user_id
        INNER JOIN media ON reviews.media_id = media.media_id
        WHERE following_user.follower_id = $1 
        AND reviews.rating >= 4{}",
        match medium.as_str() {
            "any" => "".to_string(),
            _ => format!(" AND media.medium = '{}'", medium),
        }
    );
    let followed_liked: Vec<i64> = match sqlx::query_scalar::<_, i64>(&sql)
        .bind(user_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    //followed users follow
    let sql = format!(
        "SELECT following_media.media_id FROM following_media 
        INNER JOIN following_user ON following_user.followed_id = following_media.user_id
        INNER JOIN media ON following_media.media_id = media.media_id
        WHERE following_user.follower_id = $1{}",
        match medium.as_str() {
            "any" => "".to_string(),
            _ => format!(" AND media.medium = '{}'", medium),
        }
    );
    let followed_followed: Vec<i64> = match sqlx::query_scalar::<_, i64>(&sql)
        .bind(user_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    //liked overall
    let sql = format!(
        "SELECT reviews.media_id FROM reviews 
        INNER JOIN media ON reviews.media_id = media.media_id
        WHERE reviews.user_id != $1
        AND reviews.rating >= 4{}",
        match medium.as_str() {
            "any" => "".to_string(),
            _ => format!(" AND media.medium = '{}'", medium),
        }
    );
    let liked_overall: Vec<i64> = match sqlx::query_scalar::<_, i64>(&sql)
        .bind(user_id)
        .fetch_all(pool)
        .await
    {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    let mut recommendations: Vec<i64> = Vec::new();
    for id in followed_liked.iter() {
        if !recommendations.contains(id) {
            recommendations.push(*id);
        }
    }
    for id in followed_followed.iter() {
        if !recommendations.contains(id) {
            recommendations.push(*id);
        }
    }
    for id in liked_overall.iter() {
        if !recommendations.contains(id) {
            recommendations.push(*id);
        }
    }

    Ok(recommendations)
}
