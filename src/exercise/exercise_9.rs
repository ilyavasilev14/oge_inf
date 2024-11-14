use std::{collections::HashMap, time::SystemTime};
use font_kit::{family_name::FamilyName, font::Font, properties::Properties, source::SystemSource};
use iced::{alignment::{Horizontal, Vertical}, widget::{button, column, container, image::Handle, scrollable, text, text_input, Image}, Alignment, Length};
use rand::{seq::SliceRandom, thread_rng, Rng};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source, StrokeStyle};
use crate::{AdditionalData, ExcerciseState, ExerciseData, Message};
use super::Exercise;


pub struct Excersise9 { }

impl Exercise for Excersise9 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let image_handle = Handle::from_bytes(include_bytes!("../learning_exercises_assets/exercise_9_learning.png").to_vec());
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

    fn practice_view<'a>(exercise_data: Option<ExerciseData>) -> iced::Element<'a, Message> {
        println!("practice view");
        if let Some(exercise_data) = exercise_data {
            let image: Image<Handle> = unsafe { Image::new(exercise_data.additional_data_to_string_unsafe(0)) };
            let exercise_container = container(
                column![
                    text(exercise_data.title).size(Self::text_size()).align_x(Horizontal::Center).align_y(Vertical::Center).center(),
                    image.width(433).height(200),
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

                    if unsafe { super::super::IS_A_TEST } == true {
                        let next_excersise = Self::exercise_number() + 1;
                        if next_excersise > 15 {
                            message = Message::ShowTestResults;
                        } else if next_excersise == 13 {
                            message = Message::SelectedSubExcersise(14, super::super::num_to_exercise_data(14));
                        } else {
                            message = Message::SelectedSubExcersise(next_excersise, super::super::num_to_exercise_data(next_excersise));
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
        generate_exercise()
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(9, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(9)
    }

    fn exercise_number() -> u8 {
        9
    }

    fn text_size() -> u16 {
        32
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(9)
    }
}


fn generate_exercise() -> ExerciseData {
    let mut dt = DrawTarget::new(650, 300);

    let font = SystemSource::new()
        .select_best_match(
            &[FamilyName::Title("Roboto".into())],
            &Properties::new().weight(font_kit::properties::Weight::MEDIUM),
        )
        .unwrap()
        .load()
        .unwrap();

    let a_position = raqote::Point::new(50.0, 150.0);
    let b_position = raqote::Point::new(300.0, 50.0);
    let v_position = raqote::Point::new(350.0, 150.0);
    let g_position = raqote::Point::new(300.0, 250.0);
    let d_position = raqote::Point::new(550.0, 50.0);
    let e_position = raqote::Point::new(550.0, 250.0);
    let k_position = raqote::Point::new(600.0, 150.0);

    // k is excluded because it won't connect to anything anyways
    let letters = vec!["А", "Б", "В", "Г", "Д", "Е"]; 
    let mut possible_letters_connections = HashMap::new();
    possible_letters_connections.insert("А", vec!["Б", "В", "Г"]);
    possible_letters_connections.insert("Б", vec!["В", "Д", "К"]);
    possible_letters_connections.insert("В", vec!["Г", "К", "Е"]);
    possible_letters_connections.insert("Г", vec!["К", "Е"]);
    possible_letters_connections.insert("Д", vec!["К"]);
    possible_letters_connections.insert("Е", vec!["К"]);

    draw_text("А", &mut dt, &font, a_position);
    draw_text("Б", &mut dt, &font, b_position);
    draw_text("В", &mut dt, &font, v_position);
    draw_text("Г", &mut dt, &font, g_position);
    draw_text("Д", &mut dt, &font, d_position);
    draw_text("Е", &mut dt, &font, e_position);
    draw_text("К", &mut dt, &font, k_position);

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for letter in letters {
        let mut possible_connections = possible_letters_connections[letter].clone();
        possible_connections.shuffle(&mut thread_rng());
        let max_possible_connections = possible_connections.len();
        let connections_count = thread_rng().gen_range(1..=max_possible_connections);
        let mut connections = Vec::new();

        for connection in 0..connections_count {
            connections.push(possible_connections[connection])
        }

        for connection in connections {
            let start_position = match letter {
                "А" => a_position,
                "Б" => b_position,
                "В" => v_position,
                "Г" => g_position,
                "Д" => d_position,
                "Е" => e_position,
                _ => unreachable!(),
            };

            let finish_position = match connection {
                "А" => a_position,
                "Б" => b_position,
                "В" => v_position,
                "Г" => g_position,
                "Д" => d_position,
                "Е" => e_position,
                "К" => k_position,
                _ => unreachable!(),
            };

            match graph.get_mut(letter) {
                Some(connections) => connections.push(connection),
                None => {
                    graph.insert(letter, vec![connection]);
                },
            }
            draw_arrow(&mut dt, start_position, finish_position);
        }
    }

    let right_answer = paths_count(&graph).to_string();
    dbg!(&right_answer);
    let unix_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs().to_string();
    let file_name = "/tmp/oge_traning_9_".to_string() + &unix_time + ".png";
    dt.write_png(file_name.clone()).expect("Failed to save an exercise 9 image!");

    let title = "На рисунке изображена схема дорог, связывающих города А, Б, В, Г, Д, Е, К. По каждой дороге можно двигаться только в одном направлении, указанном стрелкой. Сколько существует различных путей из города А в город К?".into();

    ExerciseData {
        title,
        right_answer,
        input_field_text: String::new(),
        state: ExcerciseState::NotDone,
        additional_data: vec![AdditionalData::String(file_name)],
    }
}



fn draw_text(text: &str, dt: &mut DrawTarget, font: &Font, position: raqote::Point) {
    dt.draw_text(
        &font,
        30.,
        text,
        position,
        &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)),
        &DrawOptions::new(),
    );
}

fn draw_arrow(dt: &mut DrawTarget, start_point: raqote::Point, finish_point: raqote::Point) {
    let arrow_direction = arrow_direction(start_point, finish_point);
    dbg!(&arrow_direction);


    let mut path_builder = PathBuilder::new();
    let mut arrow_path_builder = PathBuilder::new();

    match arrow_direction {
        ArrowDirection::UpRight => {
            path_builder.move_to(start_point.x + 15.0, start_point.y - 25.0);
            path_builder.line_to(finish_point.x + 1.0, finish_point.y + 10.0);

            arrow_path_builder.move_to(finish_point.x + 5.0, finish_point.y + 7.0);
            arrow_path_builder.line_to(finish_point.x - 7.0, finish_point.y + 10.0);
            arrow_path_builder.line_to(finish_point.x + 3.0, finish_point.y + 15.0);
        },
        ArrowDirection::UpLeft => {
            path_builder.move_to(start_point.x + 5.0, start_point.y - 25.0);
            path_builder.line_to(finish_point.x + 14.0, finish_point.y + 8.0);

            arrow_path_builder.move_to(finish_point.x + 12.0, finish_point.y + 4.0);
            arrow_path_builder.line_to(finish_point.x + 9.0, finish_point.y + 12.0);
            arrow_path_builder.line_to(finish_point.x + 21.0, finish_point.y + 8.0);
        },
        ArrowDirection::Left => {
            path_builder.move_to(start_point.x - 2.0, start_point.y - 10.0);
            path_builder.line_to(finish_point.x + 20.0, finish_point.y - 10.0);

            arrow_path_builder.move_to(finish_point.x + 20.0, finish_point.y - 10.0);
            arrow_path_builder.line_to(finish_point.x + 23.0, finish_point.y - 13.0);
            arrow_path_builder.line_to(finish_point.x + 23.0, finish_point.y - 7.0);
        },
        ArrowDirection::Right => {
            path_builder.move_to(start_point.x + 20.0, start_point.y - 10.0);
            path_builder.line_to(finish_point.x - 3.0, finish_point.y - 10.0);

            arrow_path_builder.move_to(finish_point.x - 3.0, finish_point.y - 10.0);
            arrow_path_builder.line_to(finish_point.x - 6.0, finish_point.y - 13.0);
            arrow_path_builder.line_to(finish_point.x - 6.0, finish_point.y - 7.0);
        },
        ArrowDirection::DownRight => {
            path_builder.move_to(start_point.x + 10.0, start_point.y + 3.0);
            path_builder.line_to(finish_point.x, finish_point.y - 25.0);

            arrow_path_builder.move_to(finish_point.x - 1.0, finish_point.y - 24.0);
            arrow_path_builder.line_to(finish_point.x - 5.0, finish_point.y - 23.0);
            arrow_path_builder.line_to(finish_point.x, finish_point.y - 28.0);
        },
        ArrowDirection::DownLeft => {
            path_builder.move_to(start_point.x + 10.0, start_point.y + 3.0);
            path_builder.line_to(finish_point.x + 20.0, finish_point.y - 25.0);

            arrow_path_builder.move_to(finish_point.x + 19.0, finish_point.y - 24.0);
            arrow_path_builder.line_to(finish_point.x + 18.0, finish_point.y - 28.0);
            arrow_path_builder.line_to(finish_point.x + 23.0, finish_point.y - 23.0);
        },
        ArrowDirection::Down => {
            path_builder.move_to(start_point.x + 10.0, start_point.y + 3.0);
            path_builder.line_to(finish_point.x + 10.0, finish_point.y - 25.0);

            arrow_path_builder.move_to(finish_point.x + 10.0, finish_point.y - 25.0);
            arrow_path_builder.line_to(finish_point.x + 13.0, finish_point.y - 28.0);
            arrow_path_builder.line_to(finish_point.x + 7.0, finish_point.y - 28.0);
        },
        ArrowDirection::Up => {
            path_builder.move_to(start_point.x + 10.0, start_point.y - 20.0);
            path_builder.line_to(finish_point.x + 10.0, finish_point.y + 3.0);

            arrow_path_builder.move_to(finish_point.x + 10.0, finish_point.y + 3.0);
            arrow_path_builder.line_to(finish_point.x + 13.0, finish_point.y + 6.0);
            arrow_path_builder.line_to(finish_point.x + 7.0, finish_point.y + 6.0);
        },
    }

    path_builder.close();
    let path = path_builder.finish();
    dt.stroke(&path, &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)), &StrokeStyle { width: 3.0, ..Default::default() }, &DrawOptions::default());

    arrow_path_builder.close();
    let arrow_path = arrow_path_builder.finish();
    dt.stroke(&arrow_path, &Source::Solid(SolidSource::from_unpremultiplied_argb(255, 0, 0, 0)), &StrokeStyle { width: 3.0, ..Default::default() }, &DrawOptions::default());
}

fn arrow_direction(start_point: raqote::Point, finish_point: raqote::Point) -> ArrowDirection {
    if start_point.y == finish_point.y {
        if start_point.x > finish_point.x {
            return ArrowDirection::Left
        } else {
            return ArrowDirection::Right
        }
    }
    if (start_point.x - finish_point.x).abs() < 60.0 {
        if finish_point.y > start_point.y {
            return ArrowDirection::Down
        } else {
            return ArrowDirection::Up
        }
    }

    if start_point.x > finish_point.x {
        if finish_point.y > start_point.y {
            return ArrowDirection::DownLeft
        } else {
            return ArrowDirection::UpLeft
        }
    } else {
        if finish_point.y > start_point.y {
            return ArrowDirection::DownRight
        } else {
            return ArrowDirection::UpRight
        }
    }
}

#[derive(Debug)]
enum ArrowDirection {
    UpRight,
    UpLeft,
    Left,
    Right,
    DownRight,
    DownLeft,
    Down,
    Up
}

fn paths_count(graph: &HashMap<&str, Vec<&str>>) -> u32 {
    let mut paths_count = 0;
    let mut queue = Vec::new();
    queue.push(("А", vec![]));

    while queue.len() > 0 {
        let node = queue[0].0;
        let mut already_visited = queue[0].1.clone();

        let destinations = graph.get(node);
        if let Some(destinations) = destinations {
            for destination in destinations {
                if already_visited.contains(destination) {
                    continue;
                }

                if *destination == "К" {
                    paths_count += 1;
                } else {
                    already_visited.push(node.into());
                    queue.push((destination, already_visited.clone()));
                }
            }
        }
        queue.remove(0);
    }

    paths_count
}

