use uuid::Uuid;

// TTL policies (in seconds)
pub const TTL_USER: usize = 60 * 60 * 24; // 24 hours
pub const TTL_REPO: usize = 60 * 60; // 1 hour
pub const TTL_ACTIVITY: usize = 60 * 5; // 5 minutes

// Cache key helpers
pub fn user_cache_key(user_id: &Uuid) -> String {
    format!("user:{}", user_id)
}

pub fn repo_cache_key(repo_id: &Uuid) -> String {
    format!("repo:{}", repo_id)
}

pub fn org_cache_key(org_id: &Uuid) -> String {
    format!("org:{}", org_id)
}

pub fn activity_cache_key(activity_id: &Uuid) -> String {
    format!("activity:{}", activity_id)
}

pub trait CacheableEntity {
    fn cache_key(&self) -> String;
    fn cache_key_from_id(id: &Uuid) -> String;
    fn cache_ttl() -> usize;
}
