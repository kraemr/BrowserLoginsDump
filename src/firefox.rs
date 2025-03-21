use std::fs;
use std::path::PathBuf;
use dirs_next::home_dir; // Correct import for getting the home directory

use rusqlite::{params, Connection, Result};

use serde::Deserialize;
use std::fs::File;
use std::io::Read;

#[derive(Debug, Deserialize)]
pub struct Login {
    pub hostname: String,
    pub username: String,
    pub password: String,  // This is the encrypted password
}

#[derive(Debug, Deserialize)]
pub struct EncryptedLogin {
    pub hostname: String,
    pub encryptedUsername: String,
    pub encryptedPassword: String,  // This is the encrypted password
}

#[derive(Debug, Deserialize)]
pub struct EncryptedLoginsJson {
    pub logins: Vec<EncryptedLogin>,
}

#[derive(Debug, Deserialize)]
pub struct LoginsJson {
    pub logins: Vec<Login>,
}


fn append_to_path(path: &Option<PathBuf>, str: &str) -> Option<PathBuf> {
    Some(path.as_ref()?.join(str))
}


pub fn GetFirefoxPasswordDbPaths() -> Option<(PathBuf, PathBuf)> {
    let home_dir = home_dir()?; 
    let firefox_profiles_path = home_dir.join(".mozilla/firefox");

    // Ensure the Firefox profiles directory exists
    if !firefox_profiles_path.exists() {
        println!("firefox profiles doesnt exist");
        return None;
    }
    let profiles_ini_path = firefox_profiles_path.join("profiles.ini");

    // Read profiles.ini to find the default profile
    let profiles_ini_content = fs::read_to_string(&profiles_ini_path).ok()?;
    let mut profile_paths: Vec<Option<PathBuf>> = vec![];

    for line in profiles_ini_content.lines() {
        if line.starts_with("Path=") {
            let profile_name = line.split('=').nth(1)?;
            let profile_path = Some(firefox_profiles_path.join(profile_name));
            profile_paths.push(profile_path);
            //break;
        }
    }
    

    // TODO: Handle returning multiple of these
    for path in profile_paths {
        let logins_json = append_to_path(&path,"logins.json");
        let key4_db = append_to_path(&path,"key4.db");
        if let (Some(logins), Some(key4)) = (logins_json.as_ref(), key4_db.as_ref()) {
            if logins.exists() && key4.exists() {
                return Some((logins_json?, key4_db?));
            }
        }

    }
    None
}

pub fn read_logins_json(file_path: &str) -> Result<EncryptedLoginsJson, Box<dyn std::error::Error>> {
    // Open the file
    let mut file = File::open(file_path)?;

    // Read the file contents into a string
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    // Deserialize the JSON into the LoginsJson struct
    let logins_json: EncryptedLoginsJson = serde_json::from_str(&contents)?;
    Ok(logins_json)
}

pub fn extract_decryption_key(key4_db_path: &str) -> Result<Vec<u8>> {
    let conn = Connection::open(key4_db_path)?;
    let mut stmt = conn.prepare("SELECT item1 FROM metadata ")?;
    let mut rows = stmt.query(params![])?;

    if let Some(row) = rows.next()? {
        let key: Vec<u8> = row.get(0)?;
        Ok(key)
    } else {
        Err(rusqlite::Error::QueryReturnedNoRows)
    }
}