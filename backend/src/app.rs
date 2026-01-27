use axum::Router;
use tower_http::trace::TraceLayer;

use crate::db::init_db;
use crate::routes;

pub async fn creat_app() -> anyhow::Result<Router>{

    let db = init_db().await?;

    let app = Router::new().merge(routes::router()).with_state(db).layer(TraceLayer::new_for_http());

    Ok(app)
}
