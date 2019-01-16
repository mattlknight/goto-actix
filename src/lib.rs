#![allow(proc_macro_derive_resolution_fallback)]
#![feature(proc_macro_hygiene, decl_macro)]
#![warn(rust_2018_idioms)]

#[macro_use] extern crate diesel;

mod schema;
mod models;
mod routes;
mod types;
mod db;
pub mod server;
