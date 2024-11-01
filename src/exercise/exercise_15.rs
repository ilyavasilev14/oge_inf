use iced::{alignment::Horizontal, widget::{button, column, container, scrollable, text, Image}, Alignment, Length};
use rand::{Rng, seq::SliceRandom};
use serde::{Serialize, Deserialize};
use crate::{Message, ExerciseData, ExcerciseState};
use super::Exercise;


pub struct Excersise15 { }

impl Exercise for Excersise15 {
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

    fn practice_view<'a>(excersise_data: Option<ExerciseData>) -> iced::Element<'a, Message> {
        println!("practice view");
        if let Some(excersise_data) = excersise_data {
            let answer: Excersise15Answer = toml::from_str(&excersise_data.right_answer).unwrap();

            let excersise_container = container(
                column![
                    text(excersise_data.title).size(Self::text_size()).center()
                        .width(Length::Fill),

                    button(text("Изменить решение")
                        .size(48)
                        .center())
                        .width(Length::Fixed(500.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::OpenSolutionFile),

                    button(text("Проверить ответ")
                        .size(48)
                        .center())
                        .width(Length::Fixed(500.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::PythonCheckAnswer(answer.input, answer.output.clone())),
                ].align_x(Alignment::Center).spacing(15)
            )
                .center(Length::Fill)
                .width(Length::Fill)
                .height(Length::Fill);

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

                    container(column![
                        text("Задание решено неверно!")
                            .size(48).align_x(Horizontal::Center),
                        button(text("Новое задание").align_x(Horizontal::Center).size(48))
                            .on_press(message).width(500),
                    ]
                    .align_x(Alignment::Center)
                    .spacing(15)).center(Length::Fill).into()
                }
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
                    container(column![
                        text("Задание решено верно!").size(48).align_x(Horizontal::Center),
                        button(text("Новое задание").align_x(Horizontal::Center).size(48))
                            .on_press(message).width(500),
                    ]
                    .align_x(Alignment::Center)
                    .spacing(15)).center(Length::Fill).into()
                },
                ExcerciseState::NanAnswer =>
                    container(column![
                        text("Введите число в ответ задания").size(48),
                        button(text("Исправить ответ").align_x(Horizontal::Center).size(48))
                            .on_press(Message::SetState(ExcerciseState::NotDone)).width(500)
                    ]
                    .align_x(Alignment::Center)
                    .spacing(15)).center(Length::Fill).into(),
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
        Message::SelectedSubExcersise(15, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(15)
    }

    fn exercise_number() -> u8 {
        15
    }

    fn text_size() -> u16 {
        32
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(15)
    }
}


// Max number that may be divided by x
// Напишите программу, которая в последовательности натуральных чисел определяет максимальное число, кратное 5. Программа получает на вход количество чисел в последовательности, а затем сами числа. В последовательности всегда имеется число, кратное 5. Количество чисел не превышает 1000. Введенные числа не превышают 30 000. Программа должна вывести одно число  — максимальное число, кратное 5.
fn generate_excersise_type1() -> ExerciseData {
    let example = Excersise15Data::new_type1();

    let title = format!(
"Напишите программу на языке программирования Python, которая в последовательности натуральных чисел определяет максимальное число, кратное {}, Программа получает на вход количество чисел в последовательности, а затем сами числа. В последовательности всегда имеется число, кратное {}. Количество чисел не превышает 4. Введенные числа не превышают {}. Программа должна вывести одно число - максимальное число, кратное {}.
Пример работы программы:
Входные данные:\n{}
Выходные данные:\n{}", 
    example.division_num, example.division_num, example.max_num, example.division_num, example.num_pretty_str, example.output);

    let answer = Excersise15Data::from_excersise_data_type1(example);
    let answer_str = toml::to_string(&answer).unwrap();
    dbg!(&answer_str);

    ExerciseData { 
        title,
        right_answer: answer_str,
        input_field_text: "".into(), 
        state: ExcerciseState::NotDone,
        additional_data: Vec::new(),
    }
}

// Sum of numbers that may be divided by x
fn generate_excersise_type2() -> ExerciseData {
    let example = Excersise15Data::new_type2();

    let title = format!(
        "Напишите программу на языке программирования Python, которая в последовательности натуральных чисел определяет сумму чисел, кратных {}, Программа получает на вход количество чисел в последовательности, а затем сами числа. В последовательности всегда имеется число, кратное {}. Количество чисел не превышает 4. Введенные числа не превышают {}. Программа должна вывести одно число - сумму чисел, кратных {}.
Пример работы программы:
Входные данные:\n{}
Выходные данные:\n{}", 
        example.division_num, example.division_num, example.max_num, example.division_num, example.num_pretty_str, example.output);

    let answer = Excersise15Data::from_excersise_data_type2(example);
    let answer_str = toml::to_string(&answer).unwrap();
    println!("type 2");
    dbg!(&answer_str);

    ExerciseData { 
        title,
        right_answer: answer_str,
        input_field_text: "".into(), 
        state: ExcerciseState::NotDone,
        additional_data: Vec::new(),
    }
}


#[derive(Serialize, Deserialize)]
struct Excersise15Data {
    pub division_num: u32,
    pub four_number_task: bool,
    pub max_num: u32,
    pub num_count: usize,
    pub numbers: Vec<u32>,
    pub num_pretty_str: String,
    pub output: u32,
}

impl Excersise15Data {
    pub fn new_type2() -> Excersise15Data {
        let division_num = rand::thread_rng().gen_range(2..=5);
        let four_number_task: bool = rand::thread_rng().gen();
        let max_num = rand::thread_rng().gen_range(25..=500);

        let num_count;
        let mut numbers: Vec<u32> = Vec::new();
        let mut right_numbers_count;

        if four_number_task {
            right_numbers_count = rand::thread_rng().gen_range(1..=4);
            num_count = 4;
        } else {
            right_numbers_count = rand::thread_rng().gen_range(1..=3);
            num_count = 3;
        }

        while numbers.len() < num_count {
            let mut num = rand::thread_rng().gen_range(1..max_num);
            if right_numbers_count > 0 {
                right_numbers_count -= 1;
                while num % division_num != 0 {
                    num = rand::thread_rng().gen_range(1..max_num);
                }
            }

            numbers.push(num);
        }

        numbers.shuffle(&mut rand::thread_rng());

        let mut num_pretty_str = num_count.to_string() + "\n";
        let mut output = 0;
        numbers.iter().for_each(|num| {
            num_pretty_str += &num.to_string();
            num_pretty_str += "\n";

            if num % division_num == 0 {
                output += num;
            }
        });
        dbg!(&output);

        Excersise15Data { division_num, four_number_task, max_num, num_count, numbers, num_pretty_str, output }
    }

    pub fn new_type1() -> Excersise15Data {
        let division_num = rand::thread_rng().gen_range(2..=5);
        let four_number_task: bool = rand::thread_rng().gen();
        let max_num = rand::thread_rng().gen_range(25..=500);

        let num_count;
        let mut numbers: Vec<u32> = Vec::new();
        let mut right_numbers_count;

        if four_number_task {
            right_numbers_count = rand::thread_rng().gen_range(1..=4);
            num_count = 4;
        } else {
            right_numbers_count = rand::thread_rng().gen_range(1..=3);
            num_count = 3;
        }

        while numbers.len() < num_count {
            let mut num = rand::thread_rng().gen_range(1..max_num);
            if right_numbers_count > 0 {
                right_numbers_count -= 1;
                while num % division_num != 0 {
                    num = rand::thread_rng().gen_range(1..max_num);
                }
            }

            numbers.push(num);
        }

        numbers.shuffle(&mut rand::thread_rng());

        let mut num_pretty_str = num_count.to_string() + "\n";
        let mut output = 0;
        numbers.iter().for_each(|num| {
            num_pretty_str += &num.to_string();
            num_pretty_str += "\n";

            if num % division_num == 0 && num > &output {
                output = *num;
            }
        });

        Excersise15Data { division_num, four_number_task, max_num, num_count, numbers, num_pretty_str, output }
    }

    pub fn from_excersise_data_type1(answer: Excersise15Data) -> Excersise15Answer {
        let mut numbers: Vec<u32> = Vec::new();
        let mut right_numbers_count;

        if answer.four_number_task {
            right_numbers_count = rand::thread_rng().gen_range(1..=4);
        } else {
            right_numbers_count = rand::thread_rng().gen_range(1..=3);
        }

        while numbers.len() < answer.num_count {
            let mut num = rand::thread_rng().gen_range(1..answer.max_num);
            if right_numbers_count > 0 {
                right_numbers_count -= 1;
                while num % answer.division_num != 0 {
                    num = rand::thread_rng().gen_range(1..answer.max_num);
                }
            }

            numbers.push(num);
        }

        numbers.shuffle(&mut rand::thread_rng());

        let mut input = answer.num_count.to_string() + "\n";
        let mut output = 0;
        numbers.iter().for_each(|num| {
            input += &num.to_string();
            input += "\n";

            if num % answer.division_num == 0 && num > &output {
                output = *num;
            }
        });
        let output = output.to_string();

        Excersise15Answer {
            input,
            output,
        }
    }

    pub fn from_excersise_data_type2(answer: Excersise15Data) -> Excersise15Answer {
        let mut numbers: Vec<u32> = Vec::new();
        let mut right_numbers_count;

        if answer.four_number_task {
            right_numbers_count = rand::thread_rng().gen_range(1..=4);
        } else {
            right_numbers_count = rand::thread_rng().gen_range(1..=3);
        }

        while numbers.len() < answer.num_count {
            let mut num = rand::thread_rng().gen_range(1..answer.max_num);
            if right_numbers_count > 0 {
                right_numbers_count -= 1;
                while num % answer.division_num != 0 {
                    num = rand::thread_rng().gen_range(1..answer.max_num);
                }
            }

            numbers.push(num);
        }

        numbers.shuffle(&mut rand::thread_rng());

        let mut input = answer.num_count.to_string() + "\n";
        let mut output = 0;
        numbers.iter().for_each(|num| {
            input += &num.to_string();
            input += "\n";

            if num % answer.division_num == 0 {
                output += num;
            }
        });
        let output = output.to_string();

        Excersise15Answer {
            input,
            output,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Excersise15Answer {
    pub input: String,
    pub output: String,
}
