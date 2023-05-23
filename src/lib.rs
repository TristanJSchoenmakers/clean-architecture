//! <h1 align="center">clean-architecture (🔨 UNDER CONSTRUCTION)</h1>
//!
//! <p align="center">A cleanly designed Rust REST Api</p>
//!
//! <div align="center">
//!   <!-- Github Actions -->
//!   <a href="https://github.com/TristanJSchoenmakers/clean-architecture/actions/workflows/build-validation.yml">
//!     <img src="https://img.shields.io/github/actions/workflow/status/TristanJSchoenmakers/clean-architecture/build-validation.yml?branch=main&style=flat-square"
//!       alt="actions status" />
//!   </a>
//! </div>
//!
//! <br />
//!
//!
//! ## Technologies 🔧
//!
//! - [Axum](https://github.com/tokio-rs/axum)
//! - [Sqlx](https://github.com/launchbadge/sqlx)
//! - [Xtask](https://github.com/matklad/cargo-xtask)
//!
//!
//! ## Getting started <span style="color:crimson">▶</span>
//!
//! Prerequisites for running locally:
//!
//! - [cargo](https://www.rust-lang.org/tools/install)
//! - [docker (compose)](https://docs.docker.com/engine/install/)
//!
//!
//! #### 1 - Run Docker compose
//!
//! ```bash
//! docker compose up --detach
//! ```
//!
//!
//! #### 2 - Initialize
//!
//! Run `cargo xtask --help` for a list of the other xtask commands
//!
//! ```bash
//! cargo xtask init
//! ```
//!
//!
//! #### 3 - Running the API
//!
//! ```bash
//! cargo run
//! ```

pub mod config;
pub mod domain;
pub mod routes;

#[cfg(test)]
mod test_util {
    use crate::{config::Config, routes};
    use axum::{Extension, Router};
    use clap::Parser;
    use sqlx::postgres::PgPoolOptions;

    /// Sets up the API for testing
    pub async fn setup_api() -> Router {
        let config = Config::parse();

        let db = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .unwrap();

        routes::app().layer(Extension(db))
    }
}
