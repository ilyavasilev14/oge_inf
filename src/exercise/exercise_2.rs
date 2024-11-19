use iced::{alignment::Horizontal, widget::{button, column, container, image::Handle, scrollable, Image}, Length};
use rand::Rng;
use crate::{Message, ExerciseData, ExcerciseState};
use super::Exercise;


pub struct Excersise2 { }

impl Exercise for Excersise2 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let image_handle = Handle::from_bytes(include_bytes!("../learning_exercises_assets/exercise_2_learning.png").to_vec());
        let image: Image<Handle> = Image::new(image_handle).width(951).height(600);


        let scroll = scrollable(column![image].align_x(Horizontal::Center).width(Length::Fill));
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
        let mut right_answer = String::new();
        let mut numbers: Vec<u32> = Vec::new();
        let length = rand::thread_rng().gen_range(3..=6);

        for _ in 0..length {
            let letter_type = rand::thread_rng().gen_range(0..=3);
            match letter_type {
                0 => {
                    numbers.push(10);
                    right_answer.push_str("А");
                },// А 
                1 => {
                    numbers.push(110);
                    right_answer.push_str("Б");
                }, // Б
                2 => {
                    numbers.push(12);
                    right_answer.push_str("В");
                }, // В
                3 => {
                    numbers.push(102);
                    right_answer.push_str("Г");
                }, // Г
                _ => unreachable!()
            }
        }

        dbg!(&numbers);
        let mut num_str = String::new();
        numbers.iter().for_each(|num| {
            num_str += &num.to_string();
        });

        let title = format!(
"Мальчики играли в шпионов и закодировали сообщение придуманным шифром. В сообщении присутствуют только буквы из приведенного фрагмента кодовой таблицы:
А = 10
Б = 110
В = 12
Г = 102
Определите, какое сообщение закодировано в строчке:
{}
В ответе запишите последовательность букв без запятых и других знаков препинания.", num_str);

        ExerciseData {
            title,
            right_answer,
            input_field_text: String::new(),
            state: ExcerciseState::NotDone,
            additional_data: Vec::new(),
        }
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(2, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(2)
    }

    fn exercise_number() -> u8 {
        2
    }

    fn text_size() -> u16 {
        32
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(2)
    }
}

