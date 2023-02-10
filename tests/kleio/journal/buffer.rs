use olympus::kleio::journal::{journal::{KJournalBuffer, KJOURNAL_BUFFER_MIN, KJOURNAL_BUFFER_MAX}, KJournalEntrySeverity, KJournalEntry};

/// Used test buffer size. Is padded in function that use it to make sure it is between MIN and MAX.
static KJOURNAL_BUFFER_TEST_SIZE:usize = 250;

#[test]
/// Create a new instance of KJournalBuffer with size in the middle of KJOURNAL_BUFFER_MIN and KJOURNAL_BUFFER_MAX
/// 
/// # Verification(s)
/// V1 | KJournalBuffer::new(KJOURNAL_BUFFER_TEST_SIZE) created without error.
fn kjournal_buffer_new() {
    // Used test buffer size is KJOURNAL_BUFFER_TEST_SIZE. If min is bigger, it become min. If max is lower, it become max.
    let buffer_size:usize = std::cmp::min(std::cmp::max(KJOURNAL_BUFFER_TEST_SIZE, KJOURNAL_BUFFER_MIN), KJOURNAL_BUFFER_MAX);

    // V1 | KJournalBuffer::new(KJOURNAL_BUFFER_TEST_SIZE) created without error.
    let _ = KJournalBuffer::new(buffer_size);
}

#[test]
/// Create a new instance of KJournalBuffer with minimal allowed size.
/// 
/// # Verification(s)
/// V1 | KJournalBuffer::new(KJOURNAL_BUFFER_MIN) created without error.
fn kjournal_buffer_new_min() {
    // V1 | KJournalBuffer::new(KJOURNAL_BUFFER_MIN) created without error.
    let _ = KJournalBuffer::new(KJOURNAL_BUFFER_MIN);
}

#[test]
/// Create a new instance of KJournalBuffer with maximum allowed size.
/// 
/// # Verification(s)
/// V1 | KJournalBuffer::new(KJOURNAL_BUFFER_MAX) created without error.
fn kjournal_buffer_new_max() {
    // V1 | KJournalBuffer::new(KJOURNAL_BUFFER_MAX) created without error.
    let _ = KJournalBuffer::new(KJOURNAL_BUFFER_MAX);
}

#[test]
/// Test limit of size when creating KJournalBuffer.
///
/// # Verification(s)
/// V1 | New KJournalBuffer created with buffer size < KJOURNAL_BUFFER_MIN must return Err(KJournalError::BufferSizeTooSmall).
/// V2 | New KJournalBuffer created with buffer size > KJOURNAL_BUFFER_MAX must return Err(KJournalError::BufferSizeTooBig).
fn kjournal_buffer_new_limit() {
    // V1 | New KJournalBuffer created with buffer size < KJOURNAL_BUFFER_MIN must return Err(KJournalError::BufferSizeTooSmall).
    match KJournalBuffer::new(KJOURNAL_BUFFER_MIN - 1){
        Ok(_) => panic!("size < MIN must return Err(KJournalError::BufferSizeTooSmall)!"),
        Err(err) => match err {
            olympus::kleio::journal::journal::KJournalError::BufferSizeTooSmall => {},
            olympus::kleio::journal::journal::KJournalError::BufferSizeTooBig => panic!("size < MIN must return Err(KJournalError::BufferSizeTooSmall)!"),
        },
    }


    // V2 | New KJournalBuffer created with buffer size > KJOURNAL_BUFFER_MAX must return Err(KJournalError::BufferSizeTooBig).
    match KJournalBuffer::new(KJOURNAL_BUFFER_MAX + 1){
        Ok(_) => panic!("size > MAX must return Err(KJournalError::BufferSizeTooBig)!"),
        Err(err) => match err {
            olympus::kleio::journal::journal::KJournalError::BufferSizeTooSmall => panic!("size > MAX must return Err(KJournalError::BufferSizeTooBig)!"),
            olympus::kleio::journal::journal::KJournalError::BufferSizeTooBig => {},
        },
    }
}

#[test]
/// Verify the size of KJournalBuffer created.
/// 
/// # Verification(s)
/// V1 | KJournalBuffer::size() should be equal to created size.
fn kjournal_buffer_size() {
    // Used test buffer size is KJOURNAL_BUFFER_TEST_SIZE. If min is bigger, it become min. If max is lower, it become max.
    let buffer_size:usize = std::cmp::min(std::cmp::max(KJOURNAL_BUFFER_TEST_SIZE, KJOURNAL_BUFFER_MIN), KJOURNAL_BUFFER_MAX);

    // V1 | KJournalBuffer::size() should be equal to created size.
    let kb = KJournalBuffer::new(buffer_size).unwrap();
    assert!(kb.size() == buffer_size, "KJournalBuffer::size() is incorrect!");
}

#[test]
/// Verify that KJournalBuffer::latest() return None when no entries.
/// 
/// # Verification(s)
/// V1 | KJournalBuffer::latest() return None when empty.
fn kjournal_buffer_latest_none() {
     // Used test buffer size is KJOURNAL_BUFFER_TEST_SIZE. If min is bigger, it become min. If max is lower, it become max.
     let buffer_size:usize = std::cmp::min(std::cmp::max(KJOURNAL_BUFFER_TEST_SIZE, KJOURNAL_BUFFER_MIN), KJOURNAL_BUFFER_MAX);
 
     let mut kb = KJournalBuffer::new(buffer_size).unwrap();

     // V1 | KJournalBuffer::latest() return None when empty.
     match kb.latest() {
        Some(_) => assert!(false, "KJournalBuffer::latest() should give None when empty!"),
        None => {}, // OK do nothing
    }
} 

#[test]
/// Verify writing entries into buffer.
/// 
/// # Verification(s)
/// V1 | KJournalBuffer::write() write an entry into buffer without error.
/// V2 | Entry created is verified.
/// V3 | Write multiple different entries without error.
/// V4 | Write more entries that buffer MAX without error.
/// V5 | Retrieve and verify entries in correct order. (latest to oldest)
fn kjournal_buffer_write() {
    // Used test buffer size is KJOURNAL_BUFFER_TEST_SIZE. If min is bigger, it become min. If max is lower, it become max.
    let buffer_size:usize = std::cmp::min(std::cmp::max(KJOURNAL_BUFFER_TEST_SIZE, KJOURNAL_BUFFER_MIN), KJOURNAL_BUFFER_MAX);
    let mut kb = KJournalBuffer::new(buffer_size).unwrap();

    // V1 | KJournalBuffer::write() write an entry into buffer without error.
    kb.write(KJournalEntrySeverity::DEBUG, "Debug entry");

    // V2 | Entry created is verified.
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::DEBUG, &"Debug entry".to_owned());

    // V3 | Write multiple different entries without error.
    // V4 | Write more entries that buffer MAX without error.
    for _ in 0..(KJOURNAL_BUFFER_MAX + 5) {
        kb.write(KJournalEntrySeverity::DEBUG, "Debug entry");
        kb.write(KJournalEntrySeverity::OTHER, "Other entry");
        kb.write(KJournalEntrySeverity::INFORMATION, "Information entry");
        kb.write(KJournalEntrySeverity::WARNING, "Warning entry");
        kb.write(KJournalEntrySeverity::ERROR, "Error entry");
        kb.write(KJournalEntrySeverity::FATAL, "Fatal entry");
    }

    // V5 | Retrieve and verify entries in correct order. (latest to oldest)
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::FATAL, "Fatal entry");
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::ERROR, "Error entry");
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::WARNING, "Warning entry");
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::INFORMATION, "Information entry");
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::OTHER, "Other entry");
    verify_journal_entry(kb.latest(), KJournalEntrySeverity::DEBUG, "Debug entry");

}

#[test]
/// Verify that buffer unread count is accurate.
/// 
/// # Verification(s)
/// V1 | KJournalBuffer::unread() initial size must be 0.
/// V2 | KJournalBuffer::unread() size must be 1 after writing an entry.
/// V3 | KJournalBuffer::unread() should go back to 0 after reading entry.
/// V4 | KJournalBuffer::unread() size should be buffer_size once full.
/// V5 | KJournalBuffer::unread() size should be buffer_size - 1 once an entry is read.
/// V6 | KJournalBuffer::unread() size should be buffer_size - (buffer_size / 2) - 1 after reading another buffer_size / 2 entries.
/// V7 | KJournalBuffer::clear() should clear the buffer without error.
/// V8 | KJournalBuffer::unread() size must be 0 after clear().
fn kjournal_buffer_unread() {
    // Used test buffer size is KJOURNAL_BUFFER_TEST_SIZE. If min is bigger, it become min. If max is lower, it become max.
    let buffer_size:usize = std::cmp::min(std::cmp::max(KJOURNAL_BUFFER_TEST_SIZE, KJOURNAL_BUFFER_MIN), KJOURNAL_BUFFER_MAX);

    let mut kb = KJournalBuffer::new(buffer_size).unwrap();

    // V1 | KJournalBuffer::unread() initial size must be 0.
    assert!(kb.unread() == 0, "Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);

    // Write debug entry
    kb.write(KJournalEntrySeverity::DEBUG, "Debug entry");

    // V2 | KJournalBuffer::unread() size must be 1 after writing an entry.
    assert!(kb.unread() == 1, "Unread ({}) count incorrect! Should be {}!", kb.unread(), 1);

    // Get latest
    kb.latest();

    // V3 | KJournalBuffer::unread() should go back to 0 after reading entry.
    assert!(kb.unread() == 0, "Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);

    // Write a bunch of entries.
    for _ in 0..(KJOURNAL_BUFFER_MAX + 5) {
        kb.write(KJournalEntrySeverity::DEBUG, "Debug entry");
        kb.write(KJournalEntrySeverity::OTHER, "Other entry");
        kb.write(KJournalEntrySeverity::INFORMATION, "Information entry");
        kb.write(KJournalEntrySeverity::WARNING, "Warning entry");
        kb.write(KJournalEntrySeverity::ERROR, "Error entry");
        kb.write(KJournalEntrySeverity::FATAL, "Fatal entry");

    }

    // V4 | KJournalBuffer::unread() size should be buffer_size once full.
    assert!(kb.unread() == kb.size(), "Unread ({}) count incorrect! Should be {}!", kb.unread(), buffer_size);

     // Get latest
     kb.latest();

     // V5 | KJournalBuffer::unread() size should be buffer_size - 1 once an entry is read.
    assert!(kb.unread() == kb.size() - 1, "Unread ({}) count incorrect! Should be {}!", kb.unread(), buffer_size - 1);

    let entries_read = buffer_size / 2;
    // Get 128 entries
    for _ in 0..entries_read{
        kb.latest();
    }

    // V6 | KJournalBuffer::unread() size should be buffer_size - (buffer_size / 2) - 1 after reading another buffer_size / 2 entries.
    assert!(kb.unread() == kb.size() - entries_read - 1, "Unread ({}) count incorrect! Should be {}!", kb.unread(), buffer_size - entries_read - 1);

    // V7 | KJournalBuffer::clear() should clear the buffer without error.
    kb.clear();

    // V8 | KJournalBuffer::unread() size must be 0 after clear().
    assert!(kb.unread() == 0, "Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);


}


#[test]
#[ignore]
/// Stress test KJournalBuffer to test stability and limit.
/// 
/// # Verification(s)
/// V1 | Multiple different buffer size ranging from MIN to MAX
/// V2 | Verify initial size() is 0.
/// V3 | Write an entry into buffer and verify size is 1.
/// V4 | Verify that entry values are correct.
/// V5 | Fill buffer with entries.
/// V6 | Verify unread() is equal to buffer_size.
/// V7 | Verify 6 latest entries.
/// V8 | Verify size() is equal to buffer_size - 6.
/// V9 | Read each entry until None is returned.
/// V10 | Verify that size() is now 0.
/// V11 | Repeat until MAX is reached.
fn kjournal_buffer_stress() {   
    // V1 | Multiple different buffer size ranging from MIN to MAX
    // V11 | Repeat until MAX is reached.
    for buffer_size in (KJOURNAL_BUFFER_MIN..KJOURNAL_BUFFER_MAX).step_by((KJOURNAL_BUFFER_MAX - KJOURNAL_BUFFER_MIN) / 100) {
        let mut kb = KJournalBuffer::new(buffer_size).unwrap();

        // V2 | Verify initial size() is 0.
        assert!(kb.unread() == 0, "Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);

        // V3 | Write an entry into buffer and verify size is 1.
        kb.write(KJournalEntrySeverity::DEBUG, "Debug entry");
        assert!(kb.unread() == 1, "Unread ({}) count incorrect! Should be {}!", kb.unread(), 1);

        // V4 | Verify that entry values are correct.
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::DEBUG, &"Debug entry".to_owned());

        // V5 | Fill buffer with entries.
        for _ in 0..buffer_size + 5{
            kb.write(KJournalEntrySeverity::DEBUG, "Debug entry");
            kb.write(KJournalEntrySeverity::OTHER, "Other entry");
            kb.write(KJournalEntrySeverity::INFORMATION, "Information entry");
            kb.write(KJournalEntrySeverity::WARNING, "Warning entry");
            kb.write(KJournalEntrySeverity::ERROR, "Error entry");
            kb.write(KJournalEntrySeverity::FATAL, "Fatal entry");

        }

        // V6 | Verify unread() is equal to buffer_size.
        assert!(kb.unread() == buffer_size, "Unread ({}) count incorrect! Should be {}!", kb.unread(), buffer_size);

        // V7 | Verify 6 latest entries.
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::FATAL, &"Fatal entry".to_owned());
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::ERROR, &"Error entry".to_owned());
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::WARNING, &"Warning entry".to_owned());
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::INFORMATION, &"Information entry".to_owned());
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::OTHER, &"Other entry".to_owned());
        verify_journal_entry(kb.latest(), KJournalEntrySeverity::DEBUG, &"Debug entry".to_owned());

        // V8 | Verify size() is equal to buffer_size - 6.
        assert!(kb.unread() == buffer_size - 6, "Unread ({}) count incorrect! Should be {}!", kb.unread(), buffer_size);

        // V9 | Read each entry until None is returned.
        for i in 0..buffer_size {
            match kb.latest() {
                Some(_) => assert!(i < buffer_size - 1, "None neaver reached while reading entries!"), 
                None => break,  // Break loop
            }
        }

        // V10 | Verify that size() is now 0.
        assert!(kb.unread() == 0, "Unread ({}) count incorrect! Should be {}!", kb.unread(), 0);
    }
}


/************
* FUNCTIONS * 
************/
/// Verify a journal entry with a severity and description control.
/// 
/// # Panic
/// Will panic if any entry parameters are wrong.
fn verify_journal_entry(entry: Option<&KJournalEntry>, severity:u8, desc : &str){

    match entry {
        Some(entry) => {
            assert!(entry.get_severity() == severity, "New entry severity incorrect. {} != {}!", entry.get_severity(), severity);
            assert!(entry.get_description().eq(desc), "New entry description incorrect!");
            match entry.get_date_time().elapsed() {
                // Elapsed should be really small.
                Ok(elapsed) => assert!(elapsed.as_millis() <= 10,  "New entry date and time incorrect!"),
                Err(_) => assert!(false, "New entry date and time incorrect!"),
            }
        },
        None => assert!(false, "Journal entry write failed!"),
    }

}