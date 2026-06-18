//! # Polygone Messaging — High-level ephemeral message API
//!
//! Provides a complete send/receive pipeline for Polygone's post-quantum
//! privacy network. The protocol flow:
//!
//! ```text
//! Alice                          Network                        Bob
//!   │                                │                             │
//!   │  1. ML-KEM encapsulate         │                             │
//!   │──────────────────────────────► │ (public key fetch)           │
//!   │                                │                             │
//!   │  2. AES-256-GCM encrypt        │                             │
//!   │──────────────────────────────► │                             │
//!   │  3. Shamir 4-of-7 split        │                             │
//!   │──────────────────────────────► │                             │
//!   │  4. Route to 7 nodes           │                             │
//!   │                                │                             │
//!   │                                │◄────────────────────────────│
//!   │                                │  (collect 4+ fragments)     │
//!   │                                │                             │
//!   │  5. Receive + reassemble      │                             │
//!   │  6. AES-256-GCM decrypt        │                             │
//!   │  7. ML-KEM decapsulate        │                             │
//!   │                                │                             │
//!   ▼                                ▼                             ▼
//! ```
//!
//! # Example
//!
//! ```ignore
//! use polygone_msg::{MessageSession, OutgoingMessage};
//!
//! let session = MessageSession::new(my_secret_key, peers_public_key)?;
//! let msg = OutgoingMessage::new(b"Hello, world!");
//! let encrypted = session.encapsulate(msg)?;
//! // encrypted.envelopes ready for fragment dispatch
//! ```

pub mod message;
pub mod session;

pub use message::{Message, OutgoingMessage, Envelope, MessageId, MessageMeta};
pub use session::{MessageSession, SessionError};

/// Current protocol version for wire compatibility checks.
pub const PROTOCOL_VERSION: &str = "1.0.0";