/// # Re-export for Public API
#[doc(inline)]
pub use severity::KJournalEntrySeverity as KJournalEntrySeverity;
pub use severity::get_journal_severity_symbol as get_journal_severity_symbol;
pub use entry::KJournalEntry as KJournalEntry;
pub use listener::KJournalListener as KJournalListener;
pub use listener_print::KJournalListenerPrint as KJournalListenerPrint;
pub use journal::KJournal as KJournal;
pub use journal::KJOURNAL_BUFFER_MAX as KJOURNAL_BUFFER_MAX;
pub use journal::KJOURNAL_BUFFER_MIN as KJOURNAL_BUFFER_MIN;


// Kleio journal
#[doc(hidden)]
pub mod journal;

// Kleio journal severity
#[doc(hidden)]
pub mod severity;

// Kleio journal entry
#[doc(hidden)]
pub mod entry;

// Kleio journal listener
#[doc(hidden)]
pub mod listener;

// Kleio print implementation of journal listener
#[doc(hidden)]
pub mod listener_print;

