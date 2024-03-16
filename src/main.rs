use std::fs::create_dir;
use std::process::{Command, Stdio};

use directories::UserDirs;
use excersise::excersise_1::Excersise1;
use excersise::excersise_10::Excersise10;
use excersise::Exercise;
use excersise::excersise_12::Excersise12;
use excersise::excersise_15::Excersise15;
use excersise::excersise_3::Excersise3;
use excersise::excersise_5::Excersise5;
use excersise::excersise_6::Excersise6;
use excersise::excersise_7::Excersise7;
use iced::alignment::{Horizontal, Vertical};
use iced::widget::{column, text_input, button, text, container, row};
use iced::{Alignment, Sandbox, Settings, Length};
use login::SaveData;

use crate::login::{init_login, save_login_data, increment_done_correctly_excersise, increment_total_excersise};

mod login;
mod excersise;

fn main() {
    let mut settings = Settings::default();
    settings.window.min_size = Some((1000, 800));
    settings.default_font = Some(include_bytes!("Comfortaa-font.ttf"));
    let _ = App::run(settings);
}

struct App {
    login: String, 
    state: AppState,
    save: SaveData,
    excersise_data: Option<ExcersiseData>
}

enum AppState {
    Login,
    ChoosingExcersise,
    Excersise1,
    Excersise1Learning,
    Excersise1SubExcersise1,
    Excersise3,
    Excersise3Learning,
    Excersise3SubExcersise1,
    Excersise5,
    Excersise5Learning,
    Excersise5SubExcersise1,
    Excersise6,
    Excersise6Learning,
    Excersise6SubExcersise1,
    Excersise7,
    Excersise7Learning,
    Excersise7SubExcersise1,
    Excersise12,
    Excersise12Learning,
    Excersise12SubExcersise1,
    Excersise15,
    Excersise15Learning,
    Excersise15SubExcersise1,
    Excersise10SubExcersise1,
    Excersise10Learning,
    Excersise10,
}

#[derive(Debug, Clone)]
enum Message {
    OpenExcersiseList,
    SelectedExcersise(u8),
    SelectedSubExcersise(u8, u8, ExcersiseData),
    LoginChanged(String),
    ExcersiseTextInput(String),
    CheckAnswer,
    SetState(ExcersiseState),
    /// u8 is excersise number
    ExcersiseDoneWrong(u8),
    /// u8 is excersise number
    ExcersiseDoneCorrectly(u8), 
    OpenSolutionFile, 
    /// input, output
    PythonCheckAnswer(String, String),
    SelectedLearningExcersise(i32),
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self { login: "".into(), state: AppState::Login, excersise_data: None, save: SaveData::default() }
    }

    fn title(&self) -> String {
        "Тренажёр по ОГЭ".into()
    }

    fn update(&mut self, message: Self::Message) {
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
            Message::SelectedExcersise(excersise_number) => {
                self.handle_selecting_excersise(*excersise_number);
            }
            Message::SelectedSubExcersise(excersise_number, subexcersise_number, excersise_data) => {
                self.excersise_data = Some(excersise_data.clone());
                match excersise_number {
                    1 => {
                        match subexcersise_number {
                            1 => self.state = AppState::Excersise1SubExcersise1,
                            _ => todo!()
                        }
                    },
                    3 => {
                        match subexcersise_number {
                            1 => self.state = AppState::Excersise3SubExcersise1,
                            _ => todo!()
                        }
                    },
                    5 => {
                        match subexcersise_number {
                            1 => self.state = AppState::Excersise5SubExcersise1,
                            _ => todo!()
                        }
                    },
                    6 => {
                        match subexcersise_number {
                            1 => self.state = AppState::Excersise6SubExcersise1,
                            _ => todo!()
                        }
                    },
                    7 => {
                        match subexcersise_number {
                            1 => self.state = AppState::Excersise7SubExcersise1,
                            _ => todo!()
                        }
                    },
                    10 => {
                        match subexcersise_number {
                            1 => self.state = AppState::Excersise10SubExcersise1,
                            _ => todo!()
                        }
                    },
                    12 => {
                        match subexcersise_number {
                            1 => self.state = AppState::Excersise12SubExcersise1,
                            _ => todo!()
                        }
                    },
                    15 => {
                        match subexcersise_number {
                            1 => self.state = AppState::Excersise15SubExcersise1,
                            _ => todo!()
                        }
                    },
                    _ => todo!()
                }
            }
            Message::ExcersiseTextInput(text) => {
                if let Some(excersise_data) = &mut self.excersise_data {
                    excersise_data.input_field_text = text.into();
                }
            }
            Message::CheckAnswer => {
                if let Some(excersise_data) = &mut self.excersise_data {
                    if excersise_data.input_field_text == excersise_data.right_answer {
                        excersise_data.state = ExcersiseState::RightAnswer;
                    } else {
                        excersise_data.state = ExcersiseState::WrongAnswer;
                    }
                }
            },
            Message::SetState(state) => {
                if let Some(excersise_data) = &mut self.excersise_data {
                    excersise_data.state = state.to_owned();
                }
                save_login_data(&self.save);
            },
            Message::ExcersiseDoneWrong(excersise_num) => {
                increment_total_excersise(*excersise_num, &mut self.save);
                self.handle_selecting_excersise(*excersise_num);
                save_login_data(&self.save);
            },
            Message::ExcersiseDoneCorrectly(excersise_num) => {
                increment_total_excersise(*excersise_num, &mut self.save);
                increment_done_correctly_excersise(*excersise_num, &mut self.save);
                self.handle_selecting_excersise(*excersise_num);
                save_login_data(&self.save);
            },
            Message::OpenSolutionFile => {
                if let Some(user_dirs) = UserDirs::new() {
                    let home_dir = user_dirs.home_dir();
                    let path = home_dir.join("ОГЭ/solution.py");

                    Command::new("kate")
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
                    if output_str == expected_output {
                        self.excersise_data.as_mut().unwrap().state = ExcersiseState::RightAnswer;
                    } else {
                        self.excersise_data.as_mut().unwrap().state = ExcersiseState::WrongAnswer;
                    }

                    Command::new("killall")
                        .arg("kate")
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
            Message::SelectedLearningExcersise(excersise_number) => {
                match excersise_number {
                    1 => self.state = AppState::Excersise1Learning,
                    3 => self.state = AppState::Excersise3Learning,
                    5 => self.state = AppState::Excersise5Learning,
                    6 => self.state = AppState::Excersise6Learning,
                    7 => self.state = AppState::Excersise7Learning,
                    10 => self.state = AppState::Excersise10Learning,
                    12 => self.state = AppState::Excersise12Learning,
                    15 => self.state = AppState::Excersise15Learning,
                    _ => todo!()
                }
            },
        };

        println!("{:?}", message);
    }

    fn view(&self) -> iced::Element<'_, Self::Message> {
        let save = &self.save;
        let app_state = &self.state;
        match app_state {
            // Если пользователю надо показать экран входа в профиль
            AppState::Login => self.login_view(),
            // Если пользователю надо показать экран выбора задания
            AppState::ChoosingExcersise => self.choosing_view(),
            AppState::Excersise1 => Excersise1::select_subexcersise_view(save.total_done_excersise1, save.done_correctly_excersise1),
            AppState::Excersise1Learning => Excersise1::learning_view(),
            AppState::Excersise1SubExcersise1 => Excersise1::practice_view(self.excersise_data.clone()),
            AppState::Excersise3 => Excersise3::select_subexcersise_view(save.total_done_excersise3, save.done_correctly_excersise3),
            AppState::Excersise3Learning => Excersise3::learning_view(),
            AppState::Excersise3SubExcersise1 => Excersise3::practice_view(self.excersise_data.clone()),
            AppState::Excersise5 => Excersise5::select_subexcersise_view(save.total_done_excersise5, save.done_correctly_excersise5),
            AppState::Excersise5Learning => Excersise5::learning_view(),
            AppState::Excersise5SubExcersise1 => Excersise5::practice_view(self.excersise_data.clone()),
            AppState::Excersise6 => Excersise6::select_subexcersise_view(save.total_done_excersise6, save.done_correctly_excersise6),
            AppState::Excersise6Learning => Excersise6::learning_view(),
            AppState::Excersise6SubExcersise1 => Excersise6::practice_view(self.excersise_data.clone()),
            AppState::Excersise7 => Excersise7::select_subexcersise_view(save.total_done_excersise7, save.done_correctly_excersise7),
            AppState::Excersise7Learning => Excersise7::learning_view(),
            AppState::Excersise7SubExcersise1 => Excersise7::practice_view(self.excersise_data.clone()),
            AppState::Excersise12 => Excersise12::select_subexcersise_view(save.total_done_excersise12, save.done_correctly_excersise12),
            AppState::Excersise12Learning => Excersise12::learning_view(),
            AppState::Excersise12SubExcersise1 => Excersise12::practice_view(self.excersise_data.clone()),
            AppState::Excersise15 => Excersise15::select_subexcersise_view(save.total_done_excersise15, save.done_correctly_excersise15),
            AppState::Excersise15Learning => Excersise15::learning_view(),
            AppState::Excersise15SubExcersise1 => Excersise15::practice_view(self.excersise_data.clone()),
            AppState::Excersise10=> Excersise10::select_subexcersise_view(save.total_done_excersise10, save.done_correctly_excersise10),
            AppState::Excersise10Learning => Excersise10::learning_view(),
            AppState::Excersise10SubExcersise1 => Excersise10::practice_view(self.excersise_data.clone()),
        }
    }
}


impl App {
    fn handle_selecting_excersise(&mut self, excersise_number: u8) {
        match excersise_number {
            1 => self.state = AppState::Excersise1,
            3 => self.state = AppState::Excersise3,
            5 => self.state = AppState::Excersise5,
            6 => self.state = AppState::Excersise6,
            7 => self.state = AppState::Excersise7,
            10 => self.state = AppState::Excersise10,
            12 => self.state = AppState::Excersise12,
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
            .align_items(Alignment::Center)
        )
            .center_y()
            .center_x()
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    pub fn choosing_view(&self) -> iced::Element<'_, Message> {
        container(
            column![
                text("Выбор задания:").size(48),
                row![
                    button(text("1").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(1)),
                    button(text("2").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0)),
                    button(text("3").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(3)),
                    button(text("4").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0)),
                    button(text("5").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(5)),
                ].spacing(15),
                row![
                    button(text("6").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(6)),
                    button(text("7").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(7)),
                    button(text("8").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0)),
                    button(text("9").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0)),
                    button(text("10").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(10)),
                ].spacing(15),
                row![
                    button(text("11").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0)),
                    button(text("12").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(12)),
                    button(text("13").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0)),
                    button(text("14").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0)),
                    button(text("15").size(48).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                        .width(Length::Fixed(80.0))
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(15)),
                ].align_items(Alignment::Center).spacing(15),
                button(text("Составить вариант").size(40).horizontal_alignment(Horizontal::Center).vertical_alignment(Vertical::Center))
                    .width(Length::Fixed(459.0))
                    .height(Length::Fixed(80.0)),
            ].spacing(15)
            .align_items(Alignment::Center)
        )
            .center_y()
            .center_x()
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
pub struct ExcersiseData {
    title: String,
    right_answer: String,
    input_field_text: String,
    state: ExcersiseState
}

#[derive(Clone, Debug)]
pub enum ExcersiseState {
    NotDone,
    WrongAnswer,
    RightAnswer,
    NanAnswer
}
