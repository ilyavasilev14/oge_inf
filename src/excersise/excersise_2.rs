use iced::{alignment::{Horizontal, Vertical}, widget::{button, column, container, scrollable, text, text_input, Image}, Alignment, Length};
use iced_aw::Modal;
use rand::Rng;
use crate::{Message, ExersiseData, ExcersiseState};
use super::Exercise;


pub struct Excersise2 { }

impl Exercise for Excersise2 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let text: iced::Element<'a, Message> = text("Обучение для этого типа заданий ещё в разработке.")
            .size(Self::text_size())
            .vertical_alignment(iced::alignment::Vertical::Center)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
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

    fn practice_view<'a>(excersise_data: Option<ExersiseData>) -> iced::Element<'a, Message> {
        println!("practice view");
        if let Some(excersise_data) = excersise_data {
            let excersise_container = container(
                column![
                    text(excersise_data.title).size(Self::text_size()).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center)
                        .width(Length::Fill),

                    text_input("Ответ", &excersise_data.input_field_text)
                        .width(Length::Fixed(500.0))
                        .size(48)
                        .on_input(|text| Message::ExcersiseTextInput(text)),

                    button(text("Проверить ответ")
                        .size(48)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(500.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::CheckExcersise2),
                ].align_items(Alignment::Center).spacing(15)
            )
                .center_y()
                .center_x()
                .width(Length::Fill)
                .height(Length::Fill);

            let underlay = container(column![
                button(Image::new("back_arrow.png").width(100).height(100)).on_press(Self::select_excersise()),
                excersise_container,
            ]);

            match excersise_data.state {
                ExcersiseState::NotDone => underlay.into(),
                ExcersiseState::WrongAnswer => 
                    Modal::new(true, underlay, move ||
                        column![
                            text("Задание решено неверно")
                                .size(48).horizontal_alignment(Horizontal::Center),
                            button(text("Новое задание").horizontal_alignment(Horizontal::Center).size(48))
                                .on_press(Self::new_excersise(false)).width(500),
                        ]
                        .align_items(Alignment::Center)
                        .spacing(15)
                        .into())
                    .into(),
                ExcersiseState::RightAnswer => 
                    Modal::new(true, underlay, || column![
                        text("Задание решено верно!").size(48),
                        button(text("Новое задание").horizontal_alignment(Horizontal::Center).size(48))
                            .on_press(Self::new_excersise(true)).width(500),
                    ]
                    .align_items(Alignment::Center)
                    .spacing(15)
                    .into())
                    .into(),
                ExcersiseState::NanAnswer =>
                    Modal::new(true, underlay, || 
                        column![
                            text("Введите число в ответ задания").size(48),
                            button(text("Исправить ответ").horizontal_alignment(Horizontal::Center).size(48))
                                .on_press(Message::SetState(ExcersiseState::NotDone)).width(500)
                        ]
                        .align_items(Alignment::Center)
                        .spacing(15)
                        .into())
                    .into(),
            }
        } else {
            text("NO EXCERSISE DATA").into()
        }
    }

    fn generate_random_excersise() -> ExersiseData {
        /*letters_values.insert(10, "А");
        letters_values.insert(110, "Б");
        letters_values.insert(12, "В");
        Г - 102
        */
        let mut numbers: Vec<u32> = Vec::new();
        let length = rand::thread_rng().gen_range(3..=6);

        for _ in 0..length {
            let letter_type = rand::thread_rng().gen_range(0..=3);
            match letter_type {
                0 => numbers.push(10),// А 
                1 => numbers.push(110), // Б
                2 => numbers.push(12), // В
                3 => numbers.push(102), // Г
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

        ExersiseData {
            title,
            right_answer: num_str,
            input_field_text: String::new(),
            state: ExcersiseState::NotDone,
        }
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(2, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(2)
    }

    fn excersise_number() -> u8 {
        2
    }

    fn text_size() -> u16 {
        32
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(2)
    }
}

