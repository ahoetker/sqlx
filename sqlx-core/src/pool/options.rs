use std::{marker::PhantomData, time::Duration};

use crate::Database;

use super::Pool;

/// Builder for [Pool].
pub struct Builder<DB>
where
    DB: Database,
{
    phantom: PhantomData<DB>,
    options: Options,
}

impl<DB> Builder<DB>
where
    DB: Database,
{
    /// Get a new builder with default options.
    ///
    /// See the source of this method for current defaults.
    pub fn new() -> Self {
        Self {
            phantom: PhantomData,
            options: Options {
                // pool a maximum of 10 connections to the same database
                max_size: 10,
                // don't open connections until necessary
                min_size: 0,
                // try to connect for 30 seconds before erroring
                connect_timeout: Duration::from_secs(30),
                // reap connections that have been alive > 30 minutes
                // prevents unbounded live-leaking of memory due to naive prepared statement caching
                // see src/cache.rs for context
                max_lifetime: Some(Duration::from_secs(1800)),
                // don't reap connections based on idle time
                idle_timeout: None,
            },
        }
    }

    /// Set the maximum number of connections that this pool should maintain.
    pub fn max_size(mut self, max_size: u32) -> Self {
        self.options.max_size = max_size;
        self
    }

    /// Set the amount of time to attempt connecting to the database.
    ///
    /// If this timeout elapses, [Pool::acquire] will return an error.
    pub fn connect_timeout(mut self, connect_timeout: Duration) -> Self {
        self.options.connect_timeout = connect_timeout;
        self
    }

    /// Set the minimum number of connections to maintain at all times.
    ///
    /// When the pool is built, this many connections will be automatically spun up.
    ///
    /// If any connection is reaped by [max_lifetime] or [idle_timeout] and it brings
    /// the connection count below this amount, a new connection will be opened to replace it.
    pub fn min_size(mut self, min_size: u32) -> Self {
        self.options.min_size = min_size;
        self
    }

    /// Set the maximum lifetime of individual connections.
    ///
    /// Any connection with a lifetime greater than this will be closed.
    ///
    /// When set to `None`, all connections live until either reaped by [idle_timeout]
    /// or explicitly disconnected.
    ///
    /// Infinite connections are not recommended due to the unfortunate reality of memory/resource
    /// leaks on the database-side. It is better to retire connections periodically
    /// (even if only once daily) to allow the database the opportunity to clean up data structures
    /// (parse trees, query metadata caches, thread-local storage, etc.) that are associated with a
    /// session.
    pub fn max_lifetime(mut self, max_lifetime: impl Into<Option<Duration>>) -> Self {
        self.options.max_lifetime = max_lifetime.into();
        self
    }

    /// Set a maximum idle duration for individual connections.
    ///
    /// Any connection with an idle duration longer than this will be closed.
    ///
    /// For usage-based database server billing, this can be a cost saver.
    pub fn idle_timeout(mut self, idle_timeout: impl Into<Option<Duration>>) -> Self {
        self.options.idle_timeout = idle_timeout.into();
        self
    }

    /// Spin up the connection pool.
    ///
    /// If [min_size] was set to a non-zero value, that many connections will be immediately
    /// opened and placed into the pool.
    pub async fn build(self, url: &str) -> crate::Result<Pool<DB>> {
        Pool::with_options(url, self.options).await
    }
}

impl<DB> Default for Builder<DB>
where
    DB: Database,
{
    fn default() -> Self {
        Self::new()
    }
}

pub(crate) struct Options {
    pub max_size: u32,
    pub connect_timeout: Duration,
    pub min_size: u32,
    pub max_lifetime: Option<Duration>,
    pub idle_timeout: Option<Duration>,
}
