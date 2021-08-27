//! # Steamgriddb API
//!
//! This project is a wrapper for the steamgriddb api, written in Rust.
//! 
//! ## Getting started
//! 
//! The easiest way to get started is using the Client.
//! 
//! ```no_run
//! use steamgriddb_api::client::Client;
//! use steamgriddb_api::query_parameters::QueryType::Grid;
//! async fn example() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = Client::new("my_auth_key");
//!     let games = client.search("Celeste").await?;
//!     let first_game = games.iter().next();
//!     if let Some(first_game) = first_game {
//!         assert_eq!("Celeste", first_game.name);
//!         let images = client.get_images_for_id(first_game.id, &Grid(None)).await?;
//!     }
//!     Ok(())
//!  }
//! ```


pub mod client;
pub mod games;
pub mod images;
pub mod query_parameters;
pub mod response;
pub mod search;
pub mod steam_static;

