pub mod dto;
#[cfg(feature = "ssr")]
pub mod model;
#[cfg(feature = "ssr")]
pub mod repository;
pub mod server;
#[cfg(feature = "ssr")]
pub mod service;

#[cfg(feature = "ssr")]
pub mod render;

pub mod component;
