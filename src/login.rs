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
                let save_data: SaveData = toml::from_str(&fs::read_to_string(login_config_path).unwrap()).unwrap();
                save_data
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
    pub total_done_excersise1: u32,
    pub done_correctly_excersise1: u32,
    pub total_done_excersise2: u32,
    pub done_correctly_excersise2: u32,
    pub total_done_excersise3: u32,
    pub done_correctly_excersise3: u32,
    pub total_done_excersise4: u32,
    pub done_correctly_excersise4: u32,
    pub total_done_excersise5: u32,
    pub done_correctly_excersise5: u32,
    pub total_done_excersise6: u32,
    pub done_correctly_excersise6: u32,
    pub total_done_excersise7: u32,
    pub done_correctly_excersise7: u32,
    pub total_done_excersise8: u32,
    pub done_correctly_excersise8: u32,
    pub total_done_excersise9: u32,
    pub done_correctly_excersise9: u32,
    pub total_done_excersise10: u32,
    pub done_correctly_excersise10: u32,
    pub total_done_excersise11: u32,
    pub done_correctly_excersise11: u32,
    pub total_done_excersise12: u32,
    pub done_correctly_excersise12: u32,
    pub total_done_excersise13: u32,
    pub done_correctly_excersise13: u32,
    pub total_done_excersise14: u32,
    pub done_correctly_excersise14: u32,
    pub total_done_excersise15: u32,
    pub done_correctly_excersise15: u32,
}

pub fn increment_total_excersise(excersise_num: u8, save_data: &mut SaveData) {
    match excersise_num {
        1 => save_data.total_done_excersise1 += 1, 
        2 => save_data.total_done_excersise2 += 1,
        3 => save_data.total_done_excersise3 += 1,
        4 => save_data.total_done_excersise4 += 1,
        5 => save_data.total_done_excersise5 += 1,
        6 => save_data.total_done_excersise6 += 1,
        7 => save_data.total_done_excersise7 += 1,
        8 => save_data.total_done_excersise8 += 1,
        9 => save_data.total_done_excersise9 += 1,
        10 => save_data.total_done_excersise10 += 1,
        11 => save_data.total_done_excersise11 += 1,
        12 => save_data.total_done_excersise12 += 1,
        13 => save_data.total_done_excersise13 += 1,
        14 => save_data.total_done_excersise14 += 1,
        15 => save_data.total_done_excersise15 += 1,
        _ => ()
    }
}

pub fn increment_done_correctly_excersise(excersise_num: u8, save_data: &mut SaveData) {
    match excersise_num {
        1 => save_data.done_correctly_excersise1 += 1, 
        2 => save_data.done_correctly_excersise2 += 1,
        3 => save_data.done_correctly_excersise3 += 1,
        4 => save_data.done_correctly_excersise4 += 1,
        5 => save_data.done_correctly_excersise5 += 1,
        6 => save_data.done_correctly_excersise6 += 1,
        7 => save_data.done_correctly_excersise7 += 1,
        8 => save_data.done_correctly_excersise8 += 1,
        9 => save_data.done_correctly_excersise9 += 1,
        10 => save_data.done_correctly_excersise10 += 1,
        11 => save_data.done_correctly_excersise11 += 1,
        12 => save_data.done_correctly_excersise12 += 1,
        13 => save_data.done_correctly_excersise13 += 1,
        14 => save_data.done_correctly_excersise14 += 1,
        15 => save_data.done_correctly_excersise15 += 1,
        _ => ()
    }
}
/*
pub fn get_(excersise_num: u8, save_data: &mut SaveData) {
    match excersise_num {
        1 => save_data.total_done_excersise1 += 1, 
        2 => save_data.total_done_excersise2 += 1,
        3 => save_data.total_done_excersise3 += 1,
        4 => save_data.total_done_excersise4 += 1,
        5 => save_data.total_done_excersise5 += 1,
        6 => save_data.total_done_excersise6 += 1,
        7 => save_data.total_done_excersise7 += 1,
        8 => save_data.total_done_excersise8 += 1,
        9 => save_data.total_done_excersise9 += 1,
        10 => save_data.total_done_excersise10 += 1,
        11 => save_data.total_done_excersise11 += 1,
        12 => save_data.total_done_excersise12 += 1,
        _ => ()
    }
}*/
