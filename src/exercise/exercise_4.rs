use std::collections::HashMap;
use iced::{alignment::{Horizontal, Vertical}, widget::{button, column, container, row, scrollable, text, text_input, Image}, Alignment, Length};
use rand::{seq::SliceRandom, thread_rng, Rng};
use crate::{AdditionalData, ExcerciseState, ExerciseData, Message};
use super::Exercise;


pub struct Excersise4 { }

impl Exercise for Excersise4 {
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
        
        println!("practice view");
        if let Some(excersise_data) = excersise_data {
            if let AdditionalData::Graph(graph) = &excersise_data.additional_data[0] {
                let col1 = graph_to_col(graph.clone(), "A".into());
                let col2 = graph_to_col(graph.clone(), "B".into());
                let col3 = graph_to_col(graph.clone(), "C".into());
                let col4 = graph_to_col(graph.clone(), "D".into());
                let col5 = graph_to_col(graph.clone(), "E".into());

                let excersise_container = container(
                    column![
                    text(excersise_data.title).size(Self::text_size()).align_x(Horizontal::Center).align_y(Vertical::Center).center(),
                    row![letters_col(), col1, col2, col3, col4, col5],

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
                        let mut message = Self::new_excersise(true);

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
                                .on_press(Self::new_excersise(false)).width(500),
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
        } else {
            text("NO EXCERSISE DATA").into()
        }
    }

    fn generate_random_excersise() -> ExerciseData {
        generate_excersise_type1()
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(4, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(4)
    }

    fn exercise_number() -> u8 {
        4 
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(4)
    }
}



fn generate_excersise_type1() -> ExerciseData {
    let letters = vec!["A", "B", "C", "D"];
    let mut shuffled_letters = letters.clone();
    shuffled_letters.shuffle(&mut thread_rng());
    let mut roads = Vec::new();
    let mut graph: HashMap<&str, Vec<(String, u32)>> = HashMap::new();
    graph.insert("A", Vec::new());
    graph.insert("B", Vec::new());
    graph.insert("C", Vec::new());
    graph.insert("D", Vec::new());
    graph.insert("E", Vec::new());

    let a_to_e_road_count: usize = thread_rng().gen_range(2..=4);
    for (idx, letter) in shuffled_letters.iter().enumerate() {
        if a_to_e_road_count < idx {
            break;
        }

        roads.push(vec![letter.to_string()]);
        let mut subroads_count = 0;
        let mut shuffled_letters = shuffled_letters.clone();
        shuffled_letters.retain(|x| x != letter);
        // bool - if it is true, connect the letter straight to E, otherwise connect it to
        // something else
        while thread_rng().gen_bool(0.3) == false && subroads_count < shuffled_letters.len() {
            subroads_count += 1;
            let mut next_letter_idx: usize = thread_rng().gen_range(0..shuffled_letters.len());
            while roads[idx].contains(&shuffled_letters[next_letter_idx].to_string()) {
                next_letter_idx = thread_rng().gen_range(0..shuffled_letters.len());
            }
            roads[idx].push(shuffled_letters[next_letter_idx].into());
        }
        roads[idx].push("E".into());
    }

    // create the graph
    for road in &roads {
        for letters in road.iter().zip(road.iter().skip(1)) {
            let weight: u32 = thread_rng().gen_range(1..=9);
            match graph.get_mut(letters.0.as_str()) {
                Some(graph_element) => {
                    let mut contains_letter = false;
                    for (letter, _) in &*graph_element {
                        if letter == letters.1 {
                            contains_letter = true;
                        }
                    }

                    if !contains_letter {
                        graph_element.push((letters.1.into(), weight))
                    }
                },
                None => println!("ex 4: graph element is none!"),
            }

            match graph.get_mut(letters.1.as_str()) {
                Some(graph_element) => {
                    let mut contains_letter = false;
                    for (letter, _) in &*graph_element {
                        if letter == letters.0 {
                            contains_letter = true;
                        }
                    }

                    if !contains_letter {
                        graph_element.push((letters.0.into(), weight))
                    }
                },
                None => println!("ex 4: graph element is none!"),
            }
        }
    }
    let shortest = find_shortest_path(&graph);
    let right_answer = {
        if shortest != u32::MAX {
            shortest
        } else {
            0
        }
    }.to_string();

    let text = "Между населенными пунктами A, B, C, D, E построены дороги, протяженность которых (в километрах) приведена в таблице.".to_string();
    let text = text + "\nОпределите длину кратчайшего пути между пунктами А и E. Передвигаться можно только по дорогам, протяженность которых указана в таблице.";
    let text = text.to_string() + "Если подходящего пути нет, в ответе укажите \"0\"";
    let mut additional_data_graph = HashMap::new();
    for (key, value) in graph {
        additional_data_graph.insert(key.to_string(), value);
    }
    
    ExerciseData {
        title: text,
        right_answer,
        input_field_text: String::new(),
        state: ExcerciseState::NotDone,
        additional_data: vec![AdditionalData::Graph(additional_data_graph)],
    }
}

/// finds the shortest path from the node A to the node E. if there's none, returns u32::MAX
fn find_shortest_path(graph: &HashMap<&str, Vec<(String, u32)>>) -> u32 {
    let mut shortest = u32::MAX;
    let mut queue = Vec::new();
    queue.push(("A", 0, vec![]));

    while queue.len() > 0 {
        let node = queue[0].0;
        let node_weight = queue[0].1;
        let mut already_visited = queue[0].2.clone();

        let destinations = graph.get(node);
        if let Some(destinations) = destinations {
            for (destination, weight) in destinations {
                if already_visited.contains(destination) {
                    continue;
                }

                let total_weight = node_weight + weight;
                if destination == "E" {
                    if total_weight < shortest {
                        shortest = total_weight;
                    }
                } else {
                    already_visited.push(node.into());
                    queue.push((destination, total_weight, already_visited.clone()));
                }
            }
        }
        queue.remove(0);
    }

    shortest
}

fn letters_col() -> iced::widget::Column<'static, Message> {
    let text1: iced::widget::Text = text(" ").size(24).width(Length::Fixed(50.0));
    let text2: iced::widget::Text = text("A").size(24).width(Length::Fixed(50.0));
    let text3: iced::widget::Text = text("B").size(24).width(Length::Fixed(50.0));
    let text4: iced::widget::Text = text("C").size(24).width(Length::Fixed(50.0));
    let text5: iced::widget::Text = text("D").size(24).width(Length::Fixed(50.0));
    let text6: iced::widget::Text = text("E").size(24).width(Length::Fixed(50.0));
    let col: iced::widget::Column<'_, Message> = column![
        text1, text2, text3, text4, text5, text6
    ];
    col
}

fn graph_to_col<'a>(graph: HashMap<String, Vec<(String, u32)>>, letter: String) -> iced::widget::Column<'a, Message> {
    let paths = graph.get(&letter).unwrap();
    let text1: iced::widget::Text = text(letter.clone()).size(24);
    let text2: iced::widget::Text = text(graph_get_path_len_str(paths, "A")).size(24).width(Length::Fixed(50.0));
    let text3: iced::widget::Text = text(graph_get_path_len_str(paths, "B")).size(24).width(Length::Fixed(50.0));
    let text4: iced::widget::Text = text(graph_get_path_len_str(paths, "C")).size(24).width(Length::Fixed(50.0));
    let text5: iced::widget::Text = text(graph_get_path_len_str(paths, "D")).size(24).width(Length::Fixed(50.0));
    let text6: iced::widget::Text = text(graph_get_path_len_str(paths, "E")).size(24).width(Length::Fixed(50.0));
    let col: iced::widget::Column<'_, Message> = column![
        text1, text2, text3, text4, text5, text6
    ];
    col
}

fn graph_get_path_len_str(paths: &Vec<(String, u32)>, letter: &str) -> String {
    for (destination, weight) in paths {
        if destination == letter {
            return weight.to_string();
        }
    }

    String::new()
}
