use std::collections::HashMap;
use std::fs::create_dir;
use std::process::{Command, Stdio};

use calamine::{Data, Reader, Xlsx};
use directories::UserDirs;
use exercise::exercise_1::Excersise1;
use exercise::exercise_10::Excersise10;
use exercise::exercise_11::Excersise11;
use exercise::exercise_14::Excersise14;
use exercise::exercise_2::Excersise2;
use exercise::exercise_4::Excersise4;
use exercise::exercise_8::Excersise8;
use exercise::exercise_9::Excersise9;
use exercise::Exercise;
use exercise::exercise_12::Excersise12;
use exercise::exercise_15::Excersise15;
use exercise::exercise_3::Excersise3;
use exercise::exercise_5::Excersise5;
use exercise::exercise_6::Excersise6;
use exercise::exercise_7::Excersise7;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, text_input, button, text, container, row};
use iced::{Alignment, Length};
use login::SaveData;

use crate::exercise::exercise_14::Exersise14Answer;
use crate::login::{init_login, save_login_data, increment_done_correctly_exercise, increment_total_exercise};

mod login;
mod exercise;

fn main() {
    let run_result = iced::run("Тренажёр для ОГЭ по информатике", App::update, App::view);
    if let Err(err) = run_result {
        dbg!(err);
    }
}

struct App {
    login: String, 
    state: AppState,
    save: SaveData,
    exersise_data: Option<ExerciseData>
}

enum AppState {
    Login,
    ChoosingExcersise,
    Excersise1,
    Excersise1Learning,
    Excersise1Practice,
    Excersise2,
    Excersise2Practice,
    Excersise2Learning,
    Excersise3,
    Excersise3Learning,
    Excersise3Practice,
    Excersise4,
    Excersise4Learning,
    Excersise4Practice,
    Excersise5,
    Excersise5Learning,
    Excersise5Practice,
    Excersise6,
    Excersise6Learning,
    Excersise6Practice,
    Excersise7,
    Excersise7Learning,
    Excersise7Practice,
    Excersise8,
    Excersise8Learning,
    Excersise8Practice,
    Excersise12,
    Excersise12Learning,
    Excersise12Practice,
    Excersise15,
    Excersise15Learning,
    Excersise15Practice,
    Excersise10,
    Excersise10Learning,
    Excersise10Practice,
    Excersise11,
    Excersise11Learning,
    Excersise11Practice,
    Excersise14Practice,
    Excersise14Learning,
    Excersise14,
    Excersise9Practice,
    Excersise9Learning,
    Excersise9,
}

#[derive(Debug, Clone)]
enum Message {
    OpenExcersiseList,
    SelectedExcersise(u8),
    SelectedSubExcersise(u8, ExerciseData),
    LoginChanged(String),
    ExcersiseTextInput(String),
    CheckAnswer,
    SetState(ExcerciseState),
    /// u8 is exercise number
    ExcersiseDoneWrong(u8),
    /// u8 is exercise number
    ExcersiseDoneCorrectly(u8), 
    OpenSolutionFile, 
    /// input, output
    PythonCheckAnswer(String, String),
    /// exercise14answer serialized to toml
    Excersise14CheckAnswer,
    SelectedLearningExcersise(i32),
}

impl Default for App {
    fn default() -> Self {
        Self { login: "".into(), state: AppState::Login, exersise_data: None, save: SaveData::default() }
    }
}

impl App {
    fn update(&mut self, message: Message) {
        match &message {
            // При нажатии на кнопку входа
            Message::OpenExcersiseList => {
                // Загружаем профиль
                self.save = init_login(&self.login);
                // Создаём директорию ~/ОГЭ/ для заданий 12-15
                create_app_directory();
                // Меняем состояние программы на выбор упражнения
                self.state = AppState::ChoosingExcersise;
            },
            Message::LoginChanged(new_text) => self.login = new_text.into(),
            Message::SelectedExcersise(exercise_number) => {
                self.handle_selecting_exercise(*exercise_number);
            }
            Message::SelectedSubExcersise(exercise_number, exercise_data) => {
                self.exersise_data = Some(exercise_data.clone());
                match exercise_number {
                    1 => {
                        self.state = AppState::Excersise1Practice;
                    },
                    2 => {
                        self.state = AppState::Excersise2Practice;
                    },
                    3 => {
                        self.state = AppState::Excersise3Practice;
                    },
                    4 => {
                        self.state = AppState::Excersise4Practice;
                    },
                    5 => {
                        self.state = AppState::Excersise5Practice;
                    },
                    6 => {
                        self.state = AppState::Excersise6Practice;
                    },
                    7 => {
                        self.state = AppState::Excersise7Practice;
                    },
                    8 => {
                        self.state = AppState::Excersise8Practice;
                    },
                    9 => {
                        self.state = AppState::Excersise9Practice;
                    },
                    10 => {
                        self.state = AppState::Excersise10Practice;
                    },
                    11 => {
                        self.state = AppState::Excersise11Practice;
                    },
                    12 => {
                        self.state = AppState::Excersise12Practice;
                    },
                    14 => {
                        self.state = AppState::Excersise14Practice;
                    },
                    15 => {
                        self.state = AppState::Excersise15Practice;
                    },
                    _ => todo!()
                }
            }
            Message::ExcersiseTextInput(text) => {
                if let Some(exercise_data) = &mut self.exersise_data {
                    exercise_data.input_field_text = text.into();
                }
            }
            Message::CheckAnswer => {
                if let Some(exercise_data) = &mut self.exersise_data {
                    if exercise_data.input_field_text == exercise_data.right_answer {
                        exercise_data.state = ExcerciseState::RightAnswer;
                    } else {
                        exercise_data.state = ExcerciseState::WrongAnswer;
                    }
                }
            },
            Message::SetState(state) => {
                if let Some(exercise_data) = &mut self.exersise_data {
                    exercise_data.state = state.to_owned();
                }
                save_login_data(&self.save);
            },
            Message::ExcersiseDoneWrong(exercise_num) => {
                increment_total_exercise(*exercise_num, &mut self.save);
                self.handle_selecting_exercise(*exercise_num);
                save_login_data(&self.save);
            },
            Message::ExcersiseDoneCorrectly(exercise_num) => {
                increment_total_exercise(*exercise_num, &mut self.save);
                increment_done_correctly_exercise(*exercise_num, &mut self.save);
                self.handle_selecting_exercise(*exercise_num);
                save_login_data(&self.save);
            },
            Message::OpenSolutionFile => {
                if let Some(user_dirs) = UserDirs::new() {
                    let home_dir = user_dirs.home_dir();
                    let path = home_dir.join("ОГЭ/solution.py");

                    Command::new("kwrite")
                        .arg(path)
                        .output()
                        .expect("Failed to execute command 'kate'");
                } else {
                    unreachable!()
                }
            },
            Message::PythonCheckAnswer(input, expected_output) => {
                if let Some(user_dirs) = UserDirs::new() {
                    let home_dir = user_dirs.home_dir();
                    let path = home_dir.join("ОГЭ/solution.py");

                    let echo = Command::new("echo")//Command::new(format!("echo {} | python", input));
                        .arg(input)
                        .stdout(Stdio::piped())
                        .spawn()
                        .unwrap();

                    let output = Command::new("python")
                        .arg(&path)
                        .stdin(Stdio::from(echo.stdout.unwrap()))
                        .output()
                        .unwrap();

                    let output_str = &String::from_utf8(output.stdout).unwrap();
                    let output_str = &output_str.replace("\n", "");
                    dbg!(&output_str);
                    dbg!(&expected_output);
                    if output_str == expected_output {
                        self.exersise_data.as_mut().unwrap().state = ExcerciseState::RightAnswer;
                    } else {
                        self.exersise_data.as_mut().unwrap().state = ExcerciseState::WrongAnswer;
                    }

                    Command::new("killall")
                        .arg("kwrite")
                        .spawn()
                        .unwrap();
                    Command::new("rm")
                        .arg(path)
                        .spawn()
                        .unwrap();
                } else {
                    unreachable!()
                }
            },
            Message::Excersise14CheckAnswer => {
                if let Some(exersise_data) = &mut self.exersise_data {
                    let answer = toml::from_str::<Exersise14Answer>(&exersise_data.right_answer).unwrap();
                    dbg!(&answer);
                    if let Some(user_dirs) = UserDirs::new() {
                        let home_dir = user_dirs.home_dir();
                        let home_dir = home_dir.join("ОГЭ/");
                        let file_path = home_dir.join("14.xlsx");
                        let mut workbook: Xlsx<_> = calamine::open_workbook(&file_path).unwrap();
                        let worksheet = &workbook.worksheets()[0].1;
                        dbg!(&file_path);

                        let avg_number_value: Option<&Data> = worksheet.get_value((1, 7));
                        match avg_number_value {
                            Some(avg_number_value) => {
                                let avg_number;
                                match avg_number_value {
                                    Data::Int(value) => avg_number = *value as f32,
                                    Data::Float(value) => avg_number = *value as f32,
                                    _ => return exersise_data.state = ExcerciseState::WrongAnswer
                                }
                                let avg_number: f32 = format!("{:.2}", avg_number).parse().unwrap();

                                if &avg_number != &answer.avg_score.parse::<f32>().unwrap() {
                                    return exersise_data.state = ExcerciseState::WrongAnswer;
                                }
                            },
                            None => return exersise_data.state = ExcerciseState::WrongAnswer,
                        }
                        let four_or_five_value: Option<&Data> = worksheet.get_value((2, 7));
                        match four_or_five_value {
                            Some(four_or_five_value) => {
                                let four_or_five;
                                match four_or_five_value {
                                    Data::Int(value) => four_or_five = *value as i32,
                                    Data::Float(value) => four_or_five = (value.round()).clone() as i32,
                                    _ => return exersise_data.state = ExcerciseState::WrongAnswer
                                }

                                if &four_or_five != &answer.four_or_five {
                                    return exersise_data.state = ExcerciseState::WrongAnswer;
                                }
                            },
                            None => return exersise_data.state = ExcerciseState::WrongAnswer,
                        }
                    }
                    exersise_data.state = ExcerciseState::RightAnswer;
                }
            },
            Message::SelectedLearningExcersise(exercise_number) => {
                match exercise_number {
                    1 => self.state = AppState::Excersise1Learning,
                    2 => self.state = AppState::Excersise2Learning,
                    3 => self.state = AppState::Excersise3Learning,
                    4 => self.state = AppState::Excersise4Learning,
                    5 => self.state = AppState::Excersise5Learning,
                    6 => self.state = AppState::Excersise6Learning,
                    7 => self.state = AppState::Excersise7Learning,
                    8 => self.state = AppState::Excersise8Learning,
                    9 => self.state = AppState::Excersise9Learning,
                    10 => self.state = AppState::Excersise10Learning,
                    11 => self.state = AppState::Excersise11Learning,
                    12 => self.state = AppState::Excersise12Learning,
                    14 => self.state = AppState::Excersise14Learning,
                    15 => self.state = AppState::Excersise15Learning,
                    _ => todo!()
                }
            },
        };

        println!("{:?}", message);
    }

    fn view(&self) -> iced::Element<'_, Message> {
        let save = &self.save;
        let app_state = &self.state;
        match app_state {
            // Если пользователю надо показать экран входа в профиль
            AppState::Login => self.login_view(),
            // Если пользователю надо показать экран выбора задания
            AppState::ChoosingExcersise => self.choosing_view(),
            AppState::Excersise1 => Excersise1::select_subexercise_view(save.total_done_exercise1, save.done_correctly_exercise1),
            AppState::Excersise1Learning => Excersise1::learning_view(),
            AppState::Excersise1Practice => Excersise1::practice_view(self.exersise_data.clone()),
            AppState::Excersise2 => Excersise2::select_subexercise_view(save.total_done_exercise2, save.done_correctly_exercise2),
            AppState::Excersise2Learning => Excersise2::learning_view(),
            AppState::Excersise2Practice => Excersise2::practice_view(self.exersise_data.clone()),
            AppState::Excersise3 => Excersise3::select_subexercise_view(save.total_done_exercise3, save.done_correctly_exercise3),
            AppState::Excersise3Learning => Excersise3::learning_view(),
            AppState::Excersise3Practice => Excersise3::practice_view(self.exersise_data.clone()),
            AppState::Excersise4 => Excersise4::select_subexercise_view(save.total_done_exercise4, save.done_correctly_exercise4),
            AppState::Excersise4Learning => Excersise4::learning_view(),
            AppState::Excersise4Practice => Excersise4::practice_view(self.exersise_data.clone()),
            AppState::Excersise5 => Excersise5::select_subexercise_view(save.total_done_exercise5, save.done_correctly_exercise5),
            AppState::Excersise5Learning => Excersise5::learning_view(),
            AppState::Excersise5Practice => Excersise5::practice_view(self.exersise_data.clone()),
            AppState::Excersise6 => Excersise6::select_subexercise_view(save.total_done_exercise6, save.done_correctly_exercise6),
            AppState::Excersise6Learning => Excersise6::learning_view(),
            AppState::Excersise6Practice => Excersise6::practice_view(self.exersise_data.clone()),
            AppState::Excersise7 => Excersise7::select_subexercise_view(save.total_done_exercise7, save.done_correctly_exercise7),
            AppState::Excersise7Learning => Excersise7::learning_view(),
            AppState::Excersise7Practice => Excersise7::practice_view(self.exersise_data.clone()),
            AppState::Excersise8 => Excersise8::select_subexercise_view(save.total_done_exercise8, save.done_correctly_exercise8),
            AppState::Excersise8Learning => Excersise8::learning_view(),
            AppState::Excersise8Practice => Excersise8::practice_view(self.exersise_data.clone()),
            AppState::Excersise9 => Excersise9::select_subexercise_view(save.total_done_exercise9, save.done_correctly_exercise9),
            AppState::Excersise9Learning => Excersise9::learning_view(),
            AppState::Excersise9Practice => Excersise9::practice_view(self.exersise_data.clone()),
            AppState::Excersise12 => Excersise12::select_subexercise_view(save.total_done_exercise12, save.done_correctly_exercise12),
            AppState::Excersise12Learning => Excersise12::learning_view(),
            AppState::Excersise12Practice => Excersise12::practice_view(self.exersise_data.clone()),
            AppState::Excersise14 => Excersise14::select_subexercise_view(save.total_done_exercise12, save.done_correctly_exercise12),
            AppState::Excersise14Learning => Excersise14::learning_view(),
            AppState::Excersise14Practice => Excersise14::practice_view(self.exersise_data.clone()),
            AppState::Excersise15 => Excersise15::select_subexercise_view(save.total_done_exercise15, save.done_correctly_exercise15),
            AppState::Excersise15Learning => Excersise15::learning_view(),
            AppState::Excersise15Practice => Excersise15::practice_view(self.exersise_data.clone()),
            AppState::Excersise10=> Excersise10::select_subexercise_view(save.total_done_exercise10, save.done_correctly_exercise10),
            AppState::Excersise10Learning => Excersise10::learning_view(),
            AppState::Excersise10Practice => Excersise10::practice_view(self.exersise_data.clone()),
            AppState::Excersise11 => Excersise11::select_subexercise_view(save.total_done_exercise10, save.done_correctly_exercise10),
            AppState::Excersise11Learning => Excersise11::learning_view(),
            AppState::Excersise11Practice => Excersise11::practice_view(self.exersise_data.clone()),
        }
    }
}


impl App {
    fn handle_selecting_exercise(&mut self, exercise_number: u8) {
        match exercise_number {
            1 => self.state = AppState::Excersise1,
            2 => self.state = AppState::Excersise2,
            3 => self.state = AppState::Excersise3,
            4 => self.state = AppState::Excersise4,
            5 => self.state = AppState::Excersise5,
            6 => self.state = AppState::Excersise6,
            7 => self.state = AppState::Excersise7,
            8 => self.state = AppState::Excersise8,
            9 => self.state = AppState::Excersise9,
            10 => self.state = AppState::Excersise10,
            11 => self.state = AppState::Excersise11,
            12 => self.state = AppState::Excersise12,
            14 => self.state = AppState::Excersise14,
            15 => self.state = AppState::Excersise15,
            _ => todo!()
        }
    }

    fn login_view(&self) -> iced::Element<'_, Message> {
        container(
            column![
                text_input("Логин", &self.login)
                    .width(Length::Fixed(250.0))
                    .on_input(|text| Message::LoginChanged(text)),
                button(text("Войти в профиль").size(24))
                    .width(Length::Fixed(250.0))
                    .on_press(Message::OpenExcersiseList),
            ].spacing(15)
            .align_x(Alignment::Center)
        )
            .center(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn choosing_view(&self) -> iced::Element<'_, Message> {
        container(
            column![
                text("Выбор задания:").size(48),
                row![
                    button(text("1").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(1)),
                    button(text("2").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(2)),
                    button(text("3").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(3)),
                    button(text("4").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(4)),
                    button(text("5").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(5)),
                ].spacing(15),
                row![
                    button(text("6").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(6)),
                    button(text("7").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(7)),
                    button(text("8").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(8)),
                    button(text("9").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(9)),
                    button(text("10").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(10)),
                ].spacing(15),
                row![
                    button(text("11").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(11)),
                    button(text("12").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(12)),
                    button(text("13").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0)),
                    button(text("14").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(14)),
                    button(text("15").size(48).align_x(Horizontal::Center).align_y(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(15)),
                ].spacing(15),
                button(text("Составить вариант").size(40).align_x(Horizontal::Center).align_y(Vertical::Center))
                    .width(Length::Fixed(459.0))
                    .height(Length::Fixed(80.0)),
            ].spacing(15)
        )
        .center(Length::Fill)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn create_app_directory() {
    if let Some(user_dirs) = UserDirs::new() {
        let home_dir = user_dirs.home_dir();
        let path = home_dir.join("ОГЭ/");
        let dir = create_dir(path);
        if let Err(dir_creation_error) = dir {
            dbg!(dir_creation_error);
        }
    }
}

#[derive(Clone, Debug)]
pub struct ExerciseData {
    title: String,
    right_answer: String,
    input_field_text: String,
    state: ExcerciseState,
    additional_data: Vec<AdditionalData>
}

impl ExerciseData {
    /// panics if the additional data type is not i32
    unsafe fn additional_data_to_i32_unsafe(&self, index: usize) -> i32 {
        if let AdditionalData::I32(value) = self.additional_data[index] {
            value
        } else {
            panic!("additional_data_to_i32_unsafe panic: the data type of the value is not i32")
        }
    }

    unsafe fn additional_data_to_string_unsafe(&self, index: usize) -> String {
        if let AdditionalData::String(value) = self.additional_data[index].clone() {
            value
        } else {
            panic!("additional_data_to_i32_unsafe panic: the data type of the value is not string")
        }
    }
}

#[derive(Clone, Debug)]
pub enum ExcerciseState {
    NotDone,
    WrongAnswer,
    RightAnswer,
    NanAnswer
}

#[derive(Clone, Debug)]
pub enum AdditionalData {
    I32(i32),
    String(String),
    Vec(Vec<AdditionalData>),
    Graph(HashMap<String, Vec<(String, u32)>>),
}
