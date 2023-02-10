use super::KJournalEntry;

/// Implementing this trait is needed to listen to new [KJournalEntry].
/// 
/// Listeners are notified of new journal entry according to [severity][super::KJournalEntrySeverity] they listen
/// to according to set_severity().
pub trait KJournalListener {
    /// Notification of new entry with an unmutable reference to it.
    fn notify(&self, entry : &KJournalEntry);

    /// Set the severity the listener will listen to.
    fn set_severity(&mut self, severity:u8);

    /// Get the severity the listener is listening to.
    fn get_severity(&self) -> u8;
}


/// List of listeners listening to the journal.
pub struct KJournalListenerList<'a> {

    listeners : Vec<&'a dyn KJournalListener>

}

/// Enumeration of possible error of [KJournalListenerList].
pub enum KJournalListenerListError {
    /// Happens when adding a listener that is already in the list.
    ListenerAlreadyExists,

    /// Happens when a listener is not found in the list.
    ListenerNotFound,
}

impl<'a> KJournalListenerList<'a> {

    /// Create a new instance of KJournalListenerList
    pub fn new() -> KJournalListenerList<'a>{
        // Create listeners vector.
        let listeners : Vec<&'a dyn KJournalListener> = Vec::new();

        // Create List instance
        KJournalListenerList { listeners }
    }

    /// Notify listeners with a new entry. Will notify listeners according to their severity settings.
    pub fn notify(&self, entry : &super::KJournalEntry) {
        for listener in &self.listeners {
            // Verify that listener is listening to this severity.
            if listener.get_severity() & entry.get_severity() > 0 {
                listener.notify(entry);
            }
        }
    }

    /// Add [KJournalListener] to the list.
    /// 
    /// Returns [OK(usize)][Ok] with index of new listener added.
    /// 
    
    pub fn add_listener(&mut self, listener : &'a (dyn KJournalListener + 'a)) -> Result<usize, KJournalListenerListError> {
        
        match self.get_listener_index(listener) {
            Ok(_) => Err(KJournalListenerListError::ListenerAlreadyExists),
            Err(_) => { self.listeners.push(listener); Ok(self.listeners.len() - 1) },
        }

    }

    /// Remove a [KJournalListener] from the list.
    /// 
    /// Returns [OK(usize)][Ok] with index of listener removed.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KJournalListenerListError::ListenerNotFound]`)` if listener not found.
    pub fn remove_listener(&mut self, listener : &dyn KJournalListener) -> Result<usize, KJournalListenerListError> {
        match self.get_listener_index(listener) {
            Ok(index) => { self.listeners.remove(index); Ok(index) },
            Err(_) => Err(KJournalListenerListError::ListenerNotFound),
        }
    }

    /// Returns the count of listeners.
    pub fn count(&self)->usize {
        self.listeners.len()
    }

    /// Clear the list of listeners.
    pub fn clear(&mut self) {
        self.listeners.clear();
    }

    /// Returns the index of a listener from the list.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KJournalListenerListError::ListenerNotFound]`)` if listener not found.
    fn get_listener_index(&self, listener : &dyn KJournalListener)-> Result<usize, KJournalListenerListError> {
        let mut found = false;
        let mut index: usize = 0;

        for i in 0..self.listeners.len() {
            if std::ptr::eq(listener, self.listeners[i]) {
                found = true;
                index = i;
                break;
            }
        }
        
        if found {
            Ok(index)
        }
        else {
            Err(KJournalListenerListError::ListenerNotFound)
        }
    }
}


