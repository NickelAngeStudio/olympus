use std::{path::{PathBuf}, fs::{File}, io::Read, time::{SystemTime}};
use crate::kleio::asset::KAssetSource;

/// ##### [KAssetSource] implementation using a file system folder.
/// 
/// KAssetSourceFolder uses a given folder as a base path to retrieve assets.
/// 
/// ```no_run
/// // Import crate module
/// use std::path::PathBuf;
/// use olympus::kleio::asset::{KAssetSource , KAssetSourceFolder};
/// 
/// // Create KAssetSourceFolder using a folder as base
/// let source : KAssetSourceFolder = KAssetSourceFolder::new(PathBuf::from("/base_folder"));
/// 
/// // Get assets from KAssetSourceFolder with path relative to base
/// let asset1 = source.get_asset(PathBuf::from("audio/audio1.ogg"));
/// let asset2 = source.get_asset(PathBuf::from("models/model.obj"));
/// ```
pub struct KAssetSourceFolder {
    // Path of the source folder
    folder_path : PathBuf,

    // Metadata of KAssetSourceFolder (folderpath, created, modified)
    metadata : String
}

/// Enumeration of possible [KAssetSourceFolder] errors.
pub enum KAssetSourceFolderError {
    /// Happens when [`folder_path`][PathBuf] used to create [KAssetSourceFolder] is not found.
    FolderNotFound,

    /// Happens when [`folder_path`][PathBuf] used to create [KAssetSourceFolder] is not a folder.
    PathIsNotFolder,

    /// Happens when an error occurred while creating folder metadata.
    MetadataCreationError,
}

impl std::fmt::Debug for KAssetSourceFolderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FolderNotFound => write!(f, "FolderNotFound"),
            Self::PathIsNotFolder => write!(f, "PathIsNotFolder"),
            Self::MetadataCreationError => write!(f, "MetadataCreationError"),
        }
    }
}

impl KAssetSourceFolder {
    /// Create a new [KAssetSourceFolder] from a [`folder_path`][PathBuf].
    /// 
    /// Metadata JSON format :<br>
    /// {<br>
    ///     "path" : "\path\to\folder",<br>
    ///     "created" : "{epoch_time}",<br>
    ///     "modified" : "{epoch_time}",<br>
    /// }
    /// 
    /// Returns `Ok(`[KAssetSourceFolder]`)` if successful.
    /// 
    /// # Error(s)
    /// Returns `Err(`[KAssetSourceFolderError::FolderNotFound]`)` if folder is not found.
    /// 
    /// Returns `Err(`[KAssetSourceFolderError::PathIsNotFolder]`)` if [`folder_path`][PathBuf] is not a folder.
    /// 
    /// Returns `Err(`[KAssetSourceFolderError::MetadataCreationError]`)` if an error occurred while creating metadata.
    pub fn new(folder_path : PathBuf) -> Result<KAssetSourceFolder, KAssetSourceFolderError> {


        if !folder_path.exists() {
            return Err(KAssetSourceFolderError::FolderNotFound);
        }

        if !folder_path.is_dir(){
            return Err(KAssetSourceFolderError::PathIsNotFolder);
        }

        match  Self::create_metadata(&folder_path)  {
            Ok(metadata) => Ok(KAssetSourceFolder {
                metadata,
                folder_path,
                
            }),
            Err(_) => Err(KAssetSourceFolderError::MetadataCreationError),
        }
        
    }

    /// Create KAssetSourceFolder metadata which contains path, created and modified in JSON format.
    /// 
    /// Return `Ok(String)` with metadata created
    /// or Err(KAssetSourceFolderError::MetadataCreationError) if an error occurred.
    fn create_metadata(folder_path : &PathBuf)->Result<String, KAssetSourceFolderError> {

        let mut metadata : String = "{ \"path\":\"".to_owned() +  &folder_path.clone().into_os_string().into_string().unwrap().to_owned() + &"\",".to_owned();

        // Get folder metadata
        match folder_path.metadata(){
            // If OK, retieve date created and modified
            Ok(md) => {

                // Date created, UNIX format
                match md.created(){
                    Ok(st) => {
                        match st.duration_since(SystemTime::UNIX_EPOCH) {
                            Ok(n) => metadata.push_str(&("\"created\":\"".to_owned() + &n.as_millis().to_string().to_owned() + &"\",".to_owned())),
                            Err(_) => return Err(KAssetSourceFolderError::MetadataCreationError),
                        }
                        
                    },
                    Err(_) => {
                        metadata.push_str("\"created\":\"(metadata _error)\",");
                    },
                }

                // Date modified, UNIX format
                match md.modified(){
                    Ok(st) => {
                        match st.duration_since(SystemTime::UNIX_EPOCH) {
                            Ok(n) => metadata.push_str(&("\"modified\":\"".to_owned() + &n.as_millis().to_string().to_owned() + &"\",".to_owned())),
                            Err(_) => return Err(KAssetSourceFolderError::MetadataCreationError),
                        }
                        
                    },
                    Err(_) => metadata.push_str("\"modified\":\"(metadata _error)\","),
                }

            },
            Err(_) => {
                return Err(KAssetSourceFolderError::MetadataCreationError)
            },
        }

        Ok(metadata)

    }

}

impl KAssetSource for KAssetSourceFolder {

    fn get_metadata(&self) -> String {
        self.metadata.clone()
    }
    
    fn has_asset(&self, path: PathBuf) -> bool {
        // Get asset full path
        let mut full_path: PathBuf = self.folder_path.clone();
        full_path.push(path);

        // Return if file exists
        full_path.exists()
    }

    fn get_asset(&self, path: PathBuf) ->  Result<Box<dyn Read>, std::io::Error> {
        // Get asset full path
        let mut full_path: PathBuf = self.folder_path.clone();
        full_path.push(path);

        // Return file opened
        match File::open(full_path){
            // File opened correctly, return handle
            Ok(file) => Ok(Box::new(file)),

            // File error, return error
            Err(err) => Err(err),
        }
    }

} 
