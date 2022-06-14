pub extern crate num_cpus;

/// # Re-export of Hercules for Public API
#[doc(inline)]
pub use self::hercules::hercules::Hercules;

/// # Re-export of WorkOrder for Public API
#[doc(inline)]
pub use self::hercules::work_order::WorkOrder;

/// Collection of tools and dependencies abstractions.
pub mod tools;

/// Apollo is the deity abstraction for audio and music.
pub mod apollo;

/// ## Stage and location.
pub mod atlas;

/// ## Tasks and jobs.
#[doc(hidden)]
pub mod hercules;

/// ## Internet and multiplayer.
pub mod iris;

/// ## I/O, events and secrets.
/// Kleio is the deity abstraction for file I/O, events and encryption.
pub mod kleio;

/// ## Video rendering.
pub mod vulcan;
