use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use crate::doc::models::{Document, CreateDocument, UpdateDocument};
use crate::auth::models::AuthUser;
use uuid::Uuid;

#[axum::debug_handler]
pub async fn create_doc(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
    Json(payload): Json<CreateDocument>,
    
) -> Result<Json<Document>, String> {
    let owner_id: i32 = _user.user_id;

    let doc = sqlx::query_as!(
        Document,
        r#"
        INSERT INTO documents (owner_id, title, content)
        VALUES ($1, $2, $3)
        RETURNING id, owner_id, title, content, created_at, updated_at
        "#,
        owner_id,
        payload.title,
        payload.content.unwrap_or_default()
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(doc))
}

#[axum::debug_handler]
pub async fn get_doc(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
    Path(doc_id): Path<Uuid>,
) -> Result<Json<Document>, String> {
    let doc = sqlx::query_as!(
        Document,
        r#"
        SELECT id, owner_id, title, content, created_at, updated_at
        FROM documents
        WHERE id = $1
        "#,
        doc_id
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(doc))
}

#[axum::debug_handler]
pub async fn update_doc(
    State(pool): State<PgPool>,
    Path(doc_id): Path<Uuid>,
    AuthUser(_user): AuthUser,
    Json(payload): Json<UpdateDocument>,
) -> Result<Json<Document>, String> {
    let doc = sqlx::query_as!(
        Document,
        r#"
        UPDATE documents
        SET title = COALESCE($2, title),
            content = COALESCE($3, content),
            updated_at = now()
        WHERE id = $1
        RETURNING id, owner_id, title, content, created_at, updated_at
        "#,
        doc_id,
        payload.title,
        payload.content
    )
    .fetch_one(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(doc))
}

#[axum::debug_handler]
pub async fn delete_doc(
    State(pool): State<PgPool>,
    AuthUser(_user): AuthUser,
    Path(doc_id): Path<Uuid>,
) -> Result<String, String> {
    sqlx::query!(
        r#"
        DELETE FROM documents
        WHERE id = $1
        "#,
        doc_id
    )
    .execute(&pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(format!("Document {} deleted", doc_id))
}
