use axum::Router; 

mod auth;
mod users;
mod apps;
mod purchases;

pub fn router()-> Router{
    Router::new()
        .nest("/auth", auth::routes())
        .nest("/users", users::routes())
        .nest("/apps", apps::routes())
        .nest("/purchases", purchases::routes())

}
