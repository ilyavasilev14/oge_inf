use iced::{widget::{container, column, button, text, Image, text_input}, alignment::{Horizontal, Vertical}, Length, Alignment};
use iced_aw::Modal;
use rand::{Rng, distributions::{Distribution, Standard}};
use crate::{Message, ExcersiseData, ExcersiseState};

pub mod excersise_1;
pub mod excersise_3;
pub mod excersise_5;
pub mod excersise_6;
pub mod excersise_7;
pub mod excersise_10;
pub mod excersise_12;
pub mod excersise_15;

pub trait Exercise {
    // Выбор практики или обучения
    fn select_subexcersise_view<'a>(done_totally: u32, done_correctly: u32) -> iced::Element<'a, Message> {
        let done_correctly_percent = 
            (done_correctly as f32 / done_totally as f32 * 100.0).round() as u32;
        let buttons_container = container(
            column![
                button(text("Обучение").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                    .width(Length::Fixed(300.0))
                    .height(Length::Fixed(80.0))
                    .on_press(Self::select_learning()),
                button(text("Практика").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                    .width(Length::Fixed(300.0))
                    .height(Length::Fixed(80.0))
                    .on_press(Self::select_subexcersise()),
            ].spacing(15)
        )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill);

        let main_container = container(
            column![
                button(Image::new("back_arrow.png").width(100).height(100)).on_press(Message::OpenExcersiseList),
                buttons_container,
                text(format!("Решено всего: {}", done_totally)).size(48).horizontal_alignment(Horizontal::Center),
                text(format!("Решено правильно: {}({}%)", done_correctly, done_correctly_percent))
                    .size(48).horizontal_alignment(Horizontal::Center),
            ]);
        main_container.into()
    }

    // Практика
    fn practice_view<'a>(excersise_data: Option<ExcersiseData>) -> iced::Element<'a, Message> {
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
                    .on_press(Message::CheckAnswer),
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
                            text(format!("Задание решено неверно!\nПравильный ответ: {}", excersise_data.right_answer))
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


    // Создание случайного задания
    fn generate_random_excersise() -> ExcersiseData;
    // Обучение
    fn learning_view<'a>() -> iced::Element<'a, Message>;

    fn text_size() -> u16 {
        48
    }

    fn select_subexcersise() -> Message;
    fn select_learning() -> Message;
    fn select_excersise() -> Message;
    fn excersise_number() -> u8;

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
