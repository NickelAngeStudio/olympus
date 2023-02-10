use std::{time::{SystemTime, Duration}, thread::sleep};
use olympus::kleio::journal::{KJournalEntry, KJournalEntrySeverity};

#[test]
/// Create a new KJournal entry
/// 
/// # Verification(s)
/// V1 | KJournalEntry::new() create an entry without error.
/// V2 | Verify severity and description values to be the same as control.
/// V3 | Verify that entry date is valid.
fn kjournal_entry_new() {
    // Initial variables for comparison.
    let severity = KJournalEntrySeverity::INFORMATION;
    let description = "Entry kjournal_entry_new";

    // V1 | KJournalEntry::new() create an entry without error.
    let j = KJournalEntry::new(severity, description.clone());

    // V2 | Verify severity and description values to be the same as control.
    assert!(j.get_severity() == severity, "Error! Created entry severity is different!");
    assert!(j.get_description().eq(&description), "Error! Created entry description is different!");

    // V3 | Verify that entry date is valid.
    match j.get_date_time().duration_since(SystemTime::UNIX_EPOCH) {
        Ok(n) => 
        assert!(n.as_secs() > 0, "Error! Date and time not recorded!"),
        Err(_) => panic!("Error! SystemTime before UNIX EPOCH!"),
    }

}

#[test]
/// Update a KJournalEntry values.
/// 
/// # Verification(s)
/// V1 | KJournalEntry::update() update values without error.
/// V2 | Verify severity and description values to be the same as control.
/// V3 | Verify that entry date is valid.
fn kjournal_entry_update() {
    // Initial variables for comparison.
    let severity = KJournalEntrySeverity::INFORMATION;
    let description = "Entry kjournal_entry_new";

    // Create new entry.
    let mut j = KJournalEntry::new(severity, description.clone());

    // Copy previous date and time
    let dt = j.get_date_time().clone();

    // Wait 1 seconds (to get a different system time).
    sleep(Duration::new(1, 0));

    // Update value and entry
    let severity = KJournalEntrySeverity::ERROR;
    let description = "Entry updated";

    // V1 | KJournalEntry::update() update values without error.
    j.update(severity, description.clone());

    // V2 | Verify severity and description values to be the same as control.
    assert!(j.get_severity() == severity, "Error! Updated entry severity is different!");
    assert!(j.get_description().eq(&description), "Error! Updated entry description is different!");

    // V3 | Verify that entry date is valid.
    match j.get_date_time().duration_since(dt) {
        Ok(n) => 
        assert!(n.as_millis() > 0, "Error! Date and time difference should be higher than 0!"),
        Err(_) => panic!("Error! SystemTime before UNIX EPOCH!"),
    }

}
