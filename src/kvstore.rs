use chashmap::CHashMap;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct KvElement {
    pub data: Vec<u8>,
    pub mime_type: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub expire_at: DateTime<Utc>,
    pub update_count: i32,
    pub locked: bool
}

pub struct KvStore {
    container: CHashMap<String, KvElement>
}

impl KvStore
{
    pub fn new() -> KvStore
    {
        // TODO: prepare looped persistence
        KvStore {
            container: CHashMap::new()
        }
    }

    pub fn set(&self, key: String, value: Vec<u8>) -> Option<KvElement> {
        // TODO: prepare iterative persistence
        let mime_type = tree_magic::from_u8(value.as_ref());
        match &mut self.container.get_mut(&key) {
            Some(kv_element) => {
                kv_element.data = value;
                kv_element.mime_type = mime_type;
                kv_element.updated_at = Utc::now();
                kv_element.update_count = kv_element.update_count + 1; 
                Some(kv_element.to_owned())
            },
            None => {
                let kv_element = KvElement {
                    data: value,
                    mime_type: mime_type,
                    created_at: Utc::now(),
                    updated_at: Utc::now(),
                    expire_at: Utc::now(),
                    update_count: 1,
                    locked: false,
                };
                self.container.insert(key, kv_element)
            }
        }
    }

    pub fn get(&self, key: String) -> Option<KvElement> {
        match self.container.get(&key) {
            Some(value) => Some(value.clone()),
            None => None
        }
    }

    pub fn switch_lock(&self, key: String, to_lock: bool) -> bool {
        match &mut self.container.get_mut(&key) {
            Some(kv_element) => {
                kv_element.locked = to_lock;
                true
            },
            None => false
        }
    }

    // TODO: implement Lock, Unlock, Increment, Decrement, Expire

    pub fn drop(&self, key: String) {
        self.container.remove(&key);
    }
}