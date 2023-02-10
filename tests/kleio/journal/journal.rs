use std::{cell::RefCell, rc::Rc};
use olympus::kleio::journal::{ KJournal, KJournalEntrySeverity, KJOURNAL_BUFFER_MIN, KJOURNAL_BUFFER_MAX, KJournalEntry, KJournalListener, listener::KJournalListenerListError };

#[test]
/// Create a new instance of KJournal.
/// 
/// # Verification(s)
/// V1 | New KJournal created without error.
/// V2 | Verify KJournal name with control.
fn kjournal_new() {
    // V1 | New KJournal created without error.
    match KJournal::new("J1", KJournalEntrySeverity::ALL_WITH_DEBUG, KJOURNAL_BUFFER_MIN) {
        Ok(j) => {
            // V2 | Verify KJournal name with control.
            assert!(j.get_name().eq(&"J1".to_string()), "KJournal name error!");
        },
        Err(_) => panic!("Error while creating KJournal"),
    }
}

#[test]
/// Test creating KJournal with buffer size < KJOURNAL_BUFFER_MIN and buffer size > KJOURNAL_BUFFER_MAX.
/// 
/// # Verification(s)
/// V1 | New KJournal created with buffer size < KJOURNAL_BUFFER_MIN must return Err(KJournalError::BufferSizeTooSmall).
/// V2 | New KJournal created with buffer size > KJOURNAL_BUFFER_MAX must return Err(KJournalError::BufferSizeTooBig).
fn kjournal_new_limit() {

    // V1 | New KJournal created with buffer size < KJOURNAL_BUFFER_MIN must return Err(KJournalError::BufferSizeTooSmall).
    match KJournal::new("J1", KJournalEntrySeverity::ALL_WITH_DEBUG, KJOURNAL_BUFFER_MIN - 1) {
        Ok(_) => panic!("KJournal must return Err(KJournalError::BufferSizeTooSmall)!"),
        Err(err) => match err {
            olympus::kleio::journal::journal::KJournalError::BufferSizeTooSmall => {},
            _ => panic!("KJournal must return Err(KJournalError::BufferSizeTooSmall)!"),
        },
    }
    
    // V2 | New KJournal created with buffer size > KJOURNAL_BUFFER_MAX must return Err(KJournalError::BufferSizeTooBig).
    match KJournal::new("J1", KJournalEntrySeverity::ALL_WITH_DEBUG, KJOURNAL_BUFFER_MAX + 1) {
        Ok(_) => panic!("KJournal must return Err(KJournalError::BufferSizeTooBig)!"),
        Err(err) => match err {
            olympus::kleio::journal::journal::KJournalError::BufferSizeTooBig => {},
            _ => panic!("KJournal must return Err(KJournalError::BufferSizeTooBig)!"),
        },
    }

}

#[test]
/// Write and read multiple entries in KJournal.
/// 
/// This test is more exhaustive in buffer.rs.
/// 
/// # Verification(s)
/// V1 | Unread count should be 0 at first.
/// V2 | 10 new entries written without error.
/// V3 | Entries count should be 10.
/// V4 | 10 entries read without error.
/// V5 | Entries count should be 0.
fn kjournal_write_read() {
    
    let mut j = KJournal::new("J1", KJournalEntrySeverity::ALL_WITH_DEBUG, KJOURNAL_BUFFER_MIN).unwrap();

    // V1 | Unread count should be 0 at first.
    assert!(j.unread() == 0, "Unread count should be 0 at first!");

    // V2 | 10 new entries written without error.
    write_10_journal_entries(&mut j);

    // V3 | Entries count should be 10.
    assert!(j.unread() == 10, "Unread count should be 10!");

    // V4 | 10 entries read without error.
    verify_journal_entry(j.read(), KJournalEntrySeverity::FATAL, &"Fatal entry".to_owned());
    verify_journal_entry(j.read(), KJournalEntrySeverity::ERROR, &"Error entry".to_owned());
    verify_journal_entry(j.read(), KJournalEntrySeverity::WARNING, &"Warning entry".to_owned());
    verify_journal_entry(j.read(), KJournalEntrySeverity::INFORMATION, &"Information entry".to_owned());
    verify_journal_entry(j.read(), KJournalEntrySeverity::FATAL, &"Fatal entry".to_owned());
    verify_journal_entry(j.read(), KJournalEntrySeverity::ERROR, &"Error entry".to_owned());
    verify_journal_entry(j.read(), KJournalEntrySeverity::WARNING, &"Warning entry".to_owned());
    verify_journal_entry(j.read(), KJournalEntrySeverity::INFORMATION, &"Information entry".to_owned());
    verify_journal_entry(j.read(), KJournalEntrySeverity::OTHER, &"Other entry".to_owned());
    verify_journal_entry(j.read(), KJournalEntrySeverity::DEBUG, &"Debug entry".to_owned());

    // V5 | Entries count should be 0.
    assert!(j.unread() == 0, "Unread count should be 0!");

}

#[test]
/// Clear the KJournal
/// 
/// This test is more exhaustive in buffer.rs.
/// 
/// # Verification(s)
/// V1 | Unread count should be 0 at first.
/// V2 | 10 new entries written without error.
/// V3 | Entries count should be 10.
/// V4 | Clear entries without error.
/// V5 | Entries count should be 0.
fn kjournal_clear() {

    let mut j = KJournal::new("J1", KJournalEntrySeverity::ALL_WITH_DEBUG, KJOURNAL_BUFFER_MIN).unwrap();


    // V1 | Unread count should be 0 at first.
    assert!(j.unread() == 0, "Unread count should be 0 at first!");

    // V2 | 10 new entries written without error.
    write_10_journal_entries(&mut j);

    // V3 | Entries count should be 10.
    assert!(j.unread() == 10, "Unread count should be 10!");

    // V4 | Clear entries without error.
    j.clear();

    // V5 | Entries count should be 0.
    assert!(j.unread() == 0, "Unread count should be 0!");
}

#[test]
/// Add and remove listeners and verify that notify are given to the correct severity.
/// Also verify Err() given by add_listener and remove_listener
/// 
/// # Verification(s)
/// V1 | Try to remove listener without it being added. Must return Err(ListenerNotFound).
/// V2 | Add a listener to KJournal.
/// V3 | Add the same listener. Must return Err(ListenerAlreadyExists).
/// V4 | Add multiple listener with different severity to listen to.
/// V5 | Write multiple entries and verify if listener listened.
/// V6 | Modify KJournal severity without error.
/// V7 | Verify new severity set with get_severity().
/// V8 | Write multiple entries and verify if listener listened with new severity.
/// V9 | Remove all listeners successfully.
fn kjournal_listeners() {
    
    let mut j = KJournal::new("J1", KJournalEntrySeverity::ALL_WITH_DEBUG, KJOURNAL_BUFFER_MIN).unwrap();

    // V1 | Try to remove listener without it being added. Must return Err(ListenerNotFound).
    let nl0 = NotifiedListener::new(0);
    match j.remove_listener(&nl0){
        Ok(_) => panic!("Cannot remove listener not added!"),
        Err(err) => match err {
            olympus::kleio::journal::listener::KJournalListenerListError::ListenerNotFound => {},
            _ => panic!("Wrong error given!"),
        },
    }

    // V2 | Add a listener to KJournal.
    handle_listener_error(j.add_listener(&nl0));

    // V3 | Add the same listener. Must return Err(ListenerAlreadyExists).
    match j.add_listener(&nl0){
        Ok(_) => panic!("Must not be able to add same listener twice!"),
        Err(err) => match err {
            olympus::kleio::journal::listener::KJournalListenerListError::ListenerAlreadyExists => {},
            _ => panic!("Wrong error given!"),
        },
    }

    // V4 | Add multiple listener with different severity to listen to.
    let nl1= NotifiedListener::new(KJournalEntrySeverity::DEBUG);
    let nl2= NotifiedListener::new( KJournalEntrySeverity::OTHER);
    let nl3 = NotifiedListener::new( KJournalEntrySeverity::INFORMATION);
    let nl4= NotifiedListener::new( KJournalEntrySeverity::WARNING);
    let nl5= NotifiedListener::new( KJournalEntrySeverity::ERROR);
    let nl6 = NotifiedListener::new( KJournalEntrySeverity::FATAL);
    let nl7 = NotifiedListener::new( KJournalEntrySeverity::ALL_NO_DEBUG);
    let nl8 = NotifiedListener::new( KJournalEntrySeverity::ALL_WITH_DEBUG);
    handle_listener_error(j.add_listener(&nl1));
    handle_listener_error(j.add_listener(&nl2));
    handle_listener_error(j.add_listener(&nl3));
    handle_listener_error(j.add_listener(&nl4));
    handle_listener_error(j.add_listener(&nl5));
    handle_listener_error(j.add_listener(&nl6));
    handle_listener_error(j.add_listener(&nl7));
    handle_listener_error(j.add_listener(&nl8));

    // V5 | Write multiple entries and verify if listener listened.
    write_10_journal_entries(&mut j);

    assert!(nl0.get_notification_count() == 0, "NotifiedListener0::get_notification_count() should be 0 instead of {}!", nl0.get_notification_count());
    assert!(nl1.get_notification_count() == 1, "NotifiedListener1::get_notification_count() should be 1 instead of {}!", nl1.get_notification_count());
    assert!(nl2.get_notification_count() == 1, "NotifiedListener2::get_notification_count() should be 1 instead of {}!", nl2.get_notification_count());
    assert!(nl3.get_notification_count() == 2, "NotifiedListener3::get_notification_count() should be 2 instead of {}!", nl3.get_notification_count());
    assert!(nl4.get_notification_count() == 2, "NotifiedListener4::get_notification_count() should be 2 instead of {}!", nl4.get_notification_count());
    assert!(nl5.get_notification_count() == 2, "NotifiedListener5::get_notification_count() should be 2 instead of {}!", nl5.get_notification_count());
    assert!(nl6.get_notification_count() == 2, "NotifiedListener6::get_notification_count() should be 2 instead of {}!", nl6.get_notification_count());
    assert!(nl7.get_notification_count() == 9, "NotifiedListener7::get_notification_count() should be 9 instead of {}!", nl7.get_notification_count());
    assert!(nl8.get_notification_count() == 10, "NotifiedListener8::get_notification_count() should be 10 instead of {}!", nl8.get_notification_count());

    // V6 | Modify KJournal severity without error.
    j.set_severity(KJournalEntrySeverity::WARNING);

    // V7 | Verify new severity set with get_severity().
    assert!(j.get_severity() == KJournalEntrySeverity::WARNING, "Wrong severity set!");

    // V8 | Write multiple entries and verify if listener listened with new severity.
    write_10_journal_entries(&mut j);
    assert!(nl0.get_notification_count() == 0, "NotifiedListener0::get_notification_count() should be 0 instead of {}!", nl0.get_notification_count());
    assert!(nl1.get_notification_count() == 1, "NotifiedListener1::get_notification_count() should be 1 instead of {}!", nl1.get_notification_count());
    assert!(nl2.get_notification_count() == 1, "NotifiedListener2::get_notification_count() should be 1 instead of {}!", nl2.get_notification_count());
    assert!(nl3.get_notification_count() == 2, "NotifiedListener3::get_notification_count() should be 2 instead of {}!", nl3.get_notification_count());
    assert!(nl4.get_notification_count() == 4, "NotifiedListener4::get_notification_count() should be 4 instead of {}!", nl4.get_notification_count());
    assert!(nl5.get_notification_count() == 2, "NotifiedListener5::get_notification_count() should be 2 instead of {}!", nl5.get_notification_count());
    assert!(nl6.get_notification_count() == 2, "NotifiedListener6::get_notification_count() should be 2 instead of {}!", nl6.get_notification_count());
    assert!(nl7.get_notification_count() == 11, "NotifiedListener7::get_notification_count() should be 11 instead of {}!", nl7.get_notification_count());
    assert!(nl8.get_notification_count() == 12, "NotifiedListener8::get_notification_count() should be 12 instead of {}!", nl8.get_notification_count());

    // V9 | Remove all listeners successfully.
    handle_listener_error(j.remove_listener(&nl0));
    handle_listener_error(j.remove_listener(&nl1));
    handle_listener_error(j.remove_listener(&nl2));
    handle_listener_error(j.remove_listener(&nl3));
    handle_listener_error(j.remove_listener(&nl4));
    handle_listener_error(j.remove_listener(&nl5));
    handle_listener_error(j.remove_listener(&nl6));
    handle_listener_error(j.remove_listener(&nl7));
    handle_listener_error(j.remove_listener(&nl8));

}


#[test]
/// Test set_max_entries() limit
/// 
/// # Verification(s)
/// V1 | set_max_entries() buffer size < KJOURNAL_BUFFER_MIN must return Err(KJournalError::BufferSizeTooSmall).
/// V2 | set_max_entries() with buffer size > KJOURNAL_BUFFER_MAX must return Err(KJournalError::BufferSizeTooBig).
fn kjournal_set_max_entries_limit() {
    let mut j = KJournal::new("J1", KJournalEntrySeverity::ALL_WITH_DEBUG, KJOURNAL_BUFFER_MIN).unwrap();

    // V1 | set_max_entries() buffer size < KJOURNAL_BUFFER_MIN must return Err(KJournalError::BufferSizeTooSmall).
    match j.set_max_entries(KJOURNAL_BUFFER_MIN - 1) {
        Ok(_) => panic!("set_max_entries() must return Err(KJournalError::BufferSizeTooSmall)!"),
        Err(err) => match err {
            olympus::kleio::journal::journal::KJournalError::BufferSizeTooSmall => {},
            _ => panic!("set_max_entries() must return Err(KJournalError::BufferSizeTooSmall)!"),
        },
    }

    // V2 | set_max_entries() with buffer size > KJOURNAL_BUFFER_MAX must return Err(KJournalError::BufferSizeTooBig).
    match j.set_max_entries(KJOURNAL_BUFFER_MAX + 1) {
        Ok(_) => panic!("set_max_entries() must return Err(KJournalError::BufferSizeTooBig)!"),
        Err(err) => match err {
            olympus::kleio::journal::journal::KJournalError::BufferSizeTooBig => {},
            _ => panic!("set_max_entries() must return Err(KJournalError::BufferSizeTooBig)!"),
        },
    }
    
}

#[test]
/// Set a new max_entries buffer size.
/// 
/// # Verification(s)
/// V1 | get_max_entries() gives the correct KJournal buffer creation size.
/// V2 | set_max_entries() create a new buffer without error.
/// V3 | get_max_entries() gives the new KJournal buffer size.
fn kjournal_set_max_entries() {
    let mut j = KJournal::new("J1", KJournalEntrySeverity::ALL_WITH_DEBUG, KJOURNAL_BUFFER_MIN).unwrap();

    // V1 | get_max_entries() gives the correct KJournal buffer creation size.
    assert!(j.get_max_entries() == KJOURNAL_BUFFER_MIN, "KJournal buffer creation size error!");

    // V2 | set_max_entries() create a new buffer without error.
    match j.set_max_entries(KJOURNAL_BUFFER_MAX){
        Ok(_) => {},
        Err(_) => panic!("Error while setting new max entries!"),
    }

    // V3 | get_max_entries() gives the new KJournal buffer size.
    assert!(j.get_max_entries() == KJOURNAL_BUFFER_MAX, "New buffer size error!");
}



/*************
* STRUCTURES * 
*************/

/// ##### Custom listeners that count how many time it was notified.
struct NotifiedListener {
    // Severity to listen to
    severity : u8,

    // Was listener notified.
    notification_count: Rc<RefCell<usize>>,
}

impl NotifiedListener {
    /// Create a new instance of NotifiedListener with severity listened.
    /// 
    /// Returns new NotifiedListener created.
    pub fn new(severity:u8) -> NotifiedListener {
        NotifiedListener { severity, notification_count : Rc::new(RefCell::new(0)) }
    }

    /// Get the count of notifications.
    pub fn get_notification_count(&self) -> usize {
        let a = self.notification_count.clone();
        a.as_ref().clone().take()
    }
}

impl<'a> KJournalListener for NotifiedListener{
    fn notify(&self, _new_entry : &KJournalEntry){
        let a = self.notification_count.clone();

        let b = a.as_ref();

       
        b.replace(b.take() + 1);
    }

    fn set_severity(&mut self, severity:u8){
        self.severity = severity;
    }

    fn get_severity(&self) -> u8{
        self.severity
    }
}


/************
* FUNCTIONS * 
************/
/// Verify a journal entry with a severity and description control.
/// 
/// # Panic
/// Will panic if any entry parameters are wrong.
fn verify_journal_entry(entry: Option<&KJournalEntry>, severity:u8, desc : &String){

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

/// Panic if add_listener or remove_listener result in error.
fn handle_listener_error(res : Result<usize, KJournalListenerListError>){
    
    match  res{
        Ok(_) => {},
        Err(err) => match err {
            KJournalListenerListError::ListenerAlreadyExists => panic!("Listener already exists!"),
            KJournalListenerListError::ListenerNotFound => panic!("Listener not found!"),
        }
        ,
    }

}

/// Write 10 journals entries
fn write_10_journal_entries(j : &mut KJournal){
    j.write(KJournalEntrySeverity::DEBUG, "Debug entry");
    j.write(KJournalEntrySeverity::OTHER, "Other entry");
    j.write(KJournalEntrySeverity::INFORMATION, "Information entry");
    j.write(KJournalEntrySeverity::WARNING, "Warning entry");
    j.write(KJournalEntrySeverity::ERROR, "Error entry");
    j.write(KJournalEntrySeverity::FATAL, "Fatal entry");
    j.write(KJournalEntrySeverity::INFORMATION, "Information entry");
    j.write(KJournalEntrySeverity::WARNING, "Warning entry");
    j.write(KJournalEntrySeverity::ERROR, "Error entry");
    j.write(KJournalEntrySeverity::FATAL, "Fatal entry");

}