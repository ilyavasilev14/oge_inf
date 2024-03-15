use iced::widget::{scrollable, text, container, column, button, Image};
use rand::{Rng, distributions::{Distribution, Standard}};
use crate::{Message, ExcersiseData, ExcersiseState};
use super::Exercise;


pub struct Excersise6 { }

impl Exercise for Excersise6 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let text: iced::Element<'a, Message> = text(
"   Чтобы решить шестое задание, надо найти те пары чисел, которые будут подходить по условию, которое дано в задании.
    Допустим, что в задании дано такое условие: \"если s < 9 или t < 9\". Так как мы видим \"или\", то достаточно, чтобы было верным лишь одно из условий. Если бы в задании было \"и\", то обязательно должны быть правильными оба условия
    Например, если даны пары чисел (9, 9); (9, 10); (8, 5); (11, 6); (–11, 10); (–5, 9); (–10, 10); (4, 5); (8, 6), то условие будет подходить только к (8, 5); (11, 6); (-11, 10); (-5, 9); (-10, 10); (4, 5); (8, 6)
    Если в задании просят найти то, сколько раз программа вывела \"ДА\", то нам нужно посчитать количество пар, к которым данное условие подошло.")
            .size(48)
            .vertical_alignment(iced::alignment::Vertical::Center)
            .horizontal_alignment(iced::alignment::Horizontal::Center)
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

    fn generate_random_excersise() -> ExcersiseData {
        //let ex_type = rand::thread_rng().gen_range(0..4);
        let ex_type = rand::thread_rng().gen_range(1..=2);
        match ex_type {
            1 => generate_excersise_type1(),
            _ => generate_excersise_type2(),
        }
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(6, 1, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(6)
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(6)
    }

    fn text_size() -> u16 {
        24
    }

    fn excersise_number() -> u8 {
        6
    }
}



fn generate_excersise_type1() -> ExcersiseData {
    let random_variant: ExcersiseVariations = rand::random(); 
    let s: i32;
    let t: i32;
    let s_str: String;
    let t_str: String;

    let mut yes_count = 0;
    let mut no_count = 0;

    let mut number_sets: Vec<(i32, i32)> = Vec::new();

    for _ in 0..9 {
        let num_1 = rand::thread_rng().gen_range(2..=25);
        let num_2 = rand::thread_rng().gen_range(2..=30);
        number_sets.push((num_1, num_2));
    }

    match random_variant {
        ExcersiseVariations::MoreLess => {
            s = rand::thread_rng().gen_range(2..=20); // >
            t = rand::thread_rng().gen_range(s + 5..40); // <
            s_str = format!("s > {}", s);
            t_str = format!("t < {}", t);

            for number_set in &number_sets {
                if number_set.0 > s || number_set.1 < t {
                    yes_count += 1;
                } else {
                    no_count += 1;
                }
            }
        },
        ExcersiseVariations::MoreeqLess => {
            s = rand::thread_rng().gen_range(2..=20); // >=
            t = rand::thread_rng().gen_range(s + 5..40); // <
            s_str = format!("s >= {}", s);
            t_str = format!("t < {}", t);

            for number_set in &number_sets {
                if number_set.0 >= s || number_set.1 < t {
                    yes_count += 1;
                } else {
                    no_count += 1;
                }
            }
        },
        ExcersiseVariations::MoreeqLesseq => {
            s = rand::thread_rng().gen_range(2..=20); // >=
            t = rand::thread_rng().gen_range(s + 5..40); // <=
            s_str = format!("s >= {}", s);
            t_str = format!("t <= {}", t);

            for number_set in &number_sets {
                if number_set.0 >= s || number_set.1 <= t {
                    yes_count += 1;
                } else {
                    no_count += 1;
                }
            }
        },
        ExcersiseVariations::LessMore => {
            s = rand::thread_rng().gen_range(8..=30); // <
            t = rand::thread_rng().gen_range(2..=s - 3); // >
            s_str = format!("s < {}", s);
            t_str = format!("t > {}", t);

            for number_set in &number_sets {
                if number_set.0 < s || number_set.1 > t {
                    yes_count += 1;
                } else {
                    no_count += 1;
                }
            }
        },
        ExcersiseVariations::LesseqMoreeq => {
            s = rand::thread_rng().gen_range(9..=20); // <=
            t = rand::thread_rng().gen_range(2..=s - 1); // >=
            s_str = format!("s <= {}", s);
            t_str = format!("t >= {}", t);

            for number_set in &number_sets {
                if number_set.0 <= s || number_set.1 >= t {
                    yes_count += 1;
                } else {
                    no_count += 1;
                }
            }
        },
    };

    let mut number_sets_list_string = String::new();
    number_sets.iter().for_each(|set| {
        let set_text = format!("({}, {});", set.0, set.1);
        number_sets_list_string += &set_text;
    });

    let program_text = format!("алг
нач
цел s, t
ввод s
ввод t
если {} или {}
    то вывод \"YES\"
    иначе вывод \"NO\"
все
кон", s_str, t_str); // YES count

    let requested_output: &str;
    let answer: i32;
    let is_yes: bool = rand::thread_rng().gen();
    if is_yes {
        requested_output = "\"YES\"";
        answer = yes_count;
    } else {
        requested_output = "\"NO\"";
        answer = no_count;
    }

    let full_title = format!(
"Ниже приведена программа, записанная на одном из языков программирования.
Алгоритмический язык:\n{}\n
Было проведено 9 запусков программы, при которых в качестве значений переменных s и t вводились следующие пары чисел:
{}
Сколько было запусков, при которых программа напечатала {}?", 
        program_text, number_sets_list_string, requested_output);
    return ExcersiseData {
        title: full_title,
        right_answer: answer.to_string(),
        input_field_text: String::new(),
        state: ExcersiseState::NotDone,
    };
} // generate_excersise_type2 is AND excersise(this function is OR)

fn generate_excersise_type2() -> ExcersiseData {
    let random_variant: ExcersiseVariations = rand::random(); 
    let s: i32;
    let t: i32;
    let s_str: String;
    let t_str: String;

    let mut yes_count = 0;
    let mut no_count = 0;

    let mut number_sets: Vec<(i32, i32)> = Vec::new();

    for _ in 0..9 {
        let num_1 = rand::thread_rng().gen_range(2..=25);
        let num_2 = rand::thread_rng().gen_range(2..=30);
        number_sets.push((num_1, num_2));
    }

    match random_variant {
        ExcersiseVariations::MoreLess => {
            s = rand::thread_rng().gen_range(2..=20); // >
            t = rand::thread_rng().gen_range(s + 5..40); // <
            s_str = format!("s > {}", s);
            t_str = format!("t < {}", t);

            for number_set in &number_sets {
                if number_set.0 > s && number_set.1 < t {
                    yes_count += 1;
                } else {
                    no_count += 1;
                }
            }
        },
        ExcersiseVariations::MoreeqLess => {
            s = rand::thread_rng().gen_range(2..=20); // >=
            t = rand::thread_rng().gen_range(s + 5..40); // <
            s_str = format!("s >= {}", s);
            t_str = format!("t < {}", t);

            for number_set in &number_sets {
                if number_set.0 >= s && number_set.1 < t {
                    yes_count += 1;
                } else {
                    no_count += 1;
                }
            }
        },
        ExcersiseVariations::MoreeqLesseq => {
            s = rand::thread_rng().gen_range(2..=20); // >=
            t = rand::thread_rng().gen_range(s + 5..40); // <=
            s_str = format!("s >= {}", s);
            t_str = format!("t <= {}", t);

            for number_set in &number_sets {
                if number_set.0 >= s && number_set.1 <= t {
                    yes_count += 1;
                } else {
                    no_count += 1;
                }
            }
        },
        ExcersiseVariations::LessMore => {
            s = rand::thread_rng().gen_range(8..=30); // <
            t = rand::thread_rng().gen_range(2..=s - 3); // >
            s_str = format!("s < {}", s);
            t_str = format!("t > {}", t);

            for number_set in &number_sets {
                if number_set.0 < s && number_set.1 > t {
                    yes_count += 1;
                } else {
                    no_count += 1;
                }
            }
        },
        ExcersiseVariations::LesseqMoreeq => {
            s = rand::thread_rng().gen_range(9..=20); // <=
            t = rand::thread_rng().gen_range(2..=s - 1); // >=
            s_str = format!("s <= {}", s);
            t_str = format!("t >= {}", t);

            for number_set in &number_sets {
                if number_set.0 <= s && number_set.1 >= t {
                    yes_count += 1;
                } else {
                    no_count += 1;
                }
            }
        },
    };

    let mut number_sets_list_string = String::new();
    number_sets.iter().for_each(|set| {
        let set_text = format!("({}, {});", set.0, set.1);
        number_sets_list_string += &set_text;
    });

    let program_text = format!("алг
нач
цел s, t
ввод s
ввод t
если {} и {}
    то вывод \"YES\"
    иначе вывод \"NO\"
все
кон", s_str, t_str); // YES count

    let requested_output: &str;
    let answer: i32;
    let is_yes: bool = rand::thread_rng().gen();
    if is_yes {
        requested_output = "\"YES\"";
        answer = yes_count;
    } else {
        requested_output = "\"NO\"";
        answer = no_count;
    }

    let full_title = format!(
"Ниже приведена программа, записанная на одном из языков программирования.
Алгоритмический язык:\n{}\n
Было проведено 9 запусков программы, при которых в качестве значений переменных s и t вводились следующие пары чисел:
{}
Сколько было запусков, при которых программа напечатала {}?", 
        program_text, number_sets_list_string, requested_output);

    return ExcersiseData {
        title: full_title,
        right_answer: answer.to_string(),
        input_field_text: String::new(),
        state: ExcersiseState::NotDone,
    };
}


enum ExcersiseVariations {
    MoreLess,
    MoreeqLess,
    MoreeqLesseq,
    LessMore,
    LesseqMoreeq,
}

impl Distribution<ExcersiseVariations> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> ExcersiseVariations {
        match rng.gen_range(0..5) {
            0 => ExcersiseVariations::MoreLess,
            1 => ExcersiseVariations::MoreeqLess,
            2 => ExcersiseVariations::MoreeqLesseq,
            3 => ExcersiseVariations::LessMore,
            _ => ExcersiseVariations::LesseqMoreeq,
        }
    }
}
