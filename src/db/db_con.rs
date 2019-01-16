use actix::{Actor, SyncContext};
use diesel::pg::PgConnection;

pub struct DbCon(pub PgConnection);

impl Actor for DbCon {
    type Context = SyncContext<Self>;
}
