use directories::UserDirs;
use edit_xlsx::{WorkSheetCol, Workbook, Write};
use iced::{alignment::{Horizontal, Vertical}, widget::{button, column, container, scrollable, text, Image}, Alignment, Length};
use iced_aw::Modal;
use rand::Rng;
use serde::{Deserialize, Serialize};
use crate::{Message, ExersiseData, ExcersiseState};
use super::Exercise;
use std::fs;


pub struct Excersise14 { }

impl Exercise for Excersise14 {
    fn practice_view<'a>(excersise_data: Option<ExersiseData>) -> iced::Element<'a, Message> {
        println!("practice view");
        if let Some(excersise_data) = excersise_data {
            let excersise_container = container(
                column![
                    text(excersise_data.title).size(Self::text_size()).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center)
                        .width(Length::Fill),

                    button(text("Проверить ответ")
                        .size(48)
                        .horizontal_alignment(Horizontal::Center)
                        .vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(500.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::Excersise14CheckAnswer),
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
                            text("Задание решено неверно!")
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

    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let text: iced::Element<'a, Message> = text("Обучение для задания 14 сейчас находится в разработке.")
            .size(Self::text_size())
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

    fn generate_random_excersise() -> ExersiseData {
        generate_excersise()
    }

    fn select_subexcersise() -> Message {
        println!("select_subexcersise");
        Message::SelectedSubExcersise(14, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(14)
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(14)
    }

    fn excersise_number() -> u8 {
        14
    }

    fn text_size() -> u16 {
        32
    }

    fn show_right_answer() -> bool {
        false
    }
}



fn generate_excersise() -> ExersiseData {
    let mut student_col = vec!["Ученик".into()];
    for i in 1..=1001 {
        student_col.push(format!("Ученик {}", i));
    }

    let mut school_col = vec![];
    for _ in 1..=1001 {
        school_col.push(rand::thread_rng().gen_range(10..=150));
    }

    let avg_class = rand::thread_rng().gen_range(6..=9);
    let mut avg_class_ids = Vec::new();
    let mut avg_class_scores = Vec::new();

    let mut class_col = vec![];
    for i in 1..=1001 {
        let class = rand::thread_rng().gen_range(6..=9);
        if class == avg_class {
            avg_class_ids.push(i);
        }
        class_col.push(class);
    }

    let mut four_or_five = 0;
    let mut score_col = vec![];
    for i in 1..=1001 {
        let score = rand::thread_rng().gen_range(2..=5);
        if avg_class_ids.contains(&i) {
            avg_class_scores.push(score);
        }

        if score >= 4 {
            four_or_five += 1;
        }

        score_col.push(score);
    }

    let mut avg_score = 0.0;
    for score in &avg_class_scores {
        avg_score += *score as f32;
    };
    dbg!(&avg_score);
    let avg_score: f32 = format!("{:.2}", avg_score / avg_class_scores.len() as f32).parse().unwrap();
    dbg!(&avg_score);

    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir();
        let home_dir = home_dir.join("ОГЭ/");
        let file_path = home_dir.join("14.xlsx");
        dbg!(&file_path);
        let remove_old_file_result = fs::remove_file(&file_path);
        let _ = dbg!(remove_old_file_result);


        let mut workbook = Workbook::new();
        if let Ok(worksheet) = workbook.get_worksheet_mut(1) {
            let _ = worksheet.set_columns_width_pixels("A:A", 30.0);
            student_col.iter().enumerate().for_each(|(idx, val)| {
                let _ = worksheet.write(&format!("A{}", idx + 2), val.clone());
            });
            school_col.iter().enumerate().for_each(|(idx, val)| {
                let _ = worksheet.write(&format!("B{}", idx + 2), val.clone());
            });
            class_col.iter().enumerate().for_each(|(idx, val)| {
                let _ = worksheet.write(&format!("C{}", idx + 2), val.clone());
            });
            score_col.iter().enumerate().for_each(|(idx, val)| {
                let _ = worksheet.write(&format!("D{}", idx + 2), val.clone());
            });
            let _ = worksheet.write("A1", "Ученик");
            let _ = worksheet.write("B1", "Школа");
            let _ = worksheet.write("C1", "Класс");
            let _ = worksheet.write("D1", "Оценка");


            if let Err(err) = workbook.save_as(&file_path) {
                panic!("failed to save a workbook to path '{:?}'!\nerr: {:?}", &file_path, err);
            }
        } else {
            panic!("failed to get a worksheet from a workbook!");
        }
    }

    let title = format!("В электронную таблицу(файл \"14.xlsx\", находящийся в подкаталоге домашней директории \"ОГЭ\") занесли данные о результатах экзамена учеников.
В столбце A записан код ученика; в столбце B – номер школы, в которой он обучается; в столбце C – класс ученика; в столбце D – оценка за работу.
Всего в электронную таблицу были занесены данные 1000 учеников.
На основании данных в этой таблице выполните задания:
1. Найдите средний балл учащихся {} класса. Ответ запишите в ячейку H2 таблицы.
2. Сколько учеников получили оценку 4 или 5? Ответ запишите в ячейку H3 таблицы.", avg_class).into();

    let answer = Exersise14Answer {
        avg_score,
        four_or_five
    };

    ExersiseData {
        title,
        right_answer: toml::to_string(&answer).unwrap().into(),
        input_field_text: "".into(),
        state: ExcersiseState::NotDone,
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Exersise14Answer {
    pub avg_score: f32,
    pub four_or_five: i32
}
