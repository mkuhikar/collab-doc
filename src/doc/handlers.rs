use axum::{
    extract::{Path, State},
    Json,
};
use sqlx::PgPool;
use crate::doc::models::{Collaborator, Document, CreateDocument, UpdateDocument, Owner, Role};
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

#[axum::debug_handler]
    pub async fn share_doc(
        State(pool): State<PgPool>,
        AuthUser(_user): AuthUser,
        Path(doc_id): Path<Uuid>,
        Json(payload): Json<crate::doc::models::ShareRequest>,
    ) -> Result<String, String> {
        // Check if the document exists and the user is the owner
        let owner = sqlx::query_as!(
            Owner,
            r#"
            SELECT owner_id FROM documents WHERE id = $1
            "#,
            doc_id
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?;
        if owner.owner_id != _user.user_id {
            return Err("Only the owner can share the document".to_string());
        }
        //Check if the user already has access to the doc
        let existing = sqlx::query_as!(
            Collaborator,
            r#"
            SELECT * FROM doc_collaborators WHERE doc_id = $1 AND user_id = $2
            "#,
            doc_id,
            payload.user_id
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;
        if existing.is_some() {
            return Err("User already has access to the document".to_string());
        }

        
        // Add collaborator
        let collaborator = sqlx::query_as!(
            Collaborator,
            r#"
            INSERT INTO doc_collaborators (doc_id, user_id, role)
            VALUES ($1, $2, $3)
            ON CONFLICT (doc_id, user_id) DO UPDATE SET role = EXCLUDED.role
            RETURNING user_id, role
            "#,
            doc_id,
            payload.user_id,
            payload.role
        )
        .fetch_one(&pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(format!("User {} added as {} to document {}", payload.user_id, payload.role, doc_id))
    }

//get user docs
#[axum::debug_handler]
    pub async fn get_user_docs(
        State(pool): State<PgPool>,
        AuthUser(_user): AuthUser,
    ) -> Result<Json<Vec<Document>>, String> {
        let docs = sqlx::query_as!(
            Document,
            r#"
            SELECT d.id, d.owner_id, d.title, d.content, d.created_at, d.updated_at
            FROM documents d
            LEFT JOIN doc_collaborators dc ON d.id = dc.doc_id
            WHERE d.owner_id = $1 OR dc.user_id = $1
            "#,
            _user.user_id
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Json(docs))
    }

//get collaborators for a doc
#[axum::debug_handler]
    pub async fn get_doc_collaborators(
        State(pool): State<PgPool>,
        AuthUser(_user): AuthUser,
        Path(doc_id): Path<Uuid>,
    ) -> Result<Json<Vec<crate::doc::models::Collaborator>>, String> {
        // Check if the user has access to the document
        let access = sqlx::query_as!(
            Collaborator,
            r#"
            SELECT * FROM doc_collaborators WHERE doc_id = $1 AND user_id = $2
            "#,
            doc_id,
            _user.user_id
        )
        .fetch_optional(&pool)
        .await
        .map_err(|e| e.to_string())?;
        if access.is_none() {
            return Err("You do not have access to this document".to_string());
        }
        let collaborators = sqlx::query_as!(
            Collaborator,
            r#"
            SELECT doc_id, user_id, role  FROM doc_collaborators WHERE doc_id = $1
            "#,
            doc_id
        )
        .fetch_all(&pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(Json(collaborators))
    }