use std::{fs::{self, File}, io::Write, path::PathBuf, vec};

use olympus::kleio::asset::{KAssetBroker, KAssetSourceFolder, KAssetSource, KAssetBrokerError};

// Test folder where to create assets
static TEST_FOLDER: &str = "target/tests/kleio/asset/";

// Stress test loop count
static STRESS_TEST_COUNT : usize = 1000000;


/*********
* MACROS * 
*********/
/// Macro used to prepare test and execute it.
macro_rules! kasset_broker_test_prepare {

    // This macro call doesn't create KAssetSourceFolder nor the files
    ($kab_var:ident, $folder_var:ident, $folder_name:expr, $test_body:block) => {{
        // Create Broker
        #[allow(unused_mut)]
        let mut $kab_var = KAssetBroker::new();

        // Assign folder name
        let  $folder_var: &str = &(TEST_FOLDER.to_owned() + $folder_name);

        // Test body
        $test_body

        // Clean test files
        fs::remove_dir_all(PathBuf::from($folder_var)).expect("Test couldn't be cleaned!");
    }};

    // This macro call will create files and KAssetSourceFolder
    ($kab_var:ident, $folder_var:ident, $folder_name:expr, ($kaf_var:ident $(,$extra:ident)*), $test_body:block) => {{
        // Create Broker
        let mut $kab_var = KAssetBroker::new();

        // Assign folder name
        let  $folder_var: &str = &(TEST_FOLDER.to_owned() + $folder_name);

        // Create test file and add source
        kasset_broker_test_asset!($kab_var, $folder_var, 0, ($kaf_var $(,$extra)*));

        // Test body
        $test_body

        // Clean test files
        fs::remove_dir_all(PathBuf::from($folder_var)).expect("Test couldn't be cleaned!");
    }}
}

/// This macro create test files and KAssetSourceFolder
macro_rules! kasset_broker_test_asset {

    // All Token expended, do nothing
    ($kab_var:ident, $folder_var:ident, $kaf_counter:expr) => {{ }};

    // Use token to create files and KAssetSourceFolder
    ($kab_var:ident, $folder_var:ident, $kaf_counter:expr, $kaf_var:ident) => {

        // Create folder and file
        create_test_folder_files($folder_var, $kaf_counter);

        // Create KAssetSourceFolder
        let $kaf_var = KAssetSourceFolder::new(PathBuf::from($folder_var.to_owned() + "subfolder" + $kaf_counter.to_string().as_str() + "/")).unwrap();

        // Add KAssetSourceFolder to Broker
        match_source_to_broker($kab_var.add_source(&$kaf_var), false);
    };

    // Initial call
    ($kab_var:ident, $folder_var:ident, $kaf_counter:expr, ($kaf_var:ident $(,$extra:ident)*)) => {

        let cpt = 0;
        kasset_broker_test_asset!($kab_var, $folder_var, $kaf_counter + cpt, $kaf_var);
        $(
            let cpt = cpt + 1;
            kasset_broker_test_asset!($kab_var, $folder_var, $kaf_counter + cpt, $extra);
        )*
    };

}


/********
* TESTS *
********/
#[test]
/// Create a new instance of KAssetBroker
/// 
/// # Verification(s)
/// V1 |  KAssetBroker::new() create an instance without error.
fn kasset_broker_create() {
    // V1 |  KAssetBroker::new() create an instance without error.
    KAssetBroker::new();
}

#[test]
/// Create a new KAssetBroker and add a source to it.
/// 
/// # Verification(s)
/// V1 | KAssetBroker::add_source() without error.
fn kasset_broker_add_source() {
    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_add_1_source/",
        {
            // Create test folder and files
            create_test_folder_files(folder_name, 0);

            // Create KAssetSourceFolder
            let kaf = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/")).unwrap();

            // V1 | KAssetBroker::add_source() without error.
            // Add KAssetSourceFolder to broker
            match_source_to_broker(kab.add_source(&kaf), false);
        }
    );
}

#[test]
/// Create a new KAssetBroker and try adding same source twice.
/// 
/// # Verification(s)
/// V1 | KAssetBroker::add_source() is expected to fail after adding the same source twice.
fn kasset_broker_add_source_again() {
    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_add_same_source/",
        {
            // Create test folder and files
            create_test_folder_files(folder_name, 0);

            // Create KAssetSourceFolder
            let kaf = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/")).unwrap();

            // Add KAssetSourceFolder to broker
            match_source_to_broker(kab.add_source(&kaf), false);

            // V1 | KAssetBroker::add_source() is expected to fail after adding the same source twice.
            match_source_to_broker(kab.add_source(&kaf), true);
        }
    );
}

#[test]
/// Create a new KAssetBroker and add 10 source to it.
/// 
/// # Verification(s)
/// V1 | KAssetBroker can support multiple different sources (10 tested).
fn kasset_broker_add_source_10() {
    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_add_10_source/", (kaf0, kaf1, kaf2, kaf3, kaf4, kaf5, kaf6, kaf7, kaf8, kaf9),
        {
            // V1 | KAssetBroker can support multiple different sources (10 tested).
            match_source_to_broker(kab.add_source(&kaf0), true);
            match_source_to_broker(kab.add_source(&kaf1), true);
            match_source_to_broker(kab.add_source(&kaf2), true);
            match_source_to_broker(kab.add_source(&kaf3), true);
            match_source_to_broker(kab.add_source(&kaf4), true);
            match_source_to_broker(kab.add_source(&kaf5), true);
            match_source_to_broker(kab.add_source(&kaf6), true);
            match_source_to_broker(kab.add_source(&kaf7), true);
            match_source_to_broker(kab.add_source(&kaf8), true);
            match_source_to_broker(kab.add_source(&kaf9), true);
        }
    );
}

#[test]
/// Verify that a broker has a source (or not)
/// 
/// # Verification(s)
/// V1 | Add 2 sources out of 3
/// V2 | KAssetBroker::has_source() has 2 sources out of 3.
fn kasset_broker_has_source(){

    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_has_source/",
        {
            // Create test files
            create_test_files(folder_name, 3, 10);

            // Create 3 KAssetSources
            let kaf0 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/")).unwrap();
            let kaf1 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder1/")).unwrap();
            let kaf2 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder2/")).unwrap();

            // V1 | Add 2 sources out of 3
            match_source_to_broker(kab.add_source(&kaf0), false);
            match_source_to_broker(kab.add_source(&kaf2), false);

            // V2 | KAssetBroker::has_source() has 2 sources out of 3.
            assert!(kab.has_source(&kaf0), "KAssetBroker should contain source #0");
            assert!(!kab.has_source(&kaf1), "KAssetBroker shouldn't contain source #1");
            assert!(kab.has_source(&kaf2), "KAssetBroker should contain source #2");
        }
    );
}

#[test]
/// Remove a source from broker
/// 
/// # Verification(s)
/// V1 | KAssetBroker::remove_source() remove a source without error.
/// V2 | Broker sources length should be 3 after adding 3 sources.
/// V3 | Each source remove should lower length by 1.
fn kasset_broker_remove_source() {

    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_remove_source/", (kaf0, kaf1, kaf2),
        {
            // V2 | Broker sources length should be 3 after adding 3 sources.
            assert!(kab.get_sources().len() == 3, "Broker should contains 3 sources!");
            print_broker_sources_metadatas(&kab);

            // V1 | KAssetBroker::remove_source() remove a source without error.
            match kab.remove_source(&kaf0){
                Ok(_) => {},
                Err(_) => assert!(false, "Error while removing source #0!"),
            }

            // V3 | Each source remove should lower length by 1.
            assert!(kab.get_sources().len() == 2, "Broker should contains 2 sources!");
            print_broker_sources_metadatas(&kab);

            // V1 | KAssetBroker::remove_source() remove a source without error.
            match kab.remove_source(&kaf1){
                Ok(_) => {},
                Err(_) => assert!(false, "Error while removing source #1!"),
            }

            // V3 | Each source remove should lower length by 1.
            assert!(kab.get_sources().len() == 1, "Broker should contains 1 sources!");
            print_broker_sources_metadatas(&kab);

            // V1 | KAssetBroker::remove_source() remove a source without error.
            match kab.remove_source(&kaf2){
                Ok(_) => {},
                Err(_) => assert!(false, "Error while removing source #2!"),
            }

            // V3 | Each source remove should lower length by 1.
            assert!(kab.get_sources().len() == 0, "Broker should contains 0 sources!");
            print_broker_sources_metadatas(&kab);
        }
    );
}

#[test]
/// Verify that broker return an error when trying to remove an inexistant source.
/// 
/// # Verification(s)
/// V1 | KAssetBroker::remove_source() must return Err() since source not in broker.
fn kasset_broker_remove_source_inexistant() {

    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_remove_source_inexistant/",
        {
            // Create test files
            create_test_files(folder_name, 10, 10);
        
            // Create KAssetSources
            let kaf = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/")).unwrap();

            // V1 | KAssetBroker::remove_source() must return Err() since source not in broker.
            match kab.remove_source(&kaf){
                Ok(_) => assert!(false, "Error! Source shouldn't be in broker!"),
                Err(_) => {},
            }
        }
    );     
}

#[test]
/// Rearrange sources priotiries.
/// 
/// # Verification(s)
/// V1 | KAssetBroker::set_source_priority() modify priority without error.
/// V2 | Verify priorities orders with expected order.
/// V3 | Repeated multiple time and verified to ensure stability.
fn kasset_broker_set_source_priority() {

    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_set_source_priority/", (kaf0, kaf1, kaf2, kaf3, kaf4, kaf5, kaf6, kaf7, kaf8, kaf9),
        {
            // V1 | KAssetBroker::set_source_priority() modify priority without error.
            match kab.set_source_priority(&kaf2, 8){
                Ok(_) => {},
                Err(_) => assert!(false, "Error happens when setting source priority!"),
            }

            // V2 | Verify priorities orders with expected order.
            assert!(verify_priorities_order(&kab, vec![0,1,3,4,5,6,7,8,2,9]), "Broker priorities order error!");

            // V3 | Repeated multiple time and verified to ensure stability.
            match kab.set_source_priority(&kaf0, 9){
                Ok(_) => {},
                Err(_) => assert!(false, "Error happens when setting source priority!"),
            }
            match kab.set_source_priority(&kaf3, 9){
                Ok(_) => {},
                Err(_) => assert!(false, "Error happens when setting source priority!"),
            }
            match kab.set_source_priority(&kaf4, 9){
                Ok(_) => {},
                Err(_) => assert!(false, "Error happens when setting source priority!"),
            }
            match kab.set_source_priority(&kaf5, 9){
                Ok(_) => {},
                Err(_) => assert!(false, "Error happens when setting source priority!"),
            }
            match kab.set_source_priority(&kaf6, 9){
                Ok(_) => {},
                Err(_) => assert!(false, "Error happens when setting source priority!"),
            }
            match kab.set_source_priority(&kaf7, 9){
                Ok(_) => {},
                Err(_) => assert!(false, "Error happens when setting source priority!"),
            }
            match kab.set_source_priority(&kaf8, 9){
                Ok(_) => {},
                Err(_) => assert!(false, "Error happens when setting source priority!"),
            }

            // V2 | Verify priorities orders with expected order.
            assert!(verify_priorities_order(&kab, vec![1,2,9,0,3,4,5,6,7,8]), "Broker priorities order error!");
        }
    );
}

#[test]
/// Verify that modifying an inexistant source priority result in an error.
/// 
/// # Verification(s)
/// V1 | KAssetBroker::set_source_priority() must return Err() for inexistant source.
fn kasset_broker_set_source_priority_inexistant() {

    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_set_source_priority_inexistant/",
        {
            // Create test files
            create_test_files(folder_name, 10, 10);
        
            // Create KassetSource
            let kaf0 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/")).unwrap();
            let kaf1 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder1/")).unwrap();

            // Add sources to broker
            match_source_to_broker(kab.add_source(&kaf0), false);

            // V1 | KAssetBroker::set_source_priority() must return Err() for inexistant source.
            match kab.set_source_priority(&kaf1, 0){
                Ok(_) => assert!(false, "Error! Setting source priority to inexistant should have failed!"),
                Err(_) => {},
            }
        }
    );  
}

#[test]
/// Verify that setting a priority Out of bounds (priority > sources length) result in error. 
/// 
/// # Verification(s)
/// V1 | KAssetBroker::set_source_priority() must return Err() for out of bounds priority.
fn kasset_broker_set_source_priority_oob() {
    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_set_source_priority_oob/",
        {
            // Create test files
            create_test_files(folder_name, 10, 10);

            // Create KassetSource
            let kaf0 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/")).unwrap();
            let kaf1 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder1/")).unwrap();

            // Add sources to broker
            match_source_to_broker(kab.add_source(&kaf0), false);
            match_source_to_broker(kab.add_source(&kaf1), false);

            // V1 | KAssetBroker::set_source_priority() must return Err() for out of bounds priority.
            match kab.set_source_priority(&kaf1, 10){
                Ok(_) => assert!(false, "Error! Setting source priority higher than bound should have failed!"),
                Err(_) => {},
            }
        }
    );  
}

#[test]
/// Verify that getting an asset from an empty broker result in error.
/// 
/// # Verification(s)
/// V1 | KAssetBroker::get_asset() must return an error since it has no sources.
fn kasset_broker_get_asset_no_source() {
    
    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_get_asset_no_source/",
        {
            // Create test files
            create_test_files(folder_name, 1, 1);

            // V1 | KAssetBroker::get_asset() must return an error since it has no sources.
            match kab.get_asset(PathBuf::from( "path.txt")){
                Ok(_) => assert!(false, "Error! Got asset without any sources!"),
                Err(_) => {},
            }
        }
    );  

}

#[test]
/// Get asset from broker with 1 source.
/// 
/// # Verification(s)
/// V1 | KAssetBroker::get_asset() must return a readable asset.
/// V2 | Asset content must match expected content.
fn kasset_broker_get_asset_1_source() {
    
    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_get_asset_1_source/", (kaf0),
        {
            // V1 | KAssetBroker::get_asset() must return a readable asset.
            // V2 | Asset content must match expected content.
            fetch_asset_and_verify(&kab, "file0.txt", &String::from("Hello0, world0!"));            
        }
    );

}

#[test]
/// Get asset from broker with 10 source according to priorities.
/// 
/// # Verification(s)
/// V1 | KAssetBroker::get_asset() gives the correct asset according to priority.
fn kasset_broker_get_asset_10_source() {
    
    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_get_asset_10_source/", (kaf0, kaf1, kaf2, kaf3, kaf4, kaf5, kaf6, kaf7, kaf8, kaf9),
        {
            let mut index = 0;
            loop {
                let filename = "file".to_owned() + index.to_string().as_str() + ".txt";
                let filecontent = &String::from("Hello".to_owned() + index.to_string().as_str() +", world"+ index.to_string().as_str() + "!");

                // V1 | KAssetBroker::get_asset() gives the correct asset according to priority.
                fetch_asset_and_verify(&kab, &filename, &filecontent);

                // Remove a src (so we get next source priority)
                match kab.remove_source(kab.get_sources()[0]){
                    Ok(_) => {},
                    Err(_) => assert!(false, "Error! Couldn't remove source {}!", index),
                }

                // Increment index
                index = index + 1;
                if index >= 10 {
                    break;
                }
            }            
        }
    );

}

#[test]
/// Verify that trying to get an inexistant asset from multiple sources result in error.
/// 
/// # Verification(s)
/// V1 | KAssetBroker::get_asset() must return Err() since no source contains "Shouldnotexists.txt"
fn kasset_broker_get_asset_inexistant() {

    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_get_asset_inexistant/", (kaf0, kaf1, kaf2, kaf3, kaf4, kaf5, kaf6, kaf7, kaf8, kaf9),
    {

        // V1 | KAssetBroker::get_asset() must return Err() since no source contains "Shouldnotexists.txt"
        match kab.get_asset(PathBuf::from("Shouldnotexists.txt")){
            Ok(_) => assert!(false, "Error! Asset shouldn't be found!"),
            Err(_) => {},
        }    
    }
);

}

#[test]
#[ignore]
/// Stress test KAssetBroker to test stability and limit. Ignored by default. 
/// 
/// # Verification(s)
/// V1 | Add multiple sources.
/// V2 | Remove multiple sources.
/// V3 | Fetch multiple assets.
/// V4 | Set multiple priorities.
/// V5 | Repeat STRESS_TEST_COUNT times.
fn kasset_broker_stress() {
    
    kasset_broker_test_prepare!(kab, folder_name, "kasset_broker_get_asset_10_source/", (kaf0, kaf1, kaf2, kaf3, kaf4, kaf5, kaf6, kaf7, kaf8, kaf9),
        {
            // Create KassetSource
            let kafp0 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder0/")).unwrap();
            let kafp1 = KAssetSourceFolder::new(PathBuf::from(folder_name.to_owned() + "subfolder1/")).unwrap();

            // V5 | Repeat STRESS_TEST_COUNT times.
            for i in 0..STRESS_TEST_COUNT {

                // Make sure choosen file is between [0,9]
                let f_index = i % 10;

                // V4 | Set multiple priorities.
                let kaf = kab.get_sources()[f_index];
                match kab.set_source_priority(kaf, 0){
                    Ok(_) => {},
                    Err(_) => assert!(false, "Couldn't change source priority!"),
                }

                // V1 | Add multiple sources.
                match_source_to_broker(kab.add_source(&kafp0), false);
                match_source_to_broker(kab.add_source(&kafp1), false);

                // Set filename to use
                let filename = "file".to_owned() + f_index.to_string().as_str() + ".txt";

                // V3 | Fetch multiple assets.
                let kaf_index = extract_source_folder_index(kaf);
                let filecontent = &String::from("Hello".to_owned() + kaf_index.to_string().as_str() +", world"+ f_index.to_string().as_str() + "!");

                // Compare filecontent
                fetch_asset_and_verify(&kab, &filename, &filecontent);

                // V2 | Remove multiple sources.
                match kab.remove_source(&kafp0){
                    Ok(_) => {},
                    Err(_) => assert!(false, "Couldn't remove source kafp0!"),
                }

                match kab.remove_source(&kafp1){
                    Ok(_) => {},
                    Err(_) => assert!(false, "Couldn't remove source kafp1!"),
                }

                
            }       
        }
    );

}


/*************
 * FUNCTIONS *
 ************/
/// Create a folder with parents folders from folder_path.
/// 
/// # Panic
/// Will panic if folders not created
fn create_folder(folder_path : &str) {

    match fs::create_dir_all(folder_path){
        Ok(_) => {},
        Err(_) => assert!(false, "Error when creating folder {}!", folder_path),
    }

}

/// Create a file from file_path and it's content from file_content.
/// 
/// # Panic
/// Will panic if file not created
fn create_file_with_content(file_path : &str, file_content : &str){

    let file = File::create(&file_path);
    
    match file {
        Ok(mut file) => { 
            match file.write_all(file_content.as_bytes()) {
                Ok(_) => {},
                Err(_) => assert!(false, "Error when writing file {}!", file_path),
            }
        },
        Err(_) => assert!(false, "Error when creating file {}!", file_path),
    }
}


/// Create n test files and n sub-folders in a given folder name.
/// 
/// # Panic
/// Will panic if folder and/or file not created
fn create_test_files(folder_name : &str, sf_count: usize, file_count:usize){

    // Create folder and sub folders for test
    for i in 0..sf_count {
        create_folder(&(folder_name.to_owned() + "subfolder" + i.to_string().as_str()));
    }
    
    // Create file with content in multiple subfolders
    for i in 0..sf_count {
        for j in 0..file_count {
            let file_name = &(folder_name.to_owned() + "subfolder" + i.to_string().as_str() + "/file" + j.to_string().as_str() + ".txt");
            create_file_with_content(&file_name, &("Hello".to_owned() + i.to_string().as_str() + ", world" + j.to_string().as_str() + "!"));
        }
    }
}

/// Create test folder with 10 files.
/// 
/// # Panic
/// Will panic if folder and/or file not created
fn create_test_folder_files(folder_name : &str, subf_id: usize){

    // Create folder and sub folders for test
    create_folder(&(folder_name.to_owned() + "subfolder" + subf_id.to_string().as_str()));
    
    // Create file with content in multiple subfolders
    for j in 0..10 {
        let file_name = &(folder_name.to_owned() + "subfolder" + subf_id.to_string().as_str() + "/file" + j.to_string().as_str() + ".txt");
        create_file_with_content(&file_name, &("Hello".to_owned() + subf_id.to_string().as_str() + ", world" + j.to_string().as_str() + "!"));
    }
}

/// Match result and assert error according to expectation when adding a source.
/// 
/// # Panic
/// Will panic if !expect_fail.
fn match_source_to_broker<'a>(res : Result<usize, KAssetBrokerError>, expect_fail : bool){

    match res{
        Ok(_) => assert!(!expect_fail, "Adding the same source should fail!"),
        Err(_) => assert!(expect_fail, "Add 1 source failed!"),
    }


}

/// Print KAssetBroker sources metadata.
fn print_broker_sources_metadatas(kab : &KAssetBroker){

    println!("\n*** START BROKER SOURCES ***");


    for src in kab.get_sources().iter() {

        println!("{}", src.get_metadata());
    } 
    
    println!("*** END BROKER SOURCES ***\n");
}

/// Verify KAssetBroker sources priorities.
/// 
/// Returns True if priorities matches. False otherwise. 
fn verify_priorities_order(kab : &KAssetBroker, priority : Vec<u32>) -> bool{

    let mut is_correct : bool = true;

    for i in 0..priority.len() {

        let sf = extract_source_folder_index(kab.get_sources()[i]);

        // Compare with order vector
        if sf != priority[i] {
            println!("Error : Src #{} priority should be {} instead of !", sf, i);
            is_correct = false;
            break;
        }


    }

    is_correct
}

/// Returns asset source folder index from metadata as u32.
fn extract_source_folder_index(src:&dyn KAssetSource) -> u32 {

    // Extract metadata
    let metadata = src.get_metadata();
    let mut split = metadata.split("\",\"");
    let path  = split.next().unwrap();
    let index = &path[path.len() - 2..path.len() - 1];

    // Get source index
    index.chars().next().unwrap().to_digit(10).unwrap()
}

/// Fetch asset by name from broker and verify content.
/// 
/// # Panic
/// * Will panic if asset can't be found.
/// * Will panic if asset can't be read.
/// * Will panic if asset content is different.
fn fetch_asset_and_verify(kab : &KAssetBroker, asset_name:&str, asset_content:&String) {

    match kab.get_asset(PathBuf::from(asset_name)){
        Ok(mut asset) => {
            let mut str:String = String::new();
            match asset.read_to_string(&mut str){
                Ok(_) => {
                    assert!(str.eq(asset_content), "Error! Asset Content \"{:?}\" != \"{:?}\"", str, asset_content);
                },
                Err(_) => assert!(false, "Error! Cannot read {:?}!", asset_name),
            }
        },
        Err(_) => assert!(false, "Error! File {:?} not found!", asset_name),
    }

}