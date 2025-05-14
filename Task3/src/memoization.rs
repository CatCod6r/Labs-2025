use std::{
    collections::{HashMap, VecDeque},
    hash::Hash,
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};

/// Enum for supported eviction strategies.
#[derive(Clone)]
pub enum EvictionPolicy {
    LRU,
    LFU,
    TimeBased(Duration),
    Custom(Arc<dyn Fn(&mut HashMap<u64, CacheEntry>) + Send + Sync>),
}

struct CacheEntry {
    value: Box<dyn std::any::Any + Send + Sync>,
    hits: usize,
    last_used: Instant,
    created_at: Instant,
}

pub struct Memoizer<F, Args, Output>
where
    F: Fn(Args) -> Output + Send + Sync + 'static,
    Args: Hash + Eq + Clone + Send + 'static,
    Output: Clone + Send + Sync + 'static,
{
    function: F,
    cache: Arc<Mutex<HashMap<u64, CacheEntry>>>,
    max_size: Option<usize>,
    eviction_policy: EvictionPolicy,
    _marker: std::marker::PhantomData<(Args, Output)>,
}

impl<F, Args, Output> Memoizer<F, Args, Output>
where
    F: Fn(Args) -> Output + Send + Sync + 'static,
    Args: Hash + Eq + Clone + Send + 'static,
    Output: Clone + Send + Sync + 'static,
{
    pub fn new(function: F, max_size: Option<usize>, policy: EvictionPolicy) -> Self {
        Self {
            function,
            cache: Arc::new(Mutex::new(HashMap::new())),
            max_size,
            eviction_policy: policy,
            _marker: std::marker::PhantomData,
        }
    }

    pub fn call(&self, args: Args) -> Output {
        let key = Self::hash_key(&args);
        let mut cache = self.cache.lock().unwrap();

        if let Some(entry) = cache.get_mut(&key) {
            entry.hits += 1;
            entry.last_used = Instant::now();
            return entry.value.downcast_ref::<Output>().unwrap().clone();
        }

        let result = (self.function)(args.clone());

        if let Some(max) = self.max_size {
            if cache.len() >= max {
                self.evict(&mut cache);
            }
        }

        cache.insert(
            key,
            CacheEntry {
                value: Box::new(result.clone()),
                hits: 1,
                last_used: Instant::now(),
                created_at: Instant::now(),
            },
        );

        result
    }

    fn evict(&self, cache: &mut HashMap<u64, CacheEntry>) {
        match &self.eviction_policy {
            EvictionPolicy::LRU => {
                if let Some((&oldest_key, _)) = cache.iter().min_by_key(|(_, v)| v.last_used) {
                    cache.remove(&oldest_key);
                }
            }
            EvictionPolicy::LFU => {
                if let Some((&least_key, _)) = cache.iter().min_by_key(|(_, v)| v.hits) {
                    cache.remove(&least_key);
                }
            }
            EvictionPolicy::TimeBased(duration) => {
                let now = Instant::now();
                let expired_keys: Vec<u64> = cache
                    .iter()
                    .filter(|(_, v)| now.duration_since(v.created_at) > *duration)
                    .map(|(&k, _)| k)
                    .collect();
                for k in expired_keys {
                    cache.remove(&k);
                }
            }
            EvictionPolicy::Custom(f) => {
                f(cache);
            }
        }
    }

    fn hash_key<T: Hash>(t: &T) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        let mut s = DefaultHasher::new();
        t.hash(&mut s);
        s.finish()
    }
}

