use std::fs;
use directories::UserDirs;
use iced::widget::{button, column, container, scrollable, text, Image};
use rand::{Rng, distributions::{Alphanumeric, DistString}};
use crate::{Message, ExerciseData, ExcerciseState};
use super::Exercise;
use std::process::Command;
use std::path::PathBuf;


pub struct Excersise12 { }

impl Exercise for Excersise12 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let text: iced::Element<'a, Message> = text(
"   Чтобы решить задание 12, необходимо открыть указанный в условии каталог в файловом менеджере(обычно иконкой таких программ является изображение папки).
    После открытия каталога, нужно воспользоваться поиском. В запрос нужно ввести расширение файла, которое указано в условии. Кроме того, в задании могут быть указаны какие-либо дополнительные условия, например, информационный объём файлов.")
            .size(Self::text_size())
            .into();

        let scroll = scrollable(text);
        let column = 
            column![
                button(Image::new("back_arrow.png").width(100).height(100)).on_press(Message::OpenExcersiseList),
                scroll
            ]
            .spacing(15);
        let cont = container(column).into();

        cont
    }

    fn generate_random_excersise() -> ExerciseData {
        //let ex_type = rand::thread_rng().gen_range(0..4);
        let ex_type = rand::thread_rng().gen_range(1..=2);
        match ex_type {
            1 => generate_excersise_type1(),
            _ => generate_excersise_type2(), 
        }
    }

    fn select_subexcersise() -> Message {
        println!("select_subexcersise");
        Message::SelectedSubExcersise(12, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(12)
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(12)
    }

    fn excersise_number() -> u8 {
        12
    }
}



fn generate_excersise_type1() -> ExerciseData {
    let required_file_extention = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(2..=5));

    let required_dir_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
    let required_dir_file_count = rand::thread_rng().gen_range(6..=25);
    let required_dir_required_file_count = rand::thread_rng().gen_range(6..=25);

    let other_dir_count = rand::thread_rng().gen_range(1..4);

    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir();
        let home_dir = home_dir.join("ОГЭ/");
        let _ = fs::remove_dir_all(&home_dir);
        let _ = fs::create_dir(&home_dir);
        let required_dir_path = home_dir.join(&required_dir_name);
        fs::create_dir(&required_dir_path).expect("failed to create a required directory");
        for _ in 0..other_dir_count {
            let name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
            let file_count = rand::thread_rng().gen_range(0..=30);
            let dir_path = home_dir.join(name);
            fs::create_dir(&dir_path).expect("failed to create other directory");

            for _ in 0..file_count {
                let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
                create_file(dir_path.join(file_name));
            }
        }

        for _ in 0..required_dir_file_count {
            let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
            let file_ext = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(2..=4));
            let file_name = file_name + "." + &file_ext;
            create_file(required_dir_path.join(file_name));
        }

        for _ in 0..required_dir_file_count {
            let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
            let file_ext = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(2..=4));
            let file_name = file_name + "." + &file_ext;

            create_file(required_dir_path.join(file_name));
        }

        for _ in 0..required_dir_required_file_count {
            let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
            let file_name = file_name + "." + &required_file_extention;

            dbg!(&required_dir_path.join(&file_name));
            create_file(required_dir_path.join(file_name));
        }
    } else {
        panic!("failed to get user dirs");
    }
    let title = format!(
        "Сколько файлов с расширением {} содержится в каталоге {}? В ответе укажите только число.

        {} является подкаталогом \"ОГЭ\", который находится в домашнем каталоге.", 
        required_file_extention, required_dir_name, required_dir_name);

    ExerciseData {
        title,
        right_answer: required_dir_required_file_count.to_string(),
        input_field_text: "".into(),
        state: ExcerciseState::NotDone,
        additional_data: Vec::new(),
    }
}

fn generate_excersise_type2() -> ExerciseData {
    let required_file_extention = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(2..=5));

    let required_dir_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
    let required_dir_file_count = rand::thread_rng().gen_range(6..=25);
    let required_dir_required_file_count = rand::thread_rng().gen_range(6..=25);
    let required_dir_required_file_size_count = rand::thread_rng().gen_range(6..=25);
    let required_file_size_bytes = rand::thread_rng().gen_range(512..=2048);

    let other_dir_count = rand::thread_rng().gen_range(1..4);

    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir();
        let home_dir = home_dir.join("ОГЭ/");
        let _ = fs::remove_dir_all(&home_dir);
        let _ = fs::create_dir(&home_dir);
        let required_dir_path = home_dir.join(&required_dir_name);
        fs::create_dir(&required_dir_path).expect("failed to create a required directory");
        for _ in 0..other_dir_count {
            let name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
            let file_count = rand::thread_rng().gen_range(0..=30);
            let dir_path = home_dir.join(name);
            fs::create_dir(&dir_path).expect("failed to create other directory");

            for _ in 0..file_count {
                let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
                create_file(dir_path.join(file_name));
            }
        }

        for _ in 0..required_dir_file_count {
            let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
            let file_ext = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(2..=4));
            let file_name = file_name + "." + &file_ext;
            create_file(required_dir_path.join(file_name));
        }

        for _ in 0..required_dir_file_count {
            let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
            let file_ext = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(2..=4));
            let file_name = file_name + "." + &file_ext;

            create_file(required_dir_path.join(file_name));
        }

        for _ in 0..required_dir_required_file_count {
            let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
            let file_name = file_name + "." + &required_file_extention;

            create_file(required_dir_path.join(file_name));
        }

        for _ in 0..required_dir_required_file_size_count {
            let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
            let file_name = file_name + "." + &required_file_extention;

            //dbg!(&file_name);
            let path = required_dir_path.join(file_name.clone());
            create_file(path.clone());
            //dbg!(required_dir_path.join(file_name.clone()));
            let size = rand::thread_rng().gen_range(required_file_size_bytes + 3..=3000);
            set_file_size(path, size);
        }
    } else {
        panic!("failed to get user dirs");
    }
    let title = format!(
        "Сколько файлов с расширением {} объемом более {} байт содержится в каталоге {}? В ответе укажите только число.

        {} является подкаталогом \"ОГЭ\", который находится в домашнем каталоге.", 
        required_file_extention, required_file_size_bytes, required_dir_name, required_dir_name);

    ExerciseData {
        title,
        right_answer: required_dir_required_file_size_count.to_string(),
        input_field_text: "".into(),
        state: ExcerciseState::NotDone,
        additional_data: Vec::new(),
    }
}

fn create_file(path: PathBuf) {
    Command::new("touch")
        .arg(path)
        .spawn()
        .expect("failed to run touch command");
}

fn set_file_size(path: PathBuf, size: u32) {
    Command::new("truncate")
        .arg("-s")
        .arg(size.to_string())
        .arg(path)
        .spawn()
        .expect("failed to run truncate command");
}
