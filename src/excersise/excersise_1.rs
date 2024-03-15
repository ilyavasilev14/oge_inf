use iced::widget::text;
use rand::{distributions::{Alphanumeric, DistString}, Rng};
use crate::{ExcersiseData, ExcersiseState, Message};
use super::Exercise;


pub struct Excersise1 { }

impl Exercise for Excersise1 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        text("ad;slkfjfjasdlk;fjfj")
            .size(Self::text_size())
            .into()
    }

    fn generate_random_excersise() -> ExcersiseData {
        //let ex_type = rand::thread_rng().gen_range(0..4);
        let ex_type = rand::thread_rng().gen_range(1..=2);
        match ex_type {
            1 => generate_excersise_type1(),
            _ => generate_excersise_type2(),
        }
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(1, 1, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(1)
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(1)
    }

    fn excersise_number() -> u8 {
        1
    }

    fn text_size() -> u16 {
        36
    }
}



// From < number to >
fn generate_excersise_type1() -> ExcersiseData {
    let excersise_type = rand::thread_rng().gen_range(0..=2);
    let mut right_answer = "".into();
    let symbol_size: &str;
    let mut reduced_size = 0;
    let mut symbols_string = String::new();

    match excersise_type {
        0 => { // 8bit 
            let min_symbols_count = rand::thread_rng().gen_range(1..=5);
            let count = rand::thread_rng().gen_range(4..=6);
            let reduced_size_id = rand::thread_rng().gen_range(0..count);
            let mut strings: Vec<String> = Vec::new();
            for count in 0..count {
                let string = Alphanumeric.sample_string(&mut rand::thread_rng(), min_symbols_count + count);
                if reduced_size_id == count {
                    right_answer = string.clone();
                    reduced_size = string.len() + 2;
                }

                symbols_string += &string;
                symbols_string += ", ";
                strings.push(string);
            }
            symbol_size = "КОИ-8 кодируется 8 битами";
        }, 
        1 => { // 16bit
            let min_symbols_count = rand::thread_rng().gen_range(1..=5);
            let count = rand::thread_rng().gen_range(4..=6);
            let reduced_size_id = rand::thread_rng().gen_range(0..count);
            let mut strings: Vec<String> = Vec::new();
            for count in 0..count {
                let string = Alphanumeric.sample_string(&mut rand::thread_rng(), min_symbols_count + count);
                if reduced_size_id == count {
                    right_answer = string.clone();
                    reduced_size = (string.len() + 2) * 2;
                }

                symbols_string += &string;
                symbols_string += ", ";
                strings.push(string);
            }
            symbol_size = "UTF-16 кодируется 16 битами";
        },
        2 => { // 32bit
            let min_symbols_count = rand::thread_rng().gen_range(1..=5);
            let count = rand::thread_rng().gen_range(4..=6);
            let reduced_size_id = rand::thread_rng().gen_range(0..count);
            let mut strings: Vec<String> = Vec::new();
            for count in 0..count {
                let string = Alphanumeric.sample_string(&mut rand::thread_rng(), min_symbols_count + count);
                if reduced_size_id == count {
                    right_answer = string.clone();
                    reduced_size = (string.len() + 2) * 4;
                }

                symbols_string += &string;
                symbols_string += ", ";
                strings.push(string);
            }
            symbol_size = "UTF-32 кодируется 32 битами";
        }
        _ => unreachable!()
    }

    symbols_string.pop();
    symbols_string.pop();

    let title = 
format!("Каждый символ в кодировке {}. Миша написал текст (в нем нет лишних пробелов):

«{} - наборы символов».

Ученик вычеркнул из списка один из наборов символов. Заодно он вычеркнул ставшие лишними запятые и пробелы - два пробела не должны идти подряд.
При этом размер нового предложения в данной кодировке оказался на {} байтов меньше, чем размер исходного предложения. Напишите в ответе вычеркнутый набор символов.",
    symbol_size, symbols_string, reduced_size);
    ExcersiseData {
        title,
        right_answer,
        input_field_text: "".into(),
        state: ExcersiseState::NotDone,
    }
}

fn generate_excersise_type2() -> ExcersiseData {
    let excersise_type = rand::thread_rng().gen_range(0..=2);
    let char_size_description: &str;
    let char_size;
    let pages_count = rand::thread_rng().gen_range(5..=20);
    let lines_count = rand::thread_rng().gen_range(10..=40);
    let char_count = rand::thread_rng().gen_range(50..=100);

    match excersise_type {
        0 => {
            char_size_description = "КОИ-8 кодируется 8 битами";
            char_size = 8;
        } // 8bit
        1 => {
            char_size_description = "UTF-16 кодируется 16 битами";
            char_size = 16;
        } // 16bit
        2 => {
            char_size_description = "UTF-32 кодируется 32 битами";
            char_size = 32;
        } // 32bit
        _ => unreachable!()
    }

    let right_answer_num = pages_count * lines_count * char_count * char_size / 8;
    let right_answer = right_answer_num.to_string();



    let title = 
format!("Каждый символ в кодировке {}. Текст, набранный на компьютере, содержит {} страниц, каждая по {} строк, а каждая строка состоит из {} символов.
Определите информационный объём текста в Кбайтах.", char_size_description, pages_count, lines_count, char_count);
    ExcersiseData {
        title,
        right_answer,
        input_field_text: "".into(),
        state: ExcersiseState::NotDone,
    }
}

