use std::{io::{Read, ErrorKind}, path::PathBuf};

use super::KAssetSource;

/// Middle men between [`0..n`] [KAssetSource] to supply assets according to source priority.
/// 
/// [KAssetSource] could be the file system, a database, a blob, etc... depending on the trait implementation.
/// 
/// # Priorities
/// Sources have priorities (0 > n) and the broker will try to get an asset from higher priorities first. This is useful 
/// when handling mods or other asset modification. The base file should be the lowest priority
/// and mods the highest.
/// 
/// # Example(s)
/// ##### Creating and adding source in [KAssetBroker]
/// `Note that this example won't run since 'myfolder0', 'myfolder1' don't exists.`
/// ```no_run
/// // Import needed components.
/// use std::path::PathBuf;
/// use olympus_kleio::asset::{KAssetBroker, KAssetSourceFolder, KAssetSource};
/// 
/// // Create KAssetBroker as mutable since we add KAssetSource to it.
/// let mut kab = KAssetBroker::new();
/// 
/// // Create KAssetSources.
/// let kaf0 = KAssetSourceFolder::new(PathBuf::from("myfolder0"));
/// let kaf1 = KAssetSourceFolder::new(PathBuf::from("myfolder1"));
/// 
/// // Add sources to broker. Will panic if an error occurred.
/// if let Err(_) = kab.add_source(&kaf0) {
///     panic!("Cannot add KAssetSource0 to broker.");
/// }
/// 
/// if let Err(_) = kab.add_source(&kaf1) {
///     panic!("Cannot add KAssetSource1 to broker.");
/// }
/// 
/// // Get asset from broker. Will search if asset is in kaf0 then kaf1.
/// if let Ok(mut asset) = kab.get_asset(PathBuf::from("myasset.txt")){
///     // Asset implements the trait Read. Here we read the asset into a string and print it.
///     let mut str:String = String::new();
///     if let Ok(_) = asset.read_to_string(&mut str){
///         println!("{:?}", str);
///     }
/// } else {
///     panic!("Cannot get asset 'myasset.txt'");
/// }   
/// ```
pub struct KAssetBroker<'a> {

    // Vector of sources. Position 0 is highest priority.
    sources: Vec<&'a dyn KAssetSource>,
}

/// Enumeration of possible errors that can happens within [KAssetBroker].
pub enum KAssetBrokerError {
    
    /// Happens when a [KAssetSource] is not found within the broker.
    SourceNotFound,

    /// Happens when adding the same [KAssetSource] twice to the broker.
    SourceAlreadyExists,
    
    /// Happens when new priority set for [KAssetSource] is higher then the length of sources.
    PriorityOutOfBound,

}


impl<'a> KAssetBroker<'a> {

    /// Create a new instance of KAssetBroker.
    pub fn new() -> KAssetBroker<'a> {
        // Create sources vector
        let sources : Vec<&'a dyn KAssetSource> = Vec::new();

        // Return new data broker
        KAssetBroker { sources }
    }

    /// Add a [KAssetSource] reference to the broker. Added [KAssetSource] are always last in priority.
    /// 
    /// Returns [`Ok<usize>`][Ok<usize>] with the priority of source added if successful.
    /// # Error(s)
    /// Returns `Err(`[KAssetBrokerError::SourceAlreadyExists]`)`  if [KAssetSource] is already within the broker.
    pub fn add_source(&mut self, source : &'a dyn KAssetSource) -> Result<usize, KAssetBrokerError>{

        if self.has_source(source) == false {
            let priority = self.sources.len();
            // Add source to broker
            self.sources.push(source);

            Ok(priority)
        } else {
            Err(KAssetBrokerError::SourceAlreadyExists)
        }
    }

    /// Remove the [KAssetSource] reference from the broker. 
    /// 
    /// Returns [`Ok<usize>`][Ok<usize>] with the priority of [KAssetSource] removed if successful.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KAssetBrokerError::SourceNotFound]`)` if [KAssetSource] is not found.
    pub fn remove_source(&mut self, source : &'a dyn KAssetSource) -> Result<usize, KAssetBrokerError>{

        let priority = self.get_source_priority(source);

        match priority {
            Ok(priority) => {
                self.sources.remove(priority);
                Ok(priority)
            },
            Err(_) => Err(KAssetBrokerError::SourceNotFound),
        }
    }

    /// Set a [KAssetSource] priority. Will change other sources priorities.
    /// 
    /// Returns [`Ok<usize>`][Ok<usize>] with the new priority of [KAssetSource] if successful.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KAssetBrokerError::PriorityOutOfBound]`)` if the `priority` > broker sources length.
    /// 
    /// Returns `Err(`[KAssetBrokerError::SourceNotFound]`)` if [KAssetSource] is not found.
    pub fn set_source_priority(&mut self, source : &'a dyn KAssetSource, priority : usize)-> Result<usize, KAssetBrokerError>{

        // Get current position / priority of the source
        let position = self.get_source_priority(source);

        match position {
            Ok(mut position) => {

                if priority < self.sources.len() {

                    // Will replace the position at the correct place
                    while priority > position {
                        let temp = self.sources[position];
                        self.sources[position] = self.sources[position + 1];
                        self.sources[position + 1] = temp;
                        position += 1;
                            
                    }
                    while priority < position {
                        let temp = self.sources[position];
                        self.sources[position] = self.sources[position - 1];
                        self.sources[position - 1] = temp;                            
                        position -= 1;
                    }

                    Ok(priority)

                } else {
                    Err(KAssetBrokerError::PriorityOutOfBound)
                }

            },
            Err(_) => Err(KAssetBrokerError::SourceNotFound),
        }

    }

    /// Get an immutable reference to the broker [KAssetSource] vector.
    pub fn get_sources(&self) -> &Vec<&'a dyn KAssetSource> {
        &self.sources
    }

    /// Fetch an asset in sources from path.
    /// 
    /// Returns `Ok(Box(`[Read]`))` if asset found.
    /// 
    /// # Error(s)
    /// Returns [std::io::Error] if asset not found or IO error occurs.
    pub fn get_asset(&self, path: PathBuf) ->  Result<Box<dyn Read>, std::io::Error>{

        // Use for 0.. as priority
        for n in 0..self.sources.len() {
            let src = self.sources[n];
            
            // If sources has asset, return it
            if src.has_asset(path.clone()) {
                return src.get_asset(path);
            } 
        }

        Err(std::io::Error::new(ErrorKind::NotFound, "Asset not found!"))
    }

    /// Get the priority/position of the given [KAssetSource].
    /// 
    /// Returns [`Ok<usize>`][Ok<usize>] with the priority if found.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KAssetBrokerError::SourceNotFound]`)` if [KAssetSource] is not found.
    pub fn get_source_priority(&self, source : &'a dyn KAssetSource) -> Result<usize, KAssetBrokerError>{

        for n in 0..self.sources.len() {
            if std::ptr::eq(self.sources[n], source) {
                return Ok(n) 
            }
        } 

        Err(KAssetBrokerError::SourceNotFound)
    }

    /// Verify if broker contains given [KAssetSource].
    /// 
    /// Returns True if broker contain [KAssetSource], false otherwise
    pub fn has_source(&self, source : &'a dyn KAssetSource) -> bool{
        match  self.get_source_priority(source) {
            Ok(_) => true,
            Err(_) => false,
        } 
    }

}
