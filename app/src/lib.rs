pub mod my_error;

/// use redis-rs(async) without pool
pub mod direct;

/// with r2d2 pool of redis-rs feature
pub mod with_r2d2_feature;

/// with r2d2 pool of r2d2-redis
pub mod with_old_r2d2;

/// with bb8 pool
pub mod with_bb8;

/// with deadpool pool
pub mod with_deadpool;

/// with mobc pool
pub mod with_mobc;
