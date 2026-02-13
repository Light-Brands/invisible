//! # Invisible Scrambler
//!
//! The Scrambler implements Invisible's 7-layer network obfuscation system:
//!
//! 1. **Ghost VPN (Layer 0):** Mandatory WireGuard tunnel
//! 2. **Shamir Fragmentation (Layer 1):** K-of-N shares across paths
//! 3. **5-Layer Mixnet (Layer 2):** Sphinx packets, batch-shuffle-forward
//! 4. **Cover Traffic (Layer 3):** Constant-rate stream
//! 5. **Jurisdiction Routing (Layer 4):** Multi-country paths
//! 6. **Protocol Camouflage (Layer 5):** obfs5/uTLS/domain fronting
//! 7. **Dead Drops (Layer 6):** Anonymous relay mailboxes
//! 8. **Temporal Scrambling (Layer 7):** Poisson-distributed delays
//!
//! ## Security Properties
//!
//! - **Traffic Analysis Resistance:** Constant-rate cover traffic hides patterns
//! - **Metadata Protection:** No correlation between sender and recipient
//! - **Censorship Resistance:** Protocol camouflage defeats DPI
//! - **Geographic Diversity:** Multi-jurisdiction routing prevents single-point control

#![forbid(unsafe_code)]
#![warn(
    missing_docs,
    rust_2018_idioms,
    unused_qualifications,
    missing_debug_implementations
)]

pub mod error;
pub mod sphinx;
pub mod mixnet;
pub mod cover_traffic;
pub mod shamir;
pub mod temporal;

pub use error::{ScramblerError, Result};
