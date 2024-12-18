use iced::{alignment::{Horizontal, Vertical}, widget::{button, column, container, image::Handle, text, text_input, Image}, Alignment, Length};
use rand::{Rng, distributions::{Distribution, Standard}};
use crate::{num_to_exercise_data, ExcerciseState, ExerciseData, Message};

pub mod exercise_1;
pub mod exercise_2;
pub mod exercise_3;
pub mod exercise_4;
pub mod exercise_5;
pub mod exercise_6;
pub mod exercise_7;
pub mod exercise_8;
pub mod exercise_9;
pub mod exercise_10;
pub mod exercise_11;
pub mod exercise_12;
pub mod exercise_14;
pub mod exercise_15;

pub trait Exercise {
    // Выбор практики или обучения
    fn select_subexercise_view<'a>(done_totally: u32, done_correctly: u32) -> iced::Element<'a, Message> {
        let done_correctly_percent = 
            (done_correctly as f32 / done_totally as f32 * 100.0).round() as u32;
        let buttons_container = container(
            column![
                button(text("Обучение").size(48).center())
                    .width(Length::Fixed(300.0))
                    .height(Length::Fixed(80.0))
                    .on_press(Self::select_learning()),
                button(text("Практика").size(48).center())
                    .width(Length::Fixed(300.0))
                    .height(Length::Fixed(80.0))
                    .on_press(Self::select_subexcersise()),
            ].spacing(15)
        )
            .center(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill);

        let image_handle = Handle::from_bytes(include_bytes!("../back_arrow.png").to_vec());
        let main_container = container(
            column![
                button(Image::new(image_handle).width(100).height(100)).on_press(Message::OpenExcersiseList),
                buttons_container,
                text(format!("Решено всего: {}", done_totally)).size(48).align_x(Horizontal::Center),
                text(format!("Решено правильно: {}({}%)", done_correctly, done_correctly_percent))
                    .size(48).align_x(Horizontal::Center),
            ]);
        main_container.into()
    }

    // Практика
    fn practice_view<'a>(exercise_data: Option<ExerciseData>) -> iced::Element<'a, Message> {
        println!("practice view");
        if let Some(exercise_data) = exercise_data {
            let exercise_container = container(
                column![
                    text(exercise_data.title).size(Self::text_size()).align_x(Horizontal::Center).align_y(Vertical::Center).center(),
                    text_input("Ответ", &exercise_data.input_field_text)
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

            let image_handle = Handle::from_bytes(include_bytes!("../back_arrow.png").to_vec());
            let underlay = container(column![
                button(Image::new(image_handle).width(100).height(100)).on_press(Self::select_excersise()),
                exercise_container,
            ]);

            match exercise_data.state {
                ExcerciseState::NotDone => underlay.into(),
                ExcerciseState::WrongAnswer => {
                    let mut message = Self::new_excersise(false);

                    if unsafe { super::IS_A_TEST } == true {
                        let next_excersise = Self::exercise_number() + 1;
                        if next_excersise > 15 {
                            message = Message::ShowTestResults
                        } else if next_excersise == 13 {
                            message = Message::SelectedSubExcersise(14, num_to_exercise_data(14))
                        } else {
                            message = Message::SelectedSubExcersise(next_excersise, num_to_exercise_data(next_excersise))
                        }
                    }
                    if Self::show_right_answer() {
                        container(
                            column![
                                text(format!("Задание решено неверно!\nПравильный ответ: {}", exercise_data.right_answer))
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

                    if unsafe { super::IS_A_TEST } == true {
                        let next_excersise = Self::exercise_number() + 1;
                        if next_excersise > 15 {
                            message = Message::ShowTestResults;
                            unsafe { super::EXERCISES_DONE_RIGHT.push(Self::exercise_number()); }
                        } else if next_excersise == 13 {
                            message = Message::SelectedSubExcersise(14, num_to_exercise_data(14));
                            unsafe { super::EXERCISES_DONE_RIGHT.push(Self::exercise_number()); }
                        } else {
                            message = Message::SelectedSubExcersise(next_excersise, num_to_exercise_data(next_excersise));
                            unsafe { super::EXERCISES_DONE_RIGHT.push(Self::exercise_number()); }
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
                },
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


    // Создание случайного задания
    fn generate_random_excersise() -> ExerciseData;
    // Обучение
    fn learning_view<'a>() -> iced::Element<'a, Message>;

    fn text_size() -> u16 {
        32
    }

    fn select_subexcersise() -> Message;
    fn select_learning() -> Message;
    fn select_excersise() -> Message;
    fn exercise_number() -> u8;

    fn show_right_answer() -> bool {
        true
    }

    fn new_excersise(done_correctly: bool) -> Message { 
        match done_correctly {
            true => Message::ExcersiseDoneCorrectly(Self::exercise_number()),
            false => Message::ExcersiseDoneWrong(Self::exercise_number()),
        }
    }
}


enum Type2Variations {
    EvenLess,
    EvenLessequals,
    NotevenLess,
    NotevenLessequals,
}

impl Distribution<Type2Variations> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Type2Variations {
        match rng.gen_range(0..4) {
            0 => Type2Variations::EvenLess,
            1 => Type2Variations::EvenLessequals,
            2 => Type2Variations::NotevenLess,
            _ => Type2Variations::NotevenLessequals,
        }
    }
}
