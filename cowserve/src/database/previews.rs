use super::{cowfiles::DbCowfileDescriptor, Client};
use anyhow::Result;
use uriparse::URI;

#[allow(dead_code)]
#[derive(sqlx::FromRow)]
struct DbPreview {
    pub id: String,
    pub source: String,
    pub cowfile_id: String,
    pub created_at: chrono::NaiveDateTime,
    pub updated_at: chrono::NaiveDateTime,
    pub deleted_at: Option<chrono::NaiveDateTime>,
}

pub async fn get_unresolved(pool: &Client) -> Result<Vec<DbCowfileDescriptor>> {
    let data = sqlx::query_as!(
        DbCowfileDescriptor,
        "
SELECT
  *
FROM
  cowsay.cowfiles
WHERE
  cowsay.cowfiles.id NOT IN (
    SELECT
      cowsay.previews.cowfile_id
    FROM
      cowsay.previews
    WHERE
      cowsay.previews.deleted_at IS NULL
  );"
    )
    .fetch_all(pool)
    .await?;

    Ok(data)
}

pub async fn get_preview(pool: &Client, cow_id: &String) -> Result<DbPreview> {
    let data = sqlx::query_as!(
        DbPreview,
        "
SELECT
  *
FROM
  cowsay.previews
WHERE
  cowsay.previews.cowfile_id = $1
  AND deleted_at IS NULL;
  ", cow_id
    )
    .fetch_one(pool)
    .await?;

    Ok(data)
}

pub async fn save_preview<'a>(pool: &Client, cow_id: &String, source: &String) -> Result<()> {
    sqlx::query!(
        "
INSERT INTO
  cowsay.previews (cowfile_id, source)
VALUES
  ($1, $2);
 ",
        cow_id,
        source
    )
    .execute(pool)
    .await?;

    Ok(())
}
