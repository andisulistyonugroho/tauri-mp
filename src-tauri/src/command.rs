use crate::db::connect_db;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct PostsMetadata {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub category: String,
    pub status: String,
    pub published_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub author: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CreatePostPayload {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub status: String,
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub body: String,
}

#[tauri::command]
pub fn get_posts() -> Result<Vec<PostsMetadata>, String> {
    let conn = connect_db().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT id, slug, title, category, status, author, published_at, created_at, updated_at FROM posts_metadata LIMIT 1")
        .map_err(|e| e.to_string())?;

    let data = stmt
        .query_map([], |row| {
            Ok(PostsMetadata {
                id: row.get(0)?,
                slug: row.get(1)?,
                title: row.get(2)?,
                category: row.get(3)?,
                status: row.get(4)?,
                author: row.get(5)?,
                published_at: row.get(6)?,
                created_at: row.get(7)?,
                updated_at: row.get(8)?,
            })
        })
        .map_err(|e| e.to_string())?;

    let mut posts_metadata = Vec::new();
    for row in data {
        posts_metadata.push(row.map_err(|e| e.to_string())?)
    }

    Ok(posts_metadata)
}

#[tauri::command]
pub fn create_post(payload: CreatePostPayload) -> Result<i64, String> {
    let mut conn = connect_db().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    tx.execute(
        "INSERT INTO posts_metadata (slug, title, category, author, status) VALUES (?1,?2,?3,?4,?5)",
        (
            &payload.slug,
            &payload.title,
            &payload.category,
            &payload.author,
            &payload.status
        ),
    )
    .map_err(|e| e.to_string())?;

    let new_id = tx.last_insert_rowid();

    tx.execute(
        "INSERT INTO posts_content (post_id, body) VALUES (?1,?2)",
        (new_id, &payload.body),
    )
    .map_err(|e| e.to_string())?;

    for tag in &payload.tags {
        let trimmed_tag: &str = tag.trim();
        tx.execute(
            "INSERT INTO posts_tags (post_id, tag) VALUES (?1,?2)",
            (new_id, trimmed_tag),
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(new_id)
}
