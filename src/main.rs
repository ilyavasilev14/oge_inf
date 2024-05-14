use std::fs::create_dir;
use std::process::{Command, Stdio};

use calamine::{Data, Reader, Xlsx};
use directories::UserDirs;
use edit_xlsx::{Read, Workbook};
use excersise::excersise_1::Excersise1;
use excersise::excersise_10::Excersise10;
use excersise::excersise_14::Excersise14;
use excersise::excersise_2::Excersise2;
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

use crate::excersise::excersise_14::Exersise14Answer;
use crate::login::{init_login, save_login_data, increment_done_correctly_excersise, increment_total_excersise};

mod login;
mod excersise;

fn main() {
    let mut settings = Settings::default();
    settings.default_font = Some(include_bytes!("Montserrat-Medium.ttf"));
    let _ = App::run(settings);
}

struct App {
    login: String, 
    state: AppState,
    save: SaveData,
    exersise_data: Option<ExersiseData>
}

enum AppState {
    Login,
    ChoosingExcersise,
    Excersise1,
    Excersise1Learning,
    Excersise1Practice,
    Excersise3,
    Excersise3Learning,
    Excersise3Practice,
    Excersise5,
    Excersise5Learning,
    Excersise5Practice,
    Excersise6,
    Excersise6Learning,
    Excersise6Practice,
    Excersise7,
    Excersise7Learning,
    Excersise7Practice,
    Excersise12,
    Excersise12Learning,
    Excersise12Practice,
    Excersise15,
    Excersise15Learning,
    Excersise15Practice,
    Excersise10Practice,
    Excersise10Learning,
    Excersise10,
    Excersise2Practice,
    Excersise2Learning,
    Excersise2,
    Excersise14Practice,
    Excersise14Learning,
    Excersise14,
}

#[derive(Debug, Clone)]
enum Message {
    OpenExcersiseList,
    SelectedExcersise(u8),
    SelectedSubExcersise(u8, ExersiseData),
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
    /// excersise14answer serialized to toml
    Excersise14CheckAnswer,
    SelectedLearningExcersise(i32),
    CheckExcersise2
}

impl Sandbox for App {
    type Message = Message;

    fn new() -> Self {
        Self { login: "".into(), state: AppState::Login, exersise_data: None, save: SaveData::default() }
    }

    fn title(&self) -> String {
        "Тренажёр для ОГЭ по информатике".into()
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
            Message::SelectedSubExcersise(excersise_number, excersise_data) => {
                self.exersise_data = Some(excersise_data.clone());
                match excersise_number {
                    1 => {
                        self.state = AppState::Excersise1Practice;
                    },
                    2 => {
                        self.state = AppState::Excersise2Practice;
                    },
                    3 => {
                        self.state = AppState::Excersise3Practice;
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
                    10 => {
                        self.state = AppState::Excersise10Practice;
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
                if let Some(excersise_data) = &mut self.exersise_data {
                    excersise_data.input_field_text = text.into();
                }
            }
            Message::CheckAnswer => {
                if let Some(excersise_data) = &mut self.exersise_data {
                    if excersise_data.input_field_text == excersise_data.right_answer {
                        excersise_data.state = ExcersiseState::RightAnswer;
                    } else {
                        excersise_data.state = ExcersiseState::WrongAnswer;
                    }
                }
            },
            Message::SetState(state) => {
                if let Some(excersise_data) = &mut self.exersise_data {
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
                        self.exersise_data.as_mut().unwrap().state = ExcersiseState::RightAnswer;
                    } else {
                        self.exersise_data.as_mut().unwrap().state = ExcersiseState::WrongAnswer;
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
                                    _ => return exersise_data.state = ExcersiseState::WrongAnswer
                                }
                                let avg_number: f32 = format!("{:.2}", avg_number).parse().unwrap();

                                if &avg_number != &answer.avg_score {
                                    return exersise_data.state = ExcersiseState::WrongAnswer;
                                }
                            },
                            None => return exersise_data.state = ExcersiseState::WrongAnswer,
                        }
                        let four_or_five_value: Option<&Data> = worksheet.get_value((2, 7));
                        match four_or_five_value {
                            Some(four_or_five_value) => {
                                let four_or_five;
                                match four_or_five_value {
                                    Data::Int(value) => four_or_five = *value as i32,
                                    Data::Float(value) => four_or_five = (value.round()).clone() as i32,
                                    _ => return exersise_data.state = ExcersiseState::WrongAnswer
                                }

                                if &four_or_five != &answer.four_or_five {
                                    return exersise_data.state = ExcersiseState::WrongAnswer;
                                }
                            },
                            None => return exersise_data.state = ExcersiseState::WrongAnswer,
                        }
                    }
                    exersise_data.state = ExcersiseState::RightAnswer;
                }
            },
            Message::SelectedLearningExcersise(excersise_number) => {
                match excersise_number {
                    1 => self.state = AppState::Excersise1Learning,
                    2 => self.state = AppState::Excersise2Learning,
                    3 => self.state = AppState::Excersise3Learning,
                    5 => self.state = AppState::Excersise5Learning,
                    6 => self.state = AppState::Excersise6Learning,
                    7 => self.state = AppState::Excersise7Learning,
                    10 => self.state = AppState::Excersise10Learning,
                    12 => self.state = AppState::Excersise12Learning,
                    14 => self.state = AppState::Excersise14Learning,
                    15 => self.state = AppState::Excersise15Learning,
                    _ => todo!()
                }
            },
            Message::CheckExcersise2 => {
                if let Some(excersise_data) = &mut self.exersise_data {
                    let mut number_str = String::new();
                    let chars = excersise_data.input_field_text.chars();
                    chars.for_each(|char| {
                        match char {
                            'А' => number_str += "10",
                            'Б' => number_str += "110",
                            'В' => number_str += "12",
                            'Г' => number_str += "102",
                            _ => ()
                        }
                    });

                    if number_str == excersise_data.right_answer {
                        excersise_data.state = ExcersiseState::RightAnswer;
                    } else {
                        excersise_data.state = ExcersiseState::WrongAnswer;
                    }
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
            AppState::Excersise1Practice => Excersise1::practice_view(self.exersise_data.clone()),
            AppState::Excersise2 => Excersise2::select_subexcersise_view(save.total_done_excersise2, save.done_correctly_excersise2),
            AppState::Excersise2Learning => Excersise2::learning_view(),
            AppState::Excersise2Practice => Excersise2::practice_view(self.exersise_data.clone()),
            AppState::Excersise3 => Excersise3::select_subexcersise_view(save.total_done_excersise3, save.done_correctly_excersise3),
            AppState::Excersise3Learning => Excersise3::learning_view(),
            AppState::Excersise3Practice => Excersise3::practice_view(self.exersise_data.clone()),
            AppState::Excersise5 => Excersise5::select_subexcersise_view(save.total_done_excersise5, save.done_correctly_excersise5),
            AppState::Excersise5Learning => Excersise5::learning_view(),
            AppState::Excersise5Practice => Excersise5::practice_view(self.exersise_data.clone()),
            AppState::Excersise6 => Excersise6::select_subexcersise_view(save.total_done_excersise6, save.done_correctly_excersise6),
            AppState::Excersise6Learning => Excersise6::learning_view(),
            AppState::Excersise6Practice => Excersise6::practice_view(self.exersise_data.clone()),
            AppState::Excersise7 => Excersise7::select_subexcersise_view(save.total_done_excersise7, save.done_correctly_excersise7),
            AppState::Excersise7Learning => Excersise7::learning_view(),
            AppState::Excersise7Practice => Excersise7::practice_view(self.exersise_data.clone()),
            AppState::Excersise12 => Excersise12::select_subexcersise_view(save.total_done_excersise12, save.done_correctly_excersise12),
            AppState::Excersise12Learning => Excersise12::learning_view(),
            AppState::Excersise12Practice => Excersise12::practice_view(self.exersise_data.clone()),
            AppState::Excersise14 => Excersise14::select_subexcersise_view(save.total_done_excersise12, save.done_correctly_excersise12),
            AppState::Excersise14Learning => Excersise14::learning_view(),
            AppState::Excersise14Practice => Excersise14::practice_view(self.exersise_data.clone()),
            AppState::Excersise15 => Excersise15::select_subexcersise_view(save.total_done_excersise15, save.done_correctly_excersise15),
            AppState::Excersise15Learning => Excersise15::learning_view(),
            AppState::Excersise15Practice => Excersise15::practice_view(self.exersise_data.clone()),
            AppState::Excersise10=> Excersise10::select_subexcersise_view(save.total_done_excersise10, save.done_correctly_excersise10),
            AppState::Excersise10Learning => Excersise10::learning_view(),
            AppState::Excersise10Practice => Excersise10::practice_view(self.exersise_data.clone()),
        }
    }
}


impl App {
    fn handle_selecting_excersise(&mut self, excersise_number: u8) {
        match excersise_number {
            1 => self.state = AppState::Excersise1,
            2 => self.state = AppState::Excersise2,
            3 => self.state = AppState::Excersise3,
            5 => self.state = AppState::Excersise5,
            6 => self.state = AppState::Excersise6,
            7 => self.state = AppState::Excersise7,
            10 => self.state = AppState::Excersise10,
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
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(2)),
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
                        .height(Length::Fixed(80.0))
                        .on_press(Message::SelectedExcersise(14)),
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
pub struct ExersiseData {
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
