use crate::db::connect_db;
use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Serialize)]
pub struct PostsMetadata {
    pub id: i64,
    pub slug: String,
    pub title: String,
    pub category: String,
    pub status: String,
    pub tags: String,
    pub published_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub author: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreatePostPayload {
    pub slug: String,
    pub title: String,
    pub category: String,
    pub status: String,
    pub tags: Vec<String>,
    pub author: Option<String>,
    pub body: String,
    pub published_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PostContent {
    pub post_id: i64,
    pub body: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PostDetail {
    pub id: i64,

    #[serde(flatten)] // merge fields
    pub metadata: CreatePostPayload,
}

#[tauri::command]
pub fn get_posts() -> Result<Vec<PostsMetadata>, String> {
    let conn = connect_db().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare("SELECT a.id, a.slug, a.title, a.category, a.status, a.author, a.published_at, a.created_at, a.updated_at,
            (SELECT group_concat(b.tag) FROM posts_tags b WHERE b.post_id = a.id) AS tags
            FROM posts_metadata a order by a.id desc")
        .map_err(|e| e.to_string())?;

    let data = stmt
        .query_map([], |row| {
            Ok(PostsMetadata {
                id: row.get("id")?,
                slug: row.get("slug")?,
                title: row.get("title")?,
                category: row.get("category")?,
                status: row.get("status")?,
                author: row.get("author")?,
                published_at: row.get("published_at")?,
                created_at: row.get("created_at")?,
                updated_at: row.get("updated_at")?,
                tags: row.get("tags")?,
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
        "INSERT INTO posts_metadata (slug, title, category, author, status, published_at) VALUES (?1,?2,?3,?4,?5,?6)",
        (
            &payload.slug,
            &payload.title,
            &payload.category,
            &payload.author,
            &payload.status,
            &payload.published_at
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

#[tauri::command]
pub fn detail_post(id: i64) -> Result<PostContent, String> {
    let conn = connect_db().map_err(|e| e.to_string())?;
    let mut stmt = conn
        .prepare(
            "SELECT post_id,body
        FROM posts_content WHERE post_id = ?1",
        )
        .map_err(|e| e.to_string())?;

    let data = stmt
        .query_one([id], |row| {
            Ok(PostContent {
                post_id: row.get("post_id")?,
                body: row.get("body")?,
            })
        })
        .map_err(|e| e.to_string())?;

    Ok(data)
}

#[tauri::command]
pub fn update_post(payload: PostDetail) -> Result<i64, String> {
    let mut conn = connect_db().map_err(|e| e.to_string())?;
    let tx = conn.transaction().map_err(|e| e.to_string())?;

    tx.execute(
        "UPDATE posts_metadata SET slug = ?1, title = ?2, category = ?3, author = ?4, status = ?5, published_at = ?6 WHERE id = ?7",
        (
            &payload.metadata.slug,
            &payload.metadata.title,
            &payload.metadata.category,
            &payload.metadata.author,
            &payload.metadata.status,
            &payload.metadata.published_at,
            &payload.id,
        ),
    )
    .map_err(|e| e.to_string())?;

    tx.execute(
        "UPDATE posts_content SET body = ?2 WHERE post_id = ?1",
        (&payload.id, &payload.metadata.body),
    )
    .map_err(|e| e.to_string())?;

    tx.execute("DELETE FROM posts_tags WHERE post_id = ?1", [&payload.id])
        .map_err(|e| e.to_string())?;

    for tag in &payload.metadata.tags {
        let trimmed_tag: &str = tag.trim();
        tx.execute(
            "INSERT INTO posts_tags (post_id, tag) VALUES (?1,?2)",
            (&payload.id, trimmed_tag),
        )
        .map_err(|e| e.to_string())?;
    }

    tx.commit().map_err(|e| e.to_string())?;

    Ok(payload.id)
}

#[tauri::command]
pub fn upload_post() -> Result<String, String> {
    let make_target = "uploadpost";
    let makefile_dir = "/Users/asn/Sites/MP/backend-mp/";

    let output = Command::new("make")
        .arg("-C")
        .arg(makefile_dir)
        .arg(make_target)
        .output()
        .map_err(|e| format!("Failed to spawn 'make' command: {}", e))?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        Ok(format!(
            "Uploading parquet completed successfully!\n\n{}",
            stdout
        ))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        Err(format!(
            "Makefile execution failed (Exit Code {}):\n{}",
            output.status.code().unwrap_or(-1),
            stderr
        ))
    }
}
