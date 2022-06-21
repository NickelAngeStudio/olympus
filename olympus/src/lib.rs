//! Olympus is a simple, abstracted game engine.
// TODO: Fill Olympus description for crate

/// Collection of tools and dependencies abstractions.
#[doc(hidden)]
pub mod tools;

/// # God of audio and music.
pub mod apollo;

/// # God of cartography, stage and maps.
pub mod atlas;

/// # God of secrets and confidentiality.
/// 
/// 
pub mod harpocrates;

/// # God of multi-threaded labours.
/// 
/// [`Hercules`](https://en.wikipedia.org/wiki/Hercules) (/ˈhɜːrkjʊˌliːz/, US: /-kjə-/) is the Roman equivalent of the Greek divine hero Heracles, 
/// son of Jupiter and the mortal Alcmene. In classical mythology, Hercules is famous for his strength and for his numerous far-ranging adventures 
/// including his 12 labours.
/// 
/// Hercules supplies `Taskmaster` as a taskpool and `WorkOrder` for tasks sync.
pub mod hercules;

/// # Goddess of messages and communication.
pub mod iris;

/// # Goddess of history and events. 
pub mod kleio;

/// # God of fire and video frame crafting.
pub mod vulcan;
