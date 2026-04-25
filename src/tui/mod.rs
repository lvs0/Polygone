//! Terminal User Interface for POLYGONE.
//!
//! Built with ratatui + crossterm.
//! Provides interactive views for keygen, send, receive, node, status.

pub mod app;
pub mod views;
pub mod widgets;
pub mod favorites;

pub use app::{App, run_tui};
