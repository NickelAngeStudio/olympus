use std::time::SystemTime;


/// ##### Journal entry. Contains the severity, the date and time of entry and a description.
/// 
/// # Example(s)
/// ```
/// // Import Journal entry and severity
/// use olympus_kleio::journal::{ KJournalEntry, KJournalEntrySeverity};
/// 
/// // Create an entry with severity and description as String. (Date and time is added automatically upon creation).
/// let mut j = KJournalEntry::new(KJournalEntrySeverity::ERROR, String::from("This is an example of an error entry!"));
/// 
/// // You can also recycle / update entries (ie. for circular buffer). (Date and time is added automatically updated).
/// j.update(KJournalEntrySeverity::INFORMATION, String::from("This is now a recycle entry!"));
/// ```
pub struct KJournalEntry {
    /// Severity of the entry according to [`super::KJournalEntrySeverity`].
    severity : u8,

    /// Date and time entry occurred.
    date_time : SystemTime,

    /// Entry description / Metadata.
    description : String,
}


impl KJournalEntry {
    /// Create a new [`KJournalEntry`] from [severity][`super::KJournalEntrySeverity`] and description. Date and time will be added automatically.
    /// 
    /// Return New [`KJournalEntry`] with new date and time.
    pub fn new(severity : u8, description : &str) -> KJournalEntry {
        KJournalEntry { severity, date_time: SystemTime::now(), description : description.to_string() }
    }

    /// Update Journal entry with a new [severity][`super::KJournalEntrySeverity`] and description. Date and time will be modified automatically.
    pub fn update(&mut self, severity : u8, description : &str){
        self.date_time = SystemTime::now();
        self.severity = severity;
        self.description = description.to_string();
    }

    /// Returns entry [severity][`super::KJournalEntrySeverity`].
    pub fn get_severity(&self) -> u8{
        self.severity
    }

    /// Returns entry date and time as [`SystemTime`].
    pub fn get_date_time(&self) -> SystemTime{
        self.date_time
    }

    /// Returns entry description / metadata.
    pub fn get_description(&self) -> &String {
        &self.description
    }
}
