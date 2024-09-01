use std::sync::Arc;
use tokio::sync::Mutex;

#[derive(Default)]
pub struct Rush {}

pub type RushState = Arc<Mutex<Rush>>;

impl Rush {}
