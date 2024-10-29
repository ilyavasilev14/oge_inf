use std::collections::HashMap;

use font_kit::{family_name::FamilyName, font::Font, properties::Properties, source::SystemSource};
use iced::widget::{button, column, container, scrollable, text, Image};
use rand::{seq::SliceRandom, thread_rng, Rng};
use raqote::{DrawOptions, DrawTarget, PathBuilder, SolidSource, Source, StrokeStyle};
use crate::{Message, ExerciseData};
use super::Exercise;


pub struct Excersise9 { }

impl Exercise for Excersise9 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let text: iced::Element<'a, Message> = text("Обучение для этого типа заданий ещё в разработке.")
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

    fn generate_random_excersise() -> ExerciseData {
        generate_exercise();
        todo!()
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


fn generate_exercise() {
    let mut dt = DrawTarget::new(650, 600);

    let font = SystemSource::new()
        .select_best_match(
            &[FamilyName::Title("Roboto".into())],
            &Properties::new().weight(font_kit::properties::Weight::MEDIUM),
        )
        .unwrap()
        .load()
        .unwrap();

    let a_position = raqote::Point::new(50.0, 300.0);
    let b_position = raqote::Point::new(300.0, 50.0);
    let v_position = raqote::Point::new(350.0, 300.0);
    let g_position = raqote::Point::new(300.0, 550.0);
    let d_position = raqote::Point::new(550.0, 50.0);
    let e_position = raqote::Point::new(550.0, 550.0);
    let k_position = raqote::Point::new(600.0, 300.0);

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

    let answer = paths_count(&graph);
    dbg!(answer);
    dt.write_png("/tmp/oge_training_exercise_9.png").expect("Failed to save an exercise 9 image!");
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

