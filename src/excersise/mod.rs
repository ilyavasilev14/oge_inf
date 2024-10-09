use iced::{alignment::{Horizontal, Vertical}, widget::{button, column, container, text, text_input, Image}, Alignment, Length};
use rand::{Rng, distributions::{Distribution, Standard}};
use crate::{Message, ExerciseData, ExcerciseState};

pub mod excersise_1;
pub mod excersise_2;
pub mod excersise_3;
pub mod exercise_4;
pub mod excersise_5;
pub mod excersise_6;
pub mod excersise_7;
pub mod exercise_8;
pub mod excersise_10;
pub mod excersise_12;
pub mod excersise_14;
pub mod excersise_15;

pub trait Exercise {
    // Выбор практики или обучения
    fn select_subexcersise_view<'a>(done_totally: u32, done_correctly: u32) -> iced::Element<'a, Message> {
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

        let main_container = container(
            column![
                button(Image::new("back_arrow.png").width(100).height(100)).on_press(Message::OpenExcersiseList),
                buttons_container,
                text(format!("Решено всего: {}", done_totally)).size(48).align_x(Horizontal::Center),
                text(format!("Решено правильно: {}({}%)", done_correctly, done_correctly_percent))
                    .size(48).align_x(Horizontal::Center),
            ]);
        main_container.into()
    }

    // Практика
    fn practice_view<'a>(excersise_data: Option<ExerciseData>) -> iced::Element<'a, Message> {
        println!("practice view");
        if let Some(excersise_data) = excersise_data {
            let excersise_container = container(
                column![
                    text(excersise_data.title).size(Self::text_size()).align_x(Horizontal::Center).align_y(Vertical::Center).center(),
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
                    if Self::show_right_answer() {
                        container(
                            column![
                                text(format!("Задание решено неверно!\nПравильный ответ: {}", excersise_data.right_answer))
                                    .size(48).align_x(Horizontal::Center),
                                    button(text("Новое задание").align_x(Horizontal::Center).size(48))
                                        .on_press(Self::new_excersise(false)).width(500),
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
                                    .on_press(Self::new_excersise(false)).width(500),
                            ]
                            .spacing(15)
                        ).center(Length::Fill)
                    }.into()
                },
                ExcerciseState::RightAnswer => 
                    container(
                        column![
                            text("Задание решено верно!").size(48),
                            button(text("Новое задание").size(48).align_x(Horizontal::Center))
                                .on_press(Self::new_excersise(true)).width(500),
                        ]
                        .spacing(15)
                        .align_x(Alignment::Center)
                    ).center(Length::Fill).into(),
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
    fn excersise_number() -> u8;

    fn show_right_answer() -> bool {
        true
    }

    fn new_excersise(done_correctly: bool) -> Message { 
        match done_correctly {
            true => Message::ExcersiseDoneCorrectly(Self::excersise_number()),
            false => Message::ExcersiseDoneWrong(Self::excersise_number()),
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
