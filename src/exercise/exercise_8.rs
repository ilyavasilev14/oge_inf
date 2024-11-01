use iced::{alignment::{Horizontal, Vertical}, widget::{button, column, container, row, scrollable, text, text_input, Image}, Alignment, Length};
use rand::{thread_rng, Rng};
use crate::{AdditionalData, ExcerciseState, ExerciseData, Message};
use super::Exercise;


pub struct Excersise8 { }

impl Exercise for Excersise8 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let text: iced::Element<'a, Message> = text("Обучение для этого типа заданий в разработке")
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

    fn practice_view<'a>(excersise_data: Option<ExerciseData>) -> iced::Element<'a, Message> {
        println!("1");
        println!("practice view");
        dbg!(&excersise_data);
        if let Some(excersise_data) = excersise_data {
            let text1: String = unsafe { excersise_data.additional_data_to_i32_unsafe(0).to_string() };
            let text2: String = unsafe { excersise_data.additional_data_to_i32_unsafe(1).to_string() };
            let text3: String = unsafe { excersise_data.additional_data_to_i32_unsafe(2).to_string() };
            let exercise_type = unsafe { excersise_data.additional_data_to_i32_unsafe(3) };

            let excersise_container = container(
                column![
                text(excersise_data.title).size(Self::text_size()).align_x(Horizontal::Center).align_y(Vertical::Center).center(),
                row![letters_col(exercise_type), table_to_col(text1, text2, text3)],

                text_input("Ответ", &excersise_data.input_field_text)
                .align_x(Alignment::Center)
                .width(Length::Fixed(500.0))
                .size(48)
                .on_input(|text| Message::ExcersiseTextInput(text)),

                button(text("Проверить ответ")
                    .size(48)
                    .center())
                .width(Length::Fixed(500.0))
                .height(Length::Fixed(80.0))
                .on_press(Message::CheckAnswer),
                ].align_x(Alignment::Center).spacing(15)
            ).center(Length::Fill);

            let underlay = container(column![
                button(Image::new("back_arrow.png").width(100).height(100)).on_press(Self::select_excersise()),
                excersise_container,
            ]);

            match excersise_data.state {
                ExcerciseState::NotDone => underlay.into(),
                ExcerciseState::WrongAnswer => {
                    let mut message = Self::new_excersise(false);

                    if unsafe { super::super::IS_A_TEST } == true {
                        let next_excersise = Self::exercise_number() + 1;
                        if next_excersise > 15 {
                            message = Message::ShowTestResults
                        } else if next_excersise == 13 {
                            message = Message::SelectedSubExcersise(14, super::super::num_to_exercise_data(14))
                        } else {
                            message = Message::SelectedSubExcersise(next_excersise, super::super::num_to_exercise_data(next_excersise))
                        }
                    }

                    if Self::show_right_answer() {
                        container(
                            column![
                            text(format!("Задание решено неверно!\nПравильный ответ: {}", excersise_data.right_answer))
                            .size(48).align_x(Horizontal::Center),
                            button(text("Новое задание").align_x(Horizontal::Center).size(48))
                            .on_press(message).width(500),
                            ]
                            .align_x(Alignment::Center)
                            .spacing(15)
                        ).center(Length::Fill)
                    } else {
                        container(
                            column![
                            text("Задание решено неверно!")
                            .size(48).align_x(Horizontal::Center),
                            button(text("Новое задание").align_x(Horizontal::Center).size(48))
                            .on_press(message).width(500),
                            ]
                            .spacing(15)
                        ).center(Length::Fill)
                    }.into()
                },
                ExcerciseState::RightAnswer => {
                    let mut message = Self::new_excersise(true);

                    if unsafe { super::super::IS_A_TEST } == true {
                        let next_excersise = Self::exercise_number() + 1;
                        if next_excersise > 15 {
                            message = Message::ShowTestResults;
                            unsafe { super::super::EXERCISES_DONE_RIGHT.push(Self::exercise_number()); }
                        } else if next_excersise == 13 {
                            message = Message::SelectedSubExcersise(14, super::super::num_to_exercise_data(14));
                            unsafe { super::super::EXERCISES_DONE_RIGHT.push(Self::exercise_number()); }
                        } else {
                            message = Message::SelectedSubExcersise(next_excersise, super::super::num_to_exercise_data(next_excersise));
                            unsafe { super::super::EXERCISES_DONE_RIGHT.push(Self::exercise_number()); }
                        }
                    }
                    container(
                        column![
                        text("Задание решено верно!").size(48),
                        button(text("Новое задание").size(48).align_x(Horizontal::Center))
                        .on_press(message).width(500),
                        ]
                        .spacing(15)
                        .align_x(Alignment::Center)
                    ).center(Length::Fill).into()
                }
                ExcerciseState::NanAnswer =>
                    container(
                        column![
                        text("Введите число в ответ задания").size(48),
                        button(text("Исправить ответ").size(48).align_x(Horizontal::Center))
                        .on_press(Message::SetState(ExcerciseState::NotDone)).width(500)
                        ]
                        .align_x(Alignment::Center)
                        .spacing(15)
                    ).center(Length::Fill).into(),
            }
        } else {
            text("NO EXCERSISE DATA").into()
        }
    }

    fn generate_random_excersise() -> ExerciseData {
        let ex_type = rand::thread_rng().gen_range(1..=2);
        match ex_type {
            1 => generate_excersise_type1(),
            _ => generate_excersise_type2(),
        }
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(8, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(8)
    }

    fn exercise_number() -> u8 {
        8
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(8)
    }
}



// A, B, A|B are known, A&B is not
fn generate_excersise_type1() -> ExerciseData {
    let s1: i32 = thread_rng().gen_range(90..=300);
    let s2: i32 = thread_rng().gen_range(90..=300);
    let s3: i32 = thread_rng().gen_range(90..=300);
    let a = s1 + s2;
    let b = s2 + s3;
    let a_or_b = s1 + s2 + s3;
    let a_and_b = s2;

    let text = "В языке запросов поискового сервера для обозначения логической операции «ИЛИ» используется символ «|», а для обозначения логической операции «И»  — символ «&»."
        .to_string();
    let text = (text + "\nВ таблице приведены запросы и количество найденных по ним страниц некоторого сегмента сети Интернет.").to_string();
    let text = (text + "\nКакое количество страниц (в тысячах) будет найдено по запросу \"A & B?\"").to_string();

    ExerciseData {
        title: text,
        right_answer: a_and_b.to_string(),
        input_field_text: String::new(),
        state: ExcerciseState::NotDone,
        // additional_data: a|b, a, b exercise type
        additional_data: vec![
            AdditionalData::I32(a_or_b), 
            AdditionalData::I32(a), 
            AdditionalData::I32(b),
            AdditionalData::I32(1)
        ],
    }
}

// A, B, A&B are known, A|B is not
fn generate_excersise_type2() -> ExerciseData {
    let s1: i32 = thread_rng().gen_range(90..=300);
    let s2: i32 = thread_rng().gen_range(90..=300);
    let s3: i32 = thread_rng().gen_range(90..=300);
    let a = s1 + s2;
    let b = s2 + s3;
    let a_or_b = s1 + s2 + s3;
    let a_and_b = s2;

    let text = "В языке запросов поискового сервера для обозначения логической операции «ИЛИ» используется символ «|», а для обозначения логической операции «И»  — символ «&»."
        .to_string();
    let text = (text + "\nВ таблице приведены запросы и количество найденных по ним страниц некоторого сегмента сети Интернет.").to_string();
    let text = (text + "\nКакое количество страниц (в тысячах) будет найдено по запросу \"A | B?\"").to_string();

    ExerciseData {
        title: text,
        right_answer: a_or_b.to_string(),
        input_field_text: String::new(),
        state: ExcerciseState::NotDone,
        // additional_data: a|b, a, b exercise type
        additional_data: vec![
            AdditionalData::I32(a_and_b), 
            AdditionalData::I32(a), 
            AdditionalData::I32(b),
            AdditionalData::I32(2)
        ],
    }
}

fn letters_col(exercise_type: i32) -> iced::widget::Column<'static, Message> {
    let text1: iced::widget::Text = text("Запрос").size(24).width(Length::Fixed(100.0));
    let text2: iced::widget::Text = match exercise_type {
        1 => text("A | B").size(24).width(Length::Fixed(100.0)),
        2 => text("A & B").size(24).width(Length::Fixed(100.0)),
        _ => unreachable!(),
    };
    let text3: iced::widget::Text = text("A").size(24).width(Length::Fixed(100.0));
    let text4: iced::widget::Text = text("B").size(24).width(Length::Fixed(100.0));
    let col: iced::widget::Column<'_, Message> = column![
        text1, text2, text3, text4
    ];
    col
}

fn table_to_col<'a>(text1: String, text2: String, text3: String) -> iced::widget::Column<'a, Message> {
    let text1_widget: iced::widget::Text = text("Найдено страниц (в тысячах)").size(24).width(Length::Fixed(500.0));
    let text2_widget: iced::widget::Text = text(text1).size(24).width(Length::Fixed(500.0));
    let text3_widget: iced::widget::Text = text(text2).size(24).width(Length::Fixed(500.0));
    let text4_widget: iced::widget::Text = text(text3).size(24).width(Length::Fixed(500.0));
    let col: iced::widget::Column<'_, Message> = column![
        text1_widget, text2_widget, text3_widget, text4_widget
    ];
    col
}

