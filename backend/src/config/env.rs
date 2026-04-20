use std::env;

#[derive(Clone)]
pub struct AppConfig {
    pub app_env: String,
    pub app_port: String,
    pub server_addr: String,
    pub database_url: String,
    pub jwt_secret: String,
}

impl AppConfig {
    /// #### .env to backend.
    /// ```rust
    /// dotenv::dotenv().ok() // -> loads .env file ok() insure noting happen if failed.
    /// evn::var("varname").map_err(costumeError) //-> error handling with map_err
    /// ? // insure rearun error early as possible.
    /// ```
    ///  >load .env file and assign the variables to desired places.
    pub fn from_env() -> anyhow::Result<Self> {
        dotenvy::dotenv().ok();

        let app_env = env::var("APP_ENV").map_err(|_| anyhow::anyhow!("APP_ENV not set."))?;
        let app_port = env::var("APP_PORT").map_err(|_| anyhow::anyhow!("APP_PORT not set."))?;
        let server_addr = env::var("SERVER_ADDR").map_err(|_| anyhow::anyhow!("SERVER_ADDR not set."))?;
        let database_url = env::var("DATABASE_URL").map_err(|_| anyhow::anyhow!("DATABASE_URL not set."))?;
        let jwt_secret = env::var("JWT_SECRET").map_err(|_| anyhow::anyhow!("JWT_SECRET not set."))?;

        Ok(Self { app_env, app_port, server_addr, database_url, jwt_secret })
    }
}
