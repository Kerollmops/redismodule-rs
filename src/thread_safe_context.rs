use crate::key::{RedisKey, RedisKeyWritable};
use crate::{raw, Context};
use crate::LogLevel;
use crate::{RedisResult, RedisString};

pub struct ThreadSafeContext {
    ctx: Context,
}

unsafe impl Send for ThreadSafeContext {}

impl ThreadSafeContext {
    pub fn create() -> Self {
        let ctx = raw::get_thread_safe_context(std::ptr::null_mut());
        ThreadSafeContext { ctx: Context::new(ctx) }
    }

    pub fn dummy() -> Self {
        Self { ctx: Context::dummy() }
    }

    pub fn log(&self, level: LogLevel, message: &str) {
        self.ctx.log(level, message)
    }

    pub fn log_debug(&self, message: &str) {
        self.ctx.log_debug(message)
    }

    pub fn auto_memory(&self) {
        self.ctx.auto_memory()
    }

    pub fn call(&self, command: &str, args: &[&str]) -> RedisResult {
        raw::thread_safe_context_lock(self.ctx.ctx);
        let result = self.ctx.call(command, args);
        raw::thread_safe_context_unlock(self.ctx.ctx);
        result
    }

    pub fn reply(&self, r: RedisResult) -> raw::Status {
        self.ctx.reply(r)
    }

    #[cfg(feature = "experimental-api")]
    pub fn subscribe_to_keyspace_events(
        &self,
        types: i32,
        cb: Option<raw::RedisModuleNotificationFunc>,
    ) -> i32
    {
        self.ctx.subscribe_to_keyspace_events(types, cb)
    }

    pub fn create_string(&self, s: &str) -> RedisString {
        self.ctx.create_string(s)
    }

    pub fn open_key(&self, key: &str) -> RedisKey {
        self.ctx.open_key(key)
    }

    pub fn open_key_writable(&self, key: &str) -> RedisKeyWritable {
        self.ctx.open_key_writable(key)
    }

    pub fn replicate_verbatim(&self) {
        self.ctx.replicate_verbatim()
    }
}

impl Drop for ThreadSafeContext {
    fn drop(&mut self) {
        raw::free_thread_safe_context(self.ctx.ctx);
    }
}
