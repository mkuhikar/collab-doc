use axum::{
    extract::{Path, State},
    Json,
};
use anyhow::{Result, anyhow, bail};
use dashmap::DashMap;
use sqlx::PgPool;
use crate::doc::models::{ClientMessage, CreateDocument,DocumentWithRole, Document, Op, Owner, Role, ServerMessage,Collaborator, UpdateDocument,ShareRequest};
use crate::auth::models::AuthUser;
use uuid::Uuid;
use axum::{
    extract::{WebSocketUpgrade, ws::{Message, WebSocket}},
    response::IntoResponse,
    routing::{get, post},
};
use std::{sync::Arc, time::Duration};

use futures::{sink::SinkExt, stream::StreamExt};
use tokio::{sync::{broadcast, Mutex}, time::Instant};

use tracing::{debug, error, info, warn};
use crate::doc::sessions::{DocSession,Sessions};


#[axum::debug_handler]
pub async fn create_doc(
    State(state): State<AppState>,
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
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(doc))
}

#[axum::debug_handler]
pub async fn get_doc(
    State(state): State<AppState>,
    AuthUser(_user): AuthUser,
    Path(doc_id): Path<Uuid>,
) -> Result<Json<DocumentWithRole>, String> {
    // Get the document
    let doc = sqlx::query_as!(
        Document,
        r#"
        SELECT id, owner_id, title, content, created_at, updated_at
        FROM documents
        WHERE id = $1
        "#,
        doc_id
    )
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    if doc.owner_id == _user.user_id{
        let role = Role::Owner;
         return Ok(Json(DocumentWithRole {
        id: doc.id,
        owner_id: doc.owner_id,
        title: doc.title,
        content: doc.content,
        created_at: doc.created_at,
        updated_at: doc.updated_at,
        role,
    }))
    }

    // Get the user's role
    let role = sqlx::query_scalar!(
        r#"
        SELECT role as "role: Role"
        FROM doc_collaborators
        WHERE doc_id = $1 AND user_id = $2
        "#,
        doc_id,
        _user.user_id
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| e.to_string())?
    .unwrap_or(Role::Reader); // Default if not collaborator

    Ok(Json(DocumentWithRole {
        id: doc.id,
        owner_id: doc.owner_id,
        title: doc.title,
        content: doc.content,
        created_at: doc.created_at,
        updated_at: doc.updated_at,
        role,
    }))
}

#[axum::debug_handler]
pub async fn update_doc(
    State(state): State<AppState>,
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
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(doc))
}

#[axum::debug_handler]
pub async fn delete_doc(
    State(state): State<AppState>,
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
    .execute(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(format!("Document {} deleted", doc_id))
}

#[axum::debug_handler]
pub async fn share_doc(
    State(state): State<AppState>,
    AuthUser(_user): AuthUser,
    Path(doc_id): Path<Uuid>,
    Json(payload): Json<ShareRequest>,
) -> Result<String, String> {
    // ✅ 1. Verify that the document exists and user is the owner
    let owner:Owner = sqlx::query_as::<_, Owner>(
        r#"
        SELECT owner_id
        FROM documents
        WHERE id = $1
        "#
    )
    .bind(doc_id)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    if owner.owner_id != _user.user_id {
        return Err("Only the owner can share the document".to_string());
    }

    // ✅ 2. Get the user ID by email
    let user = sqlx::query!(
        r#"
        SELECT id FROM users WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    let target_user_id = match user {
        Some(u) => u.id,
        None => return Err(format!("No user found with email {}", payload.email)),
    };

    // ✅ 3. Check if the user already has access
    let existing = sqlx::query_as::<_, Collaborator>(
        r#"
        SELECT doc_id, user_id, role
        FROM doc_collaborators
        WHERE doc_id = $1 AND user_id = $2
        "#
    )
    .bind(doc_id)
    .bind(target_user_id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    if existing.is_some() {
        return Err("User already has access to the document".to_string());
    }

    // ✅ 4. Add collaborator
    let collaborator = sqlx::query_as::<_, Collaborator>(
        r#"
        INSERT INTO doc_collaborators (doc_id, user_id, role)
        VALUES ($1, $2, $3)
        ON CONFLICT (doc_id, user_id) DO UPDATE SET role = EXCLUDED.role
        RETURNING doc_id, user_id, role
        "#
    )
    .bind(doc_id)
    .bind(target_user_id)
    .bind(payload.role)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(format!(
        "User {} added as {:?} to document {}",
        payload.email,
        collaborator.role,
        doc_id
    ))
}

//get user docs
#[axum::debug_handler]
    pub async fn get_user_docs(
        State(state): State<AppState>,
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
        .fetch_all(&state.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(Json(docs))
    }

// get collaborators for a doc
#[axum::debug_handler]
pub async fn get_doc_collaborators(
    State(state): State<AppState>,
    AuthUser(_user): AuthUser,
    Path(doc_id): Path<Uuid>,
) -> Result<Json<Vec<crate::doc::models::Collaborator>>, String> {
    // Check if the user has access to the document
    let collaborators = sqlx::query_as::<_, crate::doc::models::Collaborator>(
        r#"
        SELECT dc.doc_id, dc.user_id, dc.role
        FROM doc_collaborators AS dc, documents AS d
        WHERE d.owner_id = $1 
          AND d.id = dc.doc_id
          AND dc.doc_id = $2
        "#
    )
    .bind(_user.user_id)
    .bind(doc_id)
    .fetch_all(&state.pool)
    .await
    .map_err(|e| e.to_string())?;

    Ok(Json(collaborators))
}

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub sessions: Arc<DashMap<Uuid, Arc<Mutex<DocSession>>>>,
}

pub async fn ws_handler(
    Path(doc_id): Path<Uuid>,
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, doc_id, state))
}

/// Main socket handler per connection
async fn handle_socket(mut socket: WebSocket, doc_id: Uuid, state: AppState) {
    info!("WS connection for doc {}", doc_id);

    // Ensure session exists (load from DB if necessary)
    let session_arc = ensure_session(&state, doc_id).await;

    // Subscribe to broadcast channel
    let rx = {
        let s = session_arc.lock().await;
        s.broadcaster.subscribe()
    };

    // Split socket into sender and receiver halves
    let (socket_sender, mut socket_receiver) = socket.split();
    let sender = Arc::new(Mutex::new(socket_sender));

    // Spawn task to forward broadcast messages to the client
    let tx = sender.clone();
    let mut rx_clone = rx.resubscribe();
    let send_task = tokio::spawn(async move {
        while let Ok(msg) = rx_clone.recv().await {
            if let Ok(json) = serde_json::to_string(&msg) {
                let mut tx_guard = tx.lock().await;
                if tx_guard.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    });

    // Task to handle messages coming *from* the client
    let session_for_recv = session_arc.clone();
    let recv_task = tokio::spawn(async move {
        while let Some(Ok(message)) = socket_receiver.next().await {
    match message {
        Message::Text(txt) => {
            debug!("📨 Raw WebSocket message received: {}", txt);

            // Try to peek at the message type before deserialization
            if let Ok(json_val) = serde_json::from_str::<serde_json::Value>(&txt) {
                if let Some(msg_type) = json_val.get("type").and_then(|v| v.as_str()) {
                    match msg_type {
                        "join" => {
                            info!("👋 Client joined doc {}: {:?}", doc_id, json_val);
                            // No client_version expected here
                            continue;
                        }
                        "doc_update" => {
                            match serde_json::from_value::<ClientMessage>(json_val) {
                                Ok(client_msg) => {
                                    debug!(
                                        "🧠 Applying client op to session {}: {:?}",
                                        client_msg.client_id, client_msg.op
                                    );

                                    if let Err(e) = apply_client_op(session_for_recv.clone(), client_msg.clone()).await {
                                        error!("apply op error: {:?}", e);
                                    }

                                   
                                }
                                Err(err) => {
                                    error!("❌ invalid doc_update message: {:?}", err);
                                }
                            }
                        }
                        _ => {
                            warn!("⚠️ Unknown message type: {}", msg_type);
                        }
                    }
                }
            } else {
                error!("❌ Invalid JSON in WS message: {}", txt);
            }
        }
        Message::Close(_) => break,
        _ => {}
    }
}
    });

    // Wait for either task to finish
    let _ = tokio::join!(send_task, recv_task);
    info!("WS connection closed for doc {}", doc_id);
}

/// Ensure a session exists for a doc_id (load from DB if not)
async fn ensure_session(state: &AppState, doc_id: Uuid) -> Arc<Mutex<DocSession>> {
    if let Some(existing) = state.sessions.get(&doc_id) {
        return existing.value().clone();
    }

    // Load content from DB
    let rec = sqlx::query_scalar!(
        r#"
        SELECT content FROM documents WHERE id = $1
        "#,
        doc_id
    )
    .fetch_one(&state.pool)
    .await
    .unwrap_or_else(|_| "".to_string()); // if missing, start empty

    let session = Arc::new(Mutex::new(DocSession::new(doc_id, rec)));
    state.sessions.insert(doc_id, session.clone());
    session
}
// /// Apply an operation coming from a client: handle versioning and transform
// async fn apply_client_op(session_arc: Arc<Mutex<DocSession>>, client_msg: ClientMessage) -> anyhow::Result<()> {
//     let mut s = session_arc.lock().await;

//     // If client is behind, transform the incoming op against ops from client_version..current_version
//     if client_msg.client_version > s.version {
//         // client ahead? impossible normally; reject
//         anyhow::bail!("client version ahead of server");
//     }

//     let mut incoming = client_msg.op.clone();

//     if client_msg.client_version < s.version {
//         // transform incoming op across subsequent ops
//         let start = client_msg.client_version as usize;
//         let history_slice = &s.history[start..];
//         for prior in history_slice {
//             incoming = transform_op(incoming, prior.clone());
//         }
//     }

//    debug!("✏️ Before apply_op_inplace, content = {:?}", s.content);
// apply_op_inplace(&mut s.content, &incoming)?;
// debug!("✅ After apply_op_inplace, content = {:?}", s.content);

//     // append to history, bump version
//     s.history.push(incoming.clone());
//     s.version += 1;

//     // broadcast to subscribers
//     let srv_msg = ServerMessage {
//         version: s.version,
//         op: incoming,
//         client_id: client_msg.client_id.clone(),

//     };
//     // ignore send errors (no subscribers)
//     let _ = s.broadcaster.send(srv_msg);

//     Ok(())
// }

// async fn apply_client_op(session_arc: Arc<Mutex<DocSession>>, client_msg: ClientMessage) -> anyhow::Result<()> {
//     let mut s = session_arc.lock().await;

//     if client_msg.client_version > s.version {
//         anyhow::bail!("client version ahead of server");
//     }

//     let mut incoming = client_msg.op.clone();

//     if client_msg.client_version < s.version {
//         let start = client_msg.client_version as usize;
//         let history_slice = &s.history[start..];
//         for prior in history_slice {
//             incoming = transform_op(incoming, prior.clone());
//         }
//     }

//     debug!("✏️ Before apply_op_inplace, content = {:?}", s.content);
//     apply_op_inplace(&mut s.content, &incoming)?;
//     debug!("✅ After apply_op_inplace, content = {:?}", s.content);

//     s.history.push(incoming.clone());
//     s.version += 1;

//     let current_version = s.version;
//     let full_content = s.content.clone();
//     let client_id_clone = client_msg.client_id.clone();

//     debug!("🔔 Broadcasting doc_update for doc {} version {}", s.doc_id, current_version);

//     let doc_update = ServerMessage::DocUpdate {
//         version: current_version,
//         content: full_content,
//         client_id: client_id_clone,
//     };

//     // send to subscribers
//     let _ = s.broadcaster.send(doc_update);

//     Ok(())
// }

async fn apply_client_op(session_arc: Arc<Mutex<DocSession>>, client_msg: ClientMessage) -> anyhow::Result<()> {
    let mut s = session_arc.lock().await;

    info!(
        "🧩 Entered apply_client_op: client_version={} server_version={} history_len={}",
        client_msg.client_version, s.version, s.history.len()
    );

   // Use a local variable for client_version so we don't have to mutate client_msg
    let mut client_version = client_msg.client_version as i64;

    if client_version > s.version as i64 {
        warn!(
            "⚠️ Client version ahead of server (client={}, server={}) — clamping to server version",
            client_version, s.version
        );
        // Clamp to server version so we can attempt to transform/apply safely.
        client_version = s.version as i64;
    }


    let mut incoming = client_msg.op.clone();

    if client_msg.client_version < s.version {
        let start = client_msg.client_version as usize;
        info!("🔁 Transforming ops from history slice starting at {}", start);
        let history_slice = &s.history[start..];
        for prior in history_slice {
            incoming = transform_op(incoming, prior.clone());
        }
    }

    debug!("✏️ Before apply_op_inplace (v{}): {}", s.version, s.content);
    if let Err(e) = apply_op_inplace(&mut s.content, &incoming) {
        error!("💥 apply_op_inplace failed: {:?}", e);
        return Err(e.into());
    }
    debug!("✅ After apply_op_inplace (v{}): {}", s.version, s.content);

    s.history.push(incoming.clone());
    s.version += 1;

    let current_version = s.version;
    let full_content = s.content.clone();
    let client_id_clone = client_msg.client_id.clone();

    info!("🔔 Broadcasting doc_update for doc {} version {}", s.doc_id, current_version);

    let doc_update = ServerMessage::DocUpdate {
        version: current_version,
        content: full_content,
        client_id: client_id_clone,
    };

    let _ = s.broadcaster.send(doc_update);

    Ok(())
}


/// Apply an Op to a text buffer
fn apply_op_inplace(buf: &mut String, op: &Op) -> anyhow::Result<()> {
    match op {
        Op::Insert { pos, text } => {
            if *pos > buf.len() {
                anyhow::bail!("insert pos out of bounds");
            }
            // insert at byte index: we must convert char index to byte index if needed
            // we'll assume pos is byte index for simplicity (frontend must send byte index)
            buf.insert_str(*pos, text);
            Ok(())
        }
        Op::Delete { pos, len } => {
            if *pos > buf.len() || pos + len > buf.len() {
                anyhow::bail!("delete range out of bounds");
            }
            buf.replace_range(*pos..pos + len, "");
            Ok(())
        }
    }
}

/// Transform incoming op 'a' against prior op 'b' (OT primitive)
fn transform_op(a: Op, b: Op) -> Op {
    use Op::*;
    match (a, b) {
        // If b inserted before a.pos, shift a.pos right by inserted length
        (Insert { pos: ap, text: at }, Insert { pos: bp, text: bt }) => {
            if bp <= ap {
                Insert { pos: ap + bt.len(), text: at }
            } else {
                Insert { pos: ap, text: at }
            }
        }
        // If b deleted region before or overlapping a.insert pos
        (Insert { pos: ap, text: at }, Delete { pos: bp, len: bl }) => {
            if bp >= ap {
                // deletion after insert pos -> no change
                Insert { pos: ap, text: at }
            } else if bp + bl <= ap {
                // deletion entirely before -> shift left
                Insert { pos: ap - bl, text: at }
            } else {
                // deletion overlaps position -> clamp
                Insert { pos: bp, text: at }
            }
        }

        // Transform delete vs insert
        (Delete { pos: ap, len: al }, Insert { pos: bp, text: bt }) => {
            if bp >= ap + al {
                // insertion after delete range -> no change
                Delete { pos: ap, len: al }
            } else if bp <= ap {
                // insertion before -> shift delete right
                Delete { pos: ap + bt.len(), len: al }
            } else {
                // insertion inside delete range -> increase delete length
                Delete { pos: ap, len: al + bt.len() }
            }
        }

        // Transform delete vs delete
        (Delete { pos: ap, len: al }, Delete { pos: bp, len: bl }) => {
            // We must compute how the second deletion affects the first
            if ap >= bp + bl {
                // b deletes before a -> shift a left
                Delete { pos: ap - bl, len: al }
            } else if ap + al <= bp {
                // b deletes after a -> no change
                Delete { pos: ap, len: al }
            } else {
                // overlapping deletions -> compute remaining range of 'a' after 'b' applied
                // compute overlap
                let a_start = ap;
                let a_end = ap + al;
                let b_start = bp;
                let b_end = bp + bl;
                let new_start = a_start.min(b_start);
                let new_end = a_end.max(b_end);
                // This is simplistic: shrink a by overlap length
                let overlap_start = a_start.max(b_start);
                let overlap_end = a_end.min(b_end);
                let overlap = if overlap_end > overlap_start {
                    overlap_end - overlap_start
                } else {
                    0
                };
                Delete { pos: new_start, len: (new_end - new_start) - overlap }
            }
        }
    }
}

/// Autosave loop: every 10 seconds, persist sessions to DB if changed
pub async fn autosave_loop(state: Arc<AppState>) {
        info!("🕒 Autosave loop started...");

    let mut interval = tokio::time::interval(Duration::from_secs(10));
    loop {
        interval.tick().await;
        info!("sessions in autosave: {}", state.sessions.len());

        // iterate sessions
        for kv in state.sessions.iter_mut() {
            let session_arc = kv.value().clone();
    let state = Arc::clone(&state); // clone the Arc, not the inner value
            // clone fields we need under lock
            tokio::spawn(async move {
                let mut s = session_arc.lock().await;
                    debug!(
                    "💾 Autosaving doc {} with content: {:?}",
                    s.doc_id, s.content
                );
                let elapsed = s.last_saved.elapsed();
                if elapsed >= Duration::from_secs(10) {
                    // save to db
                    if let Err(e) = save_doc_to_db(&state.pool, s.doc_id, &s.content).await {
                        error!("failed autosave for {}: {:?}", s.doc_id, e);
                    } else {
                        s.last_saved = Instant::now();
                        debug!("autosaved doc {}", s.doc_id);
                    }
                }
            });
        }
    }
}

/// Persist content into DB (simple upsert / update)
async fn save_doc_to_db(pool: &PgPool, doc_id: Uuid, content: &str) -> anyhow::Result<()> {
    info!("Connected to DB: {:?}", pool);
    debug!("autosaving doc {} with content: {:?}", doc_id, content);


    // Update only the content and updated_at
    sqlx::query!(
        r#"
        UPDATE documents
        SET content = $2, updated_at = now()
        WHERE id = $1
        "#,
        doc_id,
        content
    )
    .execute(pool)
    .await?;
    Ok(())
}