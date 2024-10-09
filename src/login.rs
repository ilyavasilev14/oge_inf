use std::{path::Path, fs::{self, File}, str::FromStr};

use directories::ProjectDirs;
use serde::{Deserialize, Serialize};

pub fn init_login(login: &str) -> SaveData {
    match ProjectDirs::from("", "", "oge_training".into()) {
        Some(project_dirs) => {
            let mut config_directory = String::from_str(project_dirs.config_dir().to_str().unwrap()).unwrap();
            config_directory += "/";
            let config_directory = Path::new(&config_directory);

            if !config_directory.exists() {
                if let Err(_) = fs::create_dir_all(config_directory) {
                    panic!("Failed to create a config directory")
                }
            }

            let login_config_path = config_directory.to_str().unwrap().to_owned() + login;
            let login_config_path = Path::new(&login_config_path);
            dbg!(&config_directory);
            dbg!(&login_config_path);
            if !login_config_path.exists() {
                let mut save_data = SaveData::default();
                save_data.login = login.into();
                match File::create(login_config_path) {
                    Ok(_) => {
                        fs::write(login_config_path, toml::to_string(&save_data).unwrap()).unwrap();
                    },
                    Err(_) => panic!("Failed to create a login file"),
                }
                save_data
            } else {
                let save_data: Result<SaveData, toml::de::Error> = toml::from_str(&fs::read_to_string(login_config_path).unwrap());
                match save_data {
                    Ok(save_data) => save_data,
                    Err(err) => {
                        println!("failed to load SaveData from the toml file! Err: {}. Removing it and creating a new, empty one.", err);
                        fs::remove_file(login_config_path).unwrap();
                        let save_data = SaveData {
                            login: login.to_string(),
                            ..Default::default()
                        };
                        match File::create(login_config_path) {
                            Ok(_) => {
                                fs::write(login_config_path, toml::to_string(&save_data).unwrap()).unwrap();
                            },
                            Err(_) => panic!("Failed to create a login file"),
                        }
                        save_data
                    }
                }
            }
        },
        None => panic!("Failed to get project directory"),
    }
}

pub fn save_login_data(data: &SaveData) {
    match ProjectDirs::from("", "", "oge_training".into()) {
        Some(project_dirs) => {
            println!("saving!");

            let mut config_directory = String::from_str(project_dirs.config_dir().to_str().unwrap()).unwrap();
            config_directory += "/";
            let config_directory = Path::new(&config_directory);

            let login_config_path = config_directory.to_str().unwrap().to_owned() + &data.login;
            let login_config_path = Path::new(&login_config_path);

            dbg!(&config_directory);
            dbg!(&login_config_path);

            let save_data_toml = toml::to_string(data).expect("failed to serialize save data to toml str!");
            fs::write(login_config_path, save_data_toml).expect("failed to write a save file");
        },
        None => panic!("Failed to get project directory"),
    }
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SaveData {
    login: String,
    pub total_done_exercise1: u32,
    pub done_correctly_exercise1: u32,
    pub total_done_exercise2: u32,
    pub done_correctly_exercise2: u32,
    pub total_done_exercise3: u32,
    pub done_correctly_exercise3: u32,
    pub total_done_exercise4: u32,
    pub done_correctly_exercise4: u32,
    pub total_done_exercise5: u32,
    pub done_correctly_exercise5: u32,
    pub total_done_exercise6: u32,
    pub done_correctly_exercise6: u32,
    pub total_done_exercise7: u32,
    pub done_correctly_exercise7: u32,
    pub total_done_exercise8: u32,
    pub done_correctly_exercise8: u32,
    pub total_done_exercise9: u32,
    pub done_correctly_exercise9: u32,
    pub total_done_exercise10: u32,
    pub done_correctly_exercise10: u32,
    pub total_done_exercise11: u32,
    pub done_correctly_exercise11: u32,
    pub total_done_exercise12: u32,
    pub done_correctly_exercise12: u32,
    pub total_done_exercise13: u32,
    pub done_correctly_exercise13: u32,
    pub total_done_exercise14: u32,
    pub done_correctly_exercise14: u32,
    pub total_done_exercise15: u32,
    pub done_correctly_exercise15: u32,
}

pub fn increment_total_exercise(excercise_num: u8, save_data: &mut SaveData) {
    match excercise_num {
        1 => save_data.total_done_exercise1 += 1, 
        2 => save_data.total_done_exercise2 += 1,
        3 => save_data.total_done_exercise3 += 1,
        4 => save_data.total_done_exercise4 += 1,
        5 => save_data.total_done_exercise5 += 1,
        6 => save_data.total_done_exercise6 += 1,
        7 => save_data.total_done_exercise7 += 1,
        8 => save_data.total_done_exercise8 += 1,
        9 => save_data.total_done_exercise9 += 1,
        10 => save_data.total_done_exercise10 += 1,
        11 => save_data.total_done_exercise11 += 1,
        12 => save_data.total_done_exercise12 += 1,
        13 => save_data.total_done_exercise13 += 1,
        14 => save_data.total_done_exercise14 += 1,
        15 => save_data.total_done_exercise15 += 1,
        _ => ()
    }
}

pub fn increment_done_correctly_exercise(exercise_num: u8, save_data: &mut SaveData) {
    match exercise_num {
        1 => save_data.done_correctly_exercise1 += 1, 
        2 => save_data.done_correctly_exercise2 += 1,
        3 => save_data.done_correctly_exercise3 += 1,
        4 => save_data.done_correctly_exercise4 += 1,
        5 => save_data.done_correctly_exercise5 += 1,
        6 => save_data.done_correctly_exercise6 += 1,
        7 => save_data.done_correctly_exercise7 += 1,
        8 => save_data.done_correctly_exercise8 += 1,
        9 => save_data.done_correctly_exercise9 += 1,
        10 => save_data.done_correctly_exercise10 += 1,
        11 => save_data.done_correctly_exercise11 += 1,
        12 => save_data.done_correctly_exercise12 += 1,
        13 => save_data.done_correctly_exercise13 += 1,
        14 => save_data.done_correctly_exercise14 += 1,
        15 => save_data.done_correctly_exercise15 += 1,
        _ => ()
    }
}
/*
pub fn get_(exercise_num: u8, save_data: &mut SaveData) {
    match exercise_num {
        1 => save_data.total_done_exercise1 += 1, 
        2 => save_data.total_done_exercise2 += 1,
        3 => save_data.total_done_exercise3 += 1,
        4 => save_data.total_done_exercise4 += 1,
        5 => save_data.total_done_exercise5 += 1,
        6 => save_data.total_done_exercise6 += 1,
        7 => save_data.total_done_exercise7 += 1,
        8 => save_data.total_done_exercise8 += 1,
        9 => save_data.total_done_exercise9 += 1,
        10 => save_data.total_done_exercise10 += 1,
        11 => save_data.total_done_exercise11 += 1,
        12 => save_data.total_done_exercise12 += 1,
        _ => ()
    }
}*/
