use std::collections::HashMap;
use std::sync::Arc;

/// Cache LRU simples de bytes de arquivos (para prefetch bidirecional).
/// Chave = path absoluto em string; valor = bytes crus (comprimidos, como no disco).
pub struct Cache {
    map: HashMap<String, Entry>,
    tick: u64,
    bytes: usize,
    cap: usize,
}

struct Entry {
    data: Arc<Vec<u8>>,
    last: u64,
}

impl Cache {
    pub fn new(cap: usize) -> Self {
        Self {
            map: HashMap::new(),
            tick: 0,
            bytes: 0,
            cap,
        }
    }

    pub fn set_cap(&mut self, cap: usize) {
        self.cap = cap;
        self.evict();
    }

    pub fn cap(&self) -> usize {
        self.cap
    }

    pub fn total_bytes(&self) -> usize {
        self.bytes
    }

    pub fn len(&self) -> usize {
        self.map.len()
    }

    pub fn get(&mut self, key: &str) -> Option<Arc<Vec<u8>>> {
        self.tick = self.tick.wrapping_add(1);
        let tick = self.tick;
        match self.map.get_mut(key) {
            Some(entry) => {
                entry.last = tick;
                Some(entry.data.clone())
            }
            None => None,
        }
    }

    pub fn contains(&self, key: &str) -> bool {
        self.map.contains_key(key)
    }

    pub fn put(&mut self, key: String, data: Arc<Vec<u8>>) {
        if self.cap == 0 {
            return;
        }
        let size = data.len();
        // Um único arquivo maior que o teto não entra no cache.
        if size > self.cap {
            return;
        }
        self.tick = self.tick.wrapping_add(1);
        let tick = self.tick;
        if let Some(old) = self.map.insert(key, Entry { data, last: tick }) {
            self.bytes = self.bytes.saturating_sub(old.data.len());
        }
        self.bytes += size;
        self.evict();
    }

    pub fn clear(&mut self) {
        self.map.clear();
        self.bytes = 0;
    }

    fn evict(&mut self) {
        while self.bytes > self.cap && !self.map.is_empty() {
            let mut victim: Option<String> = None;
            let mut oldest = u64::MAX;
            for (k, v) in self.map.iter() {
                if v.last < oldest {
                    oldest = v.last;
                    victim = Some(k.clone());
                }
            }
            match victim {
                Some(k) => {
                    if let Some(e) = self.map.remove(&k) {
                        self.bytes = self.bytes.saturating_sub(e.data.len());
                    }
                }
                None => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn put(cache: &mut Cache, key: &str, len: usize) {
        cache.put(key.to_string(), Arc::new(vec![0u8; len]));
    }

    #[test]
    fn evicts_lru_when_over_cap() {
        let mut cache = Cache::new(10);
        put(&mut cache, "a", 6);
        put(&mut cache, "b", 6);
        assert!(
            !cache.contains("a"),
            "o item mais antigo deveria ter sido evictado"
        );
        assert!(cache.contains("b"));
        assert!(cache.total_bytes() <= 10);
    }

    #[test]
    fn get_refreshes_lru_order() {
        let mut cache = Cache::new(10);
        put(&mut cache, "a", 6);
        put(&mut cache, "b", 4);
        assert!(cache.get("a").is_some());
        put(&mut cache, "c", 4);
        assert!(
            cache.contains("a"),
            "a foi tocado recentemente e deve ficar"
        );
        assert!(cache.contains("c"));
        assert!(!cache.contains("b"), "b era o menos usado recentemente");
        assert!(cache.total_bytes() <= 10);
    }

    #[test]
    fn skips_file_larger_than_cap() {
        let mut cache = Cache::new(4);
        put(&mut cache, "x", 5);
        assert!(!cache.contains("x"));
        assert_eq!(cache.total_bytes(), 0);
    }

    #[test]
    fn clear_empties_everything() {
        let mut cache = Cache::new(64);
        put(&mut cache, "a", 10);
        cache.clear();
        assert_eq!(cache.len(), 0);
        assert_eq!(cache.total_bytes(), 0);
    }
}
