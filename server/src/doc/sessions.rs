use tokio::{sync::{broadcast, Mutex}, time::Instant};
use uuid::Uuid;
use std::sync::Arc;
use dashmap::DashMap;
use crate::doc::models::{Op, ServerMessage};

pub struct DocSession {
    pub doc_id: Uuid,
    pub content: String,
    pub version: u64,
    pub history: Vec<Op>,
    pub broadcaster: broadcast::Sender<ServerMessage>,
    pub last_saved: Instant,
}

impl DocSession {
    pub fn new(doc_id: Uuid, initial_content: String) -> Self {
        let (tx, _rx) = broadcast::channel(1024);
        Self {
            doc_id,
            content: initial_content,
            version: 0,
            history: Vec::new(),
            broadcaster: tx,
            last_saved: Instant::now(),
        }
    }
}

pub type Sessions = Arc<DashMap<Uuid, Arc<Mutex<DocSession>>>>;
