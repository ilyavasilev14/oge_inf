use std::collections::HashMap;
use iced::widget::{button, column, container, scrollable, text, Image};
use rand::{Rng, distributions::{Alphanumeric, DistString}, seq::SliceRandom};
use crate::{Message, ExerciseData, ExcerciseState};
use super::Exercise;


pub struct Excersise7 { }

impl Exercise for Excersise7 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let text: iced::Element<'a, Message> = text(
"   Есть несколько типов 7-го задания: адрес почтового ящика, адрес файла.
    Решение задания с адресом файла: сначала указывается протокол (например, \"http\" или \"ftp\"), потом «://», потом сервер, затем «/», далее - файла указывается в конце. Пример: http://ya.ru/index.html
    Решение задания с адресом почтового ящика: сначала указывается имя почтового ящика, затем «@», а потом сервер. Пример: address@mail.ru")
            .size(48)
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
        //let ex_type = rand::thread_rng().gen_range(0..4);
        let ex_type = rand::thread_rng().gen_range(1..=2);
        match ex_type {
            1 => generate_excersise_type1(),
            _ => generate_excersise_type2(),
        }
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(7, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(7)
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(7)
    }

    fn exercise_number() -> u8 {
        7
    }
}



fn generate_excersise_type1() -> ExerciseData {
    let protocol = if rand::thread_rng().gen::<bool>() {
        "ftp"
    } else {
        "http"
    };
    let colon_double_slash = "://";

    let server_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=8));
    let server_name_ending = ".".to_string() + &Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(2..=4));

    let slash = "/";
    let file_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(6..=8));
    let file_name_ending = ".".to_string() + &Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(2..=5));

    let letters_list: Vec<&str> = vec!["А", "Б", "В", "Г", "Д", "Е", "Ж"];
    let mut used_letters: Vec<&str> = Vec::new();
    let mut letter_val: HashMap<&str, &str> = HashMap::new();
    let mut answer: String = String::new();

    let protocol_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    answer += protocol_letter;
    used_letters.push(&protocol_letter);
    letter_val.insert(protocol_letter, protocol);

    let mut colon_double_slash_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    while used_letters.contains(colon_double_slash_letter) {
        colon_double_slash_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    }
    used_letters.push(&colon_double_slash_letter);
    answer += colon_double_slash_letter;
    letter_val.insert(&colon_double_slash_letter, colon_double_slash);

    let mut server_name_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    while used_letters.contains(server_name_letter) {
        server_name_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    }
    used_letters.push(&server_name_letter);
    answer += server_name_letter;
    letter_val.insert(&server_name_letter, &server_name);

    let mut server_name_ending_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    while used_letters.contains(server_name_ending_letter) {
        server_name_ending_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    }
    used_letters.push(&server_name_ending_letter);
    answer += server_name_ending_letter;
    letter_val.insert(&server_name_ending_letter, &server_name_ending);

    let mut slash_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    while used_letters.contains(slash_letter) {
        slash_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    }
    used_letters.push(&slash_letter);
    answer += slash_letter;
    letter_val.insert(&slash_letter, &slash);

    let mut file_name_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    while used_letters.contains(file_name_letter) {
        file_name_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    }
    used_letters.push(&file_name_letter);
    answer += file_name_letter;
    letter_val.insert(&file_name_letter, &file_name);

    let mut file_name_ending_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    while used_letters.contains(file_name_ending_letter) {
        file_name_ending_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    }
    used_letters.push(&file_name_ending_letter);
    answer += file_name_ending_letter;
    letter_val.insert(&file_name_ending_letter, &file_name_ending);



    let title = format!(
"Доступ к файлу {}{}, находящемуся на сервере {}{}, осуществляется по протоколу {}. Фрагменты адреса файла закодированы буквами от А до Ж. Запишите последовательность этих букв, кодирующую адрес указанного файла в сети Интернет.
А) {}
Б) {}
В) {}
Г) {}
Д) {}
Е) {}
Ж) {}", 
    file_name, file_name_ending, server_name, server_name_ending, protocol,
    letter_val["А"], letter_val["Б"], letter_val["В"], letter_val["Г"], letter_val["Д"], 
    letter_val["Е"], letter_val["Ж"]);


    ExerciseData { 
        title, 
        right_answer: answer,
        input_field_text: "".into(), 
        state: ExcerciseState::NotDone,
        additional_data: Vec::new(),
    }
} // http/ftp

fn generate_excersise_type2() -> ExerciseData {
    let mailbox = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(4..=10));

    let at = "@";
    let server_name = Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(6..=8));
    let server_name_ending = ".".to_string() + &Alphanumeric.sample_string(&mut rand::thread_rng(), rand::thread_rng().gen_range(2..=5));

    let letters_list: Vec<&str> = vec!["А", "Б", "В", "Г"];
    let mut used_letters: Vec<&str> = Vec::new();
    let mut letter_val: HashMap<&str, &str> = HashMap::new();
    let mut answer: String = String::new();


    let mut mailbox_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    while used_letters.contains(mailbox_letter) {
        mailbox_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    }
    used_letters.push(&mailbox_letter);
    answer += mailbox_letter;
    letter_val.insert(&mailbox_letter, &mailbox);

    let mut at_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    while used_letters.contains(at_letter) {
        at_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    }
    used_letters.push(&at_letter);
    answer += at_letter;
    letter_val.insert(&at_letter, &at);

    let mut server_name_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    while used_letters.contains(server_name_letter) {
        server_name_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    }
    used_letters.push(&server_name_letter);
    answer += server_name_letter;
    letter_val.insert(&server_name_letter, &server_name);

    let mut server_name_ending_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    while used_letters.contains(server_name_ending_letter) {
        server_name_ending_letter = letters_list.choose(&mut rand::thread_rng()).unwrap();
    }
    used_letters.push(&server_name_ending_letter);
    answer += server_name_ending_letter;
    letter_val.insert(&server_name_ending_letter, &server_name_ending);



    let title = format!(
"На сервере {}{} находится почтовый ящик {}. Фрагменты адреса электронной почты закодированы буквами от А до Г. Запишите последовательность букв, кодирующую этот адрес.
А) {}
Б) {}
В) {}
Г) {}",
    server_name, server_name_ending, mailbox, letter_val["А"], letter_val["Б"], letter_val["В"], letter_val["Г"]);


    ExerciseData { 
        title, 
        right_answer: answer,
        input_field_text: "".into(), 
        state: ExcerciseState::NotDone,
        additional_data: Vec::new(),
    }

} // mail
 
