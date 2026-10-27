use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub app_name: String,
    pub db: PgPool,
}

impl AppState {
    pub fn new(db: PgPool) -> Self {
        Self {
            app_name: "toto-ticket".to_string(),
            db,
        }
    }
}
