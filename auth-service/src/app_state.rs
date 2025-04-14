use std::sync::Arc;
use tokio::sync::RwLock;
use crate::domain::UserStore;

pub type UserStoreType = Arc<RwLock<dyn UserStore>>;

#[derive(Clone)]
pub struct AppState {
    pub user_store: UserStoreType,
}