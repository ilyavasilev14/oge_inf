use std::{collections::HashMap, fs::{self, File}, io::Write};

use directories::UserDirs;
use iced::widget::{button, column, container, scrollable, text, Image};
use rand::{distributions::{Alphanumeric, DistString}, Rng};
use crate::{Message, ExerciseData, ExcerciseState};
use super::Exercise;


pub struct Excersise11 { }

impl Exercise for Excersise11 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let text: iced::Element<'a, Message> = text(("В этом задании необходимо изучить содержимое файла, который указан в задании.".to_string()
            + " Для решения этого задания откройте необходимый файл в текстовом редакторе, используйте функцию поиска для того, чтобы быстро найти нужную информацию.").to_string()
            + " Если в задании необходимо найти имя какого-либо героя произведения, учитывайте, что данная вам информация может использоваться в разных падежах.")
            .size(Self::text_size())
            .center()
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
        generate_excersise_type1()
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(11, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(11)
    }

    fn exercise_number() -> u8 {
        11
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(11)
    }
}

fn generate_excersise_type1() -> ExerciseData {
    let required_parameter_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(8..=12));
    let required_parameter_value = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(8..=12));
    let file_parameters_count: usize = rand::thread_rng().gen_range(1000..=1500);
    let required_parameter_position: usize = rand::thread_rng().gen_range(100..=file_parameters_count);

    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir();
        let home_dir = home_dir.join("ОГЭ/11.txt");
        dbg!(&home_dir);
        let _ = fs::remove_file(&home_dir);
        let file = File::create(&home_dir);
        match file {
            Ok(mut file) => {
                let mut parameters = HashMap::new();
                for i in 0..=file_parameters_count {
                    if i == required_parameter_position {
                        parameters.insert(required_parameter_name.clone(), required_parameter_value.clone());
                        continue
                    }

                    let mut parameter_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(8..=12));
                    let parameter_value = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(8..=12));
                    while parameter_name == required_parameter_name {
                        parameter_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(8..=12));
                    }

                    parameters.insert(parameter_name, parameter_value);
                }

                let parameters_string = toml::to_string_pretty(&parameters).expect("Failed to serialize the parameters HashMap to a toml string!");

                file.write(parameters_string.as_bytes()).expect("Failed to write the contents of the file!");
            },
            Err(_) => todo!(),
        }
    }

    let title = format!(
        "{}\"{}\"\n{}",
        "Откройте файл \"11.txt\" и найдите, чему равен параметер",
        required_parameter_name,
        "В ответ укажите только цифры и латинские буквы. Файл \"11.txt\" находится в каталоге \"ОГЭ\", который находится в домашней директории.");

    ExerciseData {
        title,
        right_answer: required_parameter_value,
        input_field_text: "".into(),
        state: ExcerciseState::NotDone,
        additional_data: Vec::new(),
    }
}

