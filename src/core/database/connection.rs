use anyhow::Context;
use diesel::{
    prelude::*,
    r2d2::{ConnectionManager, Pool, PooledConnection},
};
use dotenvy::dotenv;
use std::time::Duration;
pub type DbPool = Pool<ConnectionManager<PgConnection>>;
pub fn get_connection_pool(database_url: String) -> DbPool {
    dotenv().ok();

    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .test_on_check_out(true)
        .max_size(3)
        .connection_timeout(Duration::from_secs(60))
        .build(manager)
        .inspect_err(|error| println!("Error: {}", error))
        .expect("Could not build connection pool")
}

pub async fn interact<F, T>(pool: &DbPool, f: F) -> anyhow::Result<T>
where
    F: FnOnce(&mut PgConnection) -> anyhow::Result<T> + 'static + Send,
    T: Send + 'static,
{
    let pool = pool.clone();
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get()?;
        f(&mut conn)
    })
    .await?
}

pub async fn interact_with_connection<F, T>(pool: &DbPool, f: F) -> anyhow::Result<T>
where
    F: FnOnce(
            &mut PooledConnection<ConnectionManager<PgConnection>>,
        ) -> diesel::result::QueryResult<T>
        + 'static
        + Send,
    T: Send + 'static,
{
    let pool = pool.clone();
    tokio::task::spawn_blocking(move || {
        let mut conn = pool.get().context("Getting conn from pool")?;
        conn.transaction(|conn| f(conn)).map_err(Into::into)
    })
    .await
    .context("Returning from thread")?
}
