#[derive(Debug, Default)]
pub struct CacheStats {
    pub hits: usize,
    pub misses: usize,
}
