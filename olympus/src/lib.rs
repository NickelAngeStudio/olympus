//! Olympus is a simple, abstracted game engine.
// TODO: Fill Olympus description for crate
// TODO: Create logo for olympus with NAS style and color

/// Collection of tools and dependencies abstractions.
#[doc(hidden)]
pub mod tools;

/// # God of audio and music.
pub mod apollo;

/// # God of cartography, stage and maps.
pub mod atlas;

/// # God of secrets and confidentiality.
/// 
/// [`Harpocrates`](https://en.wikipedia.org/wiki/Harpocrates) (Ancient Greek: Ἁρποκράτης) was the god of silence, secrets and confidentiality in the Hellenistic religion developed in 
/// Ptolemaic Alexandria (and also an embodiment of hope, according to Plutarch). Harpocrates was adapted by the Greeks from the Egyptian 
/// child god Horus, who represented the newborn sun, rising each day at dawn. Horus is represented as a naked boy with his finger to his mouth, a 
/// realisation of the hieroglyph for "child". Misunderstanding this gesture, the later Greeks and Roman poets made Harpocrates the god of 
/// silence and secrecy.
/// 
/// Harpocrates provide 2 traits used to cypher and decypher buffers and 1 trait to conceal information in memory.
/// 
/// # Diagram
/// TODO: Add SVGZ link
pub mod harpocrates;


/// # God of multi-threaded labours.
/// 
/// [`Hercules`](https://en.wikipedia.org/wiki/Hercules) (/ˈhɜːrkjʊˌliːz/, US: /-kjə-/) is the Roman equivalent of the Greek divine hero Heracles, 
/// son of Jupiter and the mortal Alcmene. In classical mythology, Hercules is famous for his strength and for his numerous far-ranging adventures 
/// including his 12 labours.
/// 
/// Hercules supplies `Taskmaster` as a taskpool and `WorkOrder` for tasks sync.
/// 
/// # Diagram
/// TODO: Add SVGZ link


pub mod hercules;

/// # Goddess of messages and communication.
pub mod iris;

/// # Goddess of history and events. 
pub mod kleio;

/// # God of fire and video frame crafting.
pub mod vulcan;
