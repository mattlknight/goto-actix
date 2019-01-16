// use actix::{Addr, SyncArbiter};
use actix::Addr;
use crate::db;

pub struct AppState {
    pub db: Addr<db::DbCon>,
}
