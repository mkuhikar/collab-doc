use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{broadcast, RwLock};
use uuid::Uuid;

pub type Tx = broadcast::Sender<String>;

#[derive(Clone, Default)]
pub struct CollabState {
    pub docs: Arc<RwLock<HashMap<Uuid, Tx>>>,
}

impl CollabState {
    pub async fn get_or_create_channel(&self, doc_id: Uuid) -> Tx {
        let mut docs = self.docs.write().await;
        docs.entry(doc_id)
            .or_insert_with(|| broadcast::channel(100).0) // keep last 100 messages
            .clone()
    }
}
