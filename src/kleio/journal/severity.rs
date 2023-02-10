/// ##### Enumeration of journal entry severities as bytes flags in order of severity.
#[allow(non_snake_case)]
pub mod KJournalEntrySeverity {

    /// Lowest severity. Debug message. Listeners shouldn't listener to those by default.
    pub const DEBUG: u8 = 1;

    /// Other message with low severity impact.
    pub const OTHER: u8 = 2;

    /// Information about event as they occur.
    pub const INFORMATION: u8 = 4;

    /// An occurred event that can be potentially severe.
    pub const WARNING: u8 = 8;

    /// A severe error that occurred but didn't cause the program to crash.
    pub const ERROR: u8 = 16;

    /// Highest severity. Causes program to crash.
    pub const FATAL: u8 = 32;
    
    /// Quick shortcut to all severity flags without DEBUG.
    pub const ALL_NO_DEBUG: u8 = 62;

    /// Quick shortcut to all severity flags including DEBUG.
    pub const ALL_WITH_DEBUG: u8 = 63;
}

/// Returns a 1 character symbol of severity.
/// 
/// * DEBUG => 'D'
/// * OTHER => 'O'
/// * INFORMATION => 'I'
/// * WARNING => 'W'
/// * ERROR => 'E'
/// * FATAL => 'F'
/// * UNKNOWN => '?'
pub fn get_journal_severity_symbol(severity : u8) -> char {

    match severity {
        KJournalEntrySeverity::DEBUG => 'D',
        KJournalEntrySeverity::OTHER => 'O',
        KJournalEntrySeverity::INFORMATION => 'I',
        KJournalEntrySeverity::WARNING => 'W',
        KJournalEntrySeverity::ERROR => 'E',
        KJournalEntrySeverity::FATAL => 'F',
        
        _ => '?'
    }
}