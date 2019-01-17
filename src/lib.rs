#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]
#![warn(rust_2018_idioms)]

#[macro_use] extern crate diesel;

pub mod db;
pub mod routes;
pub mod schema;
pub mod types;
pub mod server;

pub use db::establish_connection;
