use iced::widget::text;
use rand::{distributions::{Alphanumeric, DistString}, Rng};
use crate::{ExcersiseData, ExcersiseState, Message};
use super::Exercise;


pub struct Excersise10 { }

impl Exercise for Excersise10 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        text("Обучение в процессе разработки.")
            .size(Self::text_size())
            .into()
    }

    fn generate_random_excersise() -> ExcersiseData {
        let number_2 = rand::thread_rng().gen_range(5..=40);
        let mut max_number = number_2;
        let mut number_8 = rand::thread_rng().gen_range(5..=40);
        while number_8 == number_2 {
            number_8 = rand::thread_rng().gen_range(5..=40);
        }
        if number_8 > max_number {
            max_number = number_8;
        }
        let mut number_16 = rand::thread_rng().gen_range(5..=40);
        while number_16 == number_8 || number_16 == number_8 {
            number_16 = rand::thread_rng().gen_range(5..=40);
        }
        if number_16 > max_number {
            max_number = number_16;
        }

        let num_2_converted = format!("{:b}", number_2);
        let num_8_converted = format!("{:o}", number_8);
        let num_16_converted = format!("{:X}", number_16);

        let title = format!(
"Среди приведенных ниже трех чисел, записанных в различных системах счисления, найдите максимальное и запишите его в ответе в десятичной системе счисления. В ответе запишите только число, основание системы счисления указывать не нужно.\n
{}(16)\n{}(8)\n{}(2)", num_16_converted, num_8_converted, num_2_converted);
        ExcersiseData {
            title,
            right_answer: max_number.to_string(),
            input_field_text: "".into(),
            state: ExcersiseState::NotDone,
        }
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(10, 1, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(10)
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(10)
    }

    fn excersise_number() -> u8 {
        10
    }

    fn text_size() -> u16 {
        36
    }
}

