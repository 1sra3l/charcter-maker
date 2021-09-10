//! The ini file related code!
extern crate tini;
use tini::Ini;
use std::fs;

// utility functions

/// Get the `item` from the `ini_details` `section` or from `[default]` as a `String`
pub fn get_or_default(item:&str, ini_details:IniDetails)-> String {
    let section:String = ini_details.section.to_owned();
    let ini_string:String = ini_details.ini_string.to_owned();
    let test_ini = Ini::from_string(ini_string.to_owned());
    if test_ini.is_err() {
        println!("data::get_or_default({:?},{:?})-> String\n`Ini::from_string` ERROR:{:?}", item, ini_details.clone(), test_ini.err());
        return String::from("")
    }
    let conf = test_ini.unwrap();
    let return_item:String = match conf.get(section.to_owned().as_str(), item) {
        Some(return_item) => return_item,
        None => match conf.get("default", item) {
            Some(default_item) => default_item,
            None => String::from(""),
        },
    };
    return_item
}

/// Get the `item` from the `ini_details` `section` or from `[default]` as a `String`
pub fn get_vec(item:&str, ini_details:IniDetails)-> Vec<String> {
    let section:String = ini_details.section.to_owned();
    let ini_string:String = ini_details.ini_string.to_owned();
    let test_ini = Ini::from_string(ini_string.to_owned());
    if test_ini.is_err() {
        println!("data::get_or_default({:?},{:?})-> String\n`Ini::from_string` ERROR:{:?}", item, ini_details.clone(), test_ini.err());
        return vec![]
    }
    let conf = test_ini.unwrap();
    let return_item:Vec<String> = match conf.get_vec(section.to_owned().as_str(), item) {
        Some(return_item) => return_item,
        None => match conf.get_vec("default", item) {
            Some(default_item) => default_item,
            None => vec![],
        },
    };
    return_item
}

/// Get the `item` from the `ini_details` `section` or from `[default]` as an `f64`
pub fn get_or_default_f64(item:&str, ini_details:IniDetails, default_return:f64)-> f64 {
    let return_item:String = get_or_default(item, ini_details.clone());
    return_item.parse::<f64>().ok().unwrap_or(default_return)
}

/// Get the `item` from the `ini_details` `section` or from `[default]` OR "0.0" as an `f64`
pub fn get_or_zero_f64(item:&str, ini_details:IniDetails)-> f64 {
    get_or_default_f64(item, ini_details.clone(), 0.0)
}

#[derive(Debug, Clone)]
/// The `ini` file details struct
pub struct IniDetails {
    /// the filename
    pub filename:String,
    /// the string contents of the file
    pub ini_string:String,
    /// The header which has the information we need
    pub section:String,
}
impl IniDetails {
    /// Make an empty `IniDetails` struct
    pub fn empty() -> Self where Self:Sized {
        IniDetails {
            filename:"".to_string(),
            ini_string:"".to_string(),
            section:"".to_string(),
        }
    }

    /// Wrapper to get section name
    pub fn _name(&self) -> String {
        self.section.to_string()
    }

    /// Read a file name in with a section to canonicalize the file name path and load the details
    pub fn load(file_name:String, section:&str) -> Self where Self:Sized {
        let filename = match fs::canonicalize(file_name.to_owned()) {
            Ok(filename) => filename.to_str().unwrap().to_owned(),
            Err(e) => {
                println!("data::IniDetails::load `canonicalize` ERROR: {:?}\nLikely due to the file: {:?}\nTry using an absolute path, if encountering errors", e, file_name);
                return Self::empty()
            },
        };
        let test_ini = Ini::from_file(&filename);
        if test_ini.is_err() {
            println!("data::IniDetails::load `Ini::from_file` ERROR: {:?} in {:?}", test_ini, filename.to_owned());
            return Self::empty()
        }
        let conf = test_ini.unwrap();
        let ini_string = conf.to_string().to_owned();
        IniDetails {
            filename:filename.to_owned(),
            ini_string:ini_string.to_owned(),
            section:section.to_string(),
        }
    }

    /// Create an `IniDetails` struct from info
    pub fn _new(filename:String, ini_string:String, section:&str) -> Self where Self:Sized {
        IniDetails {
            filename:filename.to_owned(),
            ini_string:ini_string.to_owned(),
            section:section.to_string(),
        }
    }
}

impl Default for IniDetails {
    /// By default create an empty struct
    fn default() -> Self {
        Self::empty()
    }
}
