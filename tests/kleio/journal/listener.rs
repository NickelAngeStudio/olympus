use std::{rc::Rc, cell::RefCell};
use olympus::kleio::journal::{listener::{KJournalListenerList, KJournalListenerListError}, KJournalListenerPrint, KJournalEntrySeverity, KJournalListener, KJournalEntry};

#[test]
/// Create a new intance of KJournalListenerList.
/// 
/// # Verification(s)
/// V1 | KJournalListenerList::new() create a new instance without error.
fn kjournal_listener_list_new() {
    // V1 | KJournalListenerList::new() create a new instance without error.
    let _ = KJournalListenerList::new();
}

#[test]
/// Add listener to instance of KJournalListenerList.
/// 
/// # Verification(s)
/// V1 | Initial KJournalListenerList::count() should be 0.
/// V2 | KJournalListenerPrint::new() create a new instance without error.
/// V3 | KJournalListenerList::add_listener() add listener created.
/// V4 | KJournalListenerList::count() should be 1.
fn kjournal_listener_list_add_listener() {

    let mut list = KJournalListenerList::new();

    // V1 | Initial KJournalListenerList::count() should be 0.
    assert!(list.count() == 0, "KJournalListenerList::count() should be 0!");

    // V2 | KJournalListenerPrint::new() create a new instance without error
    let listener = KJournalListenerPrint::new(KJournalEntrySeverity::ALL_WITH_DEBUG);

    // V3 | KJournalListenerList::add_listener() add listener created.
    handle_listener_error(list.add_listener(&listener));

    // V4 | KJournalListenerList::count() should be 1.
    assert!(list.count() == 1, "KJournalListenerList::count() should be 1!");
}

#[test]
#[should_panic]
/// Add the same listener twice to KJournalListenerList.
/// 
/// # Verification(s)
/// V1 | KJournalListenerList::add_listener() should panic!
fn kjournal_listener_list_add_listener_twice() {
    let mut list = KJournalListenerList::new();
    let listener = KJournalListenerPrint::new(KJournalEntrySeverity::ALL_WITH_DEBUG);
    handle_listener_error(list.add_listener(&listener));


    // V1 | KJournalListenerList::add_listener() should panic!
    handle_listener_error(list.add_listener(&listener));
}


#[test]
/// Remove a listener from KJournalListenerList.
/// 
/// # Verification(s)
/// V1 | KJournalListener::remove_listener() should remove listener without error.
/// V2 | KJournalListenerList::count() should be 0 after removal.
fn kjournal_listener_list_remove_listener() {
    let mut list = KJournalListenerList::new();
    let listener = KJournalListenerPrint::new(KJournalEntrySeverity::ALL_WITH_DEBUG);
    handle_listener_error(list.add_listener(&listener));

    // V1 | KJournalListener::remove_listener() should remove listener without error.
    handle_listener_error(list.remove_listener(&listener));

    // V2 | KJournalListenerList::count() should be 0 after removal.
    assert!(list.count() == 0, "KJournalListenerList::count() should be 0!");
}

#[test]
#[should_panic]
/// Try to remove a listener from KJournalListenerList that was not added in the first place.
/// 
/// # Verification(s)
/// V1 | KJournalListener::remove_listener() should panic! trying to remove unknown listener.
fn kjournal_listener_list_remove_listener_not_added() {
    let mut list = KJournalListenerList::new();
    let listener = KJournalListenerPrint::new(KJournalEntrySeverity::ALL_WITH_DEBUG);

    // V1 | KJournalListener::remove_listener() should panic! trying to remove unknown listener.
    handle_listener_error(list.remove_listener(&listener));
}

#[test]
/// Clear a list of listeners.
/// 
/// # Verification(s)
/// V1 | KJournalListenerList::count() initial should be 0.
/// V2 | Add 3 different listeners to list.
/// V3 | KJournalListenerList::count() should be 3.
/// V4 | KJournalListenerList::clear() should remove all listeners without error.
/// V5 | KJournalListenerList::count() should be 0 after clearing.
fn kjournal_listener_list_clear() {
    let mut list = KJournalListenerList::new();

    // V1 | KJournalListenerList::count() initial should be 0.
    assert!(list.count() == 0, "KJournalListenerList::count() should be 0!");

    // V2 | Add 3 different listeners to list.
    let l1 = KJournalListenerPrint::new(KJournalEntrySeverity::ALL_WITH_DEBUG);
    let l2= KJournalListenerPrint::new(KJournalEntrySeverity::ALL_WITH_DEBUG);
    let l3= KJournalListenerPrint::new(KJournalEntrySeverity::ALL_WITH_DEBUG);
    handle_listener_error(list.add_listener(&l1));
    handle_listener_error(list.add_listener(&l2));
    handle_listener_error(list.add_listener(&l3));

    // V3 | KJournalListenerList::count() should be 3.
    assert!(list.count() == 3, "KJournalListenerList::count() should be 3!");

    // V4 | KJournalListenerList::clear() should remove all listeners without error.
    list.clear();

    // V5 | KJournalListenerList::count() should be 0 after clearing.
    assert!(list.count() == 0, "KJournalListenerList::count() should be 0!");
}


#[test]
/// Use an implementation of KJournalListener to verify if notify is called correctly.
/// 
/// # Verification(s)
/// V1 | Create a notified listener for each severity and one with 0 as severity.
/// V2 | Create differents combinations of 2, 3, 4, 5 and 6 severities.
/// V3 | Create a KJournalListenerPrint.
/// V4 | Add ALL listeners to list.
/// V5 | KJournalListenerList::count() should be 15.
/// v6 | Send a notification for each severity.
/// V7 | Send differents notifications combinations of 2, 3, 4, 5 and 6 severities.
/// V8 | Verify notification count of each NotifiedListener.
fn kjournal_listener_list_notify() {


    let mut list = KJournalListenerList::new();

    // V1 | Create a notified listener for each severity.
    let nl0 = NotifiedListener::new(0);
    let nl1= NotifiedListener::new(KJournalEntrySeverity::DEBUG);
    let nl2= NotifiedListener::new( KJournalEntrySeverity::OTHER);
    let nl3 = NotifiedListener::new( KJournalEntrySeverity::INFORMATION);
    let nl4= NotifiedListener::new( KJournalEntrySeverity::WARNING);
    let nl5= NotifiedListener::new( KJournalEntrySeverity::ERROR);
    let nl6 = NotifiedListener::new( KJournalEntrySeverity::FATAL);
    let nl7 = NotifiedListener::new( KJournalEntrySeverity::ALL_NO_DEBUG);
    let nl8 = NotifiedListener::new( KJournalEntrySeverity::ALL_WITH_DEBUG);
    
    // V2 | Create differents combinations of 2, 3, 4, 5 and 6 severities.
    let nls0= NotifiedListener::new( KJournalEntrySeverity::DEBUG | KJournalEntrySeverity::OTHER);
    let nls1 = NotifiedListener::new(  KJournalEntrySeverity::DEBUG | KJournalEntrySeverity::OTHER |
        KJournalEntrySeverity::INFORMATION);
    let nls2= NotifiedListener::new( KJournalEntrySeverity::DEBUG | KJournalEntrySeverity::OTHER |
        KJournalEntrySeverity::INFORMATION | KJournalEntrySeverity::WARNING);
    let nls3= NotifiedListener::new(  KJournalEntrySeverity::DEBUG | KJournalEntrySeverity::OTHER |
        KJournalEntrySeverity::INFORMATION | KJournalEntrySeverity::WARNING | KJournalEntrySeverity::ERROR);
    let nls4 = NotifiedListener::new(  KJournalEntrySeverity::DEBUG | KJournalEntrySeverity::OTHER |
        KJournalEntrySeverity::INFORMATION | KJournalEntrySeverity::WARNING | KJournalEntrySeverity::ERROR | KJournalEntrySeverity::FATAL);

    // V3 | Create a KJournalListenerPrint.
    let lp0 = KJournalListenerPrint::new(KJournalEntrySeverity::ALL_WITH_DEBUG);

    // V4 | Add ALL listeners to list.
    handle_listener_error(list.add_listener(&nl0));
    handle_listener_error(list.add_listener(&nl1));
    handle_listener_error(list.add_listener(&nl2));
    handle_listener_error(list.add_listener(&nl3));
    handle_listener_error(list.add_listener(&nl4));
    handle_listener_error(list.add_listener(&nl5));
    handle_listener_error(list.add_listener(&nl6));
    handle_listener_error(list.add_listener(&nl7));
    handle_listener_error(list.add_listener(&nl8));
    handle_listener_error(list.add_listener(&nls0));
    handle_listener_error(list.add_listener(&nls1));
    handle_listener_error(list.add_listener(&nls2));
    handle_listener_error(list.add_listener(&nls3));
    handle_listener_error(list.add_listener(&nls4));
    handle_listener_error(list.add_listener(&lp0));
    
    // V5 | KJournalListenerList::count() should be 15.
    assert!(list.count() == 15, "KJournalListenerList::count() should be 15!");

    // v6 | Send a notification for each severity.
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::DEBUG, "DEBUG"));
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::OTHER, "OTHER"));
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::INFORMATION, "INFORMATION"));
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::WARNING, "WARNING"));
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::ERROR, "ERROR"));
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::FATAL, "FATAL"));

    // V7 | Send differents notifications combinations of 2, 3, 4, 5 and 6 severities.
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::DEBUG | KJournalEntrySeverity::OTHER, "DO"));
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::DEBUG | KJournalEntrySeverity::OTHER |
        KJournalEntrySeverity::INFORMATION, "DOI"));
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::DEBUG | KJournalEntrySeverity::OTHER |
        KJournalEntrySeverity::INFORMATION | KJournalEntrySeverity::WARNING, "DOIW"));
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::DEBUG | KJournalEntrySeverity::OTHER |
        KJournalEntrySeverity::INFORMATION | KJournalEntrySeverity::WARNING | KJournalEntrySeverity::ERROR, "DOIWE"));
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::DEBUG | KJournalEntrySeverity::OTHER |
        KJournalEntrySeverity::INFORMATION | KJournalEntrySeverity::WARNING | KJournalEntrySeverity::ERROR | KJournalEntrySeverity::FATAL, "DOIWEF"));

    // V8 | Verify notification count of each NotifiedListener.
    assert!(nl0.get_notification_count() == 0, "NotifiedListener0::get_notification_count() should be 0 instead of {}!", nl0.get_notification_count());
    assert!(nl1.get_notification_count() == 6, "NotifiedListener1::get_notification_count() should be 6 instead of {}!", nl1.get_notification_count());
    assert!(nl2.get_notification_count() == 6, "NotifiedListener2::get_notification_count() should be 6 instead of {}!", nl2.get_notification_count());
    assert!(nl3.get_notification_count() == 5, "NotifiedListener3::get_notification_count() should be 5 instead of {}!", nl3.get_notification_count());
    assert!(nl4.get_notification_count() == 4, "NotifiedListener4::get_notification_count() should be 4 instead of {}!", nl4.get_notification_count());
    assert!(nl5.get_notification_count() == 3, "NotifiedListener5::get_notification_count() should be 3 instead of {}!", nl5.get_notification_count());
    assert!(nl6.get_notification_count() == 2, "NotifiedListener6::get_notification_count() should be 2 instead of {}!", nl6.get_notification_count());
    assert!(nl7.get_notification_count() == 10, "NotifiedListener7::get_notification_count() should be 10 instead of {}!", nl7.get_notification_count());
    assert!(nl8.get_notification_count() == 11, "NotifiedListener8::get_notification_count() should be 11 instead of {}!", nl8.get_notification_count());

    assert!(nls0.get_notification_count() == 7, "NotifiedListenerS0::get_notification_count() should be 7! instead of {}!", nls0.get_notification_count());
    assert!(nls1.get_notification_count() == 8, "NotifiedListenerS1::get_notification_count() should be 8! instead of {}!", nls1.get_notification_count());
    assert!(nls2.get_notification_count() == 9, "NotifiedListenerS2::get_notification_count() should be 9! instead of {}!", nls2.get_notification_count());
    assert!(nls3.get_notification_count() == 10, "NotifiedListenerS3::get_notification_count() should be 10! instead of {}!", nls3.get_notification_count());
    assert!(nls4.get_notification_count() == 11, "NotifiedListenerS4::get_notification_count() should be 11! instead of {}!", nls4.get_notification_count());
 
}

#[test]
/// Notify an empty list of listeners.
/// 
/// # Verification(s)
/// V1 | KJournalListener::notify() should NOT panic! when empty and notify is called.
fn kjournal_listener_list_notify_empty() {
    
    let list = KJournalListenerList::new();

    // V1 | KJournalListener::notify() should NOT panic! when empty and notify is called.
    list.notify(&KJournalEntry::new(KJournalEntrySeverity::DEBUG, "DEBUG"));
}

/************
* FUNCTIONS * 
************/
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
        a.as_ref().take()
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