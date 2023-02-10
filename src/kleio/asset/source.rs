use std::{io::Read, path::{ PathBuf}};

/// ##### Abstration of a source of assets (audio, models, etc...)
/// 
/// Assets can come from different location like folders, archives, databases, etc...
/// 
/// KAssetSource offer an interface so all data can be retrieved the same way via a unique path.
/// 
/// Always threat KAssetSource as they were a base folder of your assets.
/// 
/// # Note(s)
/// KAssetSource assets are ALWAYS read-only since it only require [Read] trait to be implemented.
/// 
/// # Example(s)
/// ##### Creating a source from a folder (`same as KDataSourceFolder`)
/// ```
/// // Import needed components
/// use core::panic;
/// use std::{path::{PathBuf}, fs::{File}, io::Read};
/// use olympus_kleio::asset::{KAssetSource};
/// 
/// // Create struct that will keep the base folder_path 
/// pub struct SourceFolder {
///    folder_path : PathBuf,
/// }
/// 
/// // Implement a new function for struct
/// impl SourceFolder {
///    pub fn new(folder_path : PathBuf) -> SourceFolder {
///        // Make sure that folder exists
///        if !folder_path.exists() {
///            // Panic if folder doesn't exists
///            panic!("Folder {:?} not found!", folder_path.as_os_str());
///        }
///
///        // Panic if path isn't a folder
///        if !folder_path.is_dir(){
///            panic!("{:?} is not a folder!", folder_path.as_os_str());
///        }
///
///        SourceFolder {
///            folder_path,
///        }
///    }
/// }
/// 
/// // Implement KAssetSource trait for struct
/// impl KAssetSource for SourceFolder {
///    fn has_asset(&self, path: PathBuf) -> bool {
///        // Get asset full path
///        let mut full_path: PathBuf = self.folder_path.clone();
///        full_path.push(path);
///
///        // Return if file exists
///        full_path.exists()
///    }
///
///    fn get_asset(&self, path: PathBuf) ->  Result<Box<dyn Read>, std::io::Error> {
///        // Get asset full path
///        let mut full_path: PathBuf = self.folder_path.clone();
///        full_path.push(path);
///
///        // Return file opened
///        match File::open(full_path){
///            // File opened correctly, return handle
///            Ok(file) => Ok(Box::new(file)),
///
///            // File error, return error
///            Err(err) => Err(err),
///        }
///    }
///
/// } 
/// ```
pub trait KAssetSource {

    /// Get metadata that identify this source.
    /// 
    /// This is a free field to let user create meta tags to identify source in a mod manager, for example.
    /// 
    /// Returns [KAssetSource] metadata as [String]
    fn get_metadata(&self) -> String {
        String::from("(Metadata not implemented...)")
    }

    /// Verify that asset source contains asset from path.
    /// 
    /// Returns `True` if source has the asset or `false` otherwise.
    fn has_asset(&self, path: PathBuf) -> bool;

    /// Get an asset [Read] handle from [path][PathBuf].
    /// 
    /// Returns [Ok][Ok]`(`[Box][Box]`(`[Read]`))` if found or [std::io::Error] otherwise.
    fn get_asset(&self, path: PathBuf) -> Result<Box<dyn Read>, std::io::Error>;
    
}

