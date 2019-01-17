use actix::Addr;
use crate::db::DbCon;

pub struct AppState {
    pub db: Addr<DbCon>,
}
