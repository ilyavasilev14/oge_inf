use iced::widget::{button, column, container, scrollable, text, Image};
use rand::{Rng, distributions::{Distribution, Standard}};
use crate::{Message, ExerciseData, ExcerciseState};
use super::Exercise;


pub struct Excersise3 { }

impl Exercise for Excersise3 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        let text: iced::Element<'a, Message> = text(
"Чтобы решить третье задание, надо преобразовать каждую из частей условия по отдельности.
Например, в задании дано такое условие: \"НЕ (x чётное) И (x > 10)\". 
Первая часть, которую мы рассмотрим, - это \"НЕ (x чётное)\", так как перед ним стоит \"НЕ\", то надо подставить вместо этой части то, что будет равнозначно по значению, но без \"НЕ\". Заменим эту часть на \"(x нечётное)\"
Так как во второй части(\"(x > 10)\") отсутствует \"НЕ\", ничего менять не надо
Подставим всё, что получилось - (x нечётное) И (x > 10). Допустим, что нам надо найти минимальное число, которое соответствует этому условию. Правильным ответом будет являться 11, так как 11 > 10 и 11 - нечётное число. Важно, чтобы обе части условия подходили, так как стоит \"И\"

Приведём второй пример: \"(x > 4) ИЛИ НЕ (x > 3)\"
Преобразуем его: \"(x > 4) ИЛИ (x <= 3)\"
Найдём то число, при котором это высказывание ЛОЖНО. Если в выражении используется \"ИЛИ\", то это значит, что число подходит тогда, когда хотя бы одно из условий является верным. Тут подходит число 4, так как оно не больше 4 и не больше или рано 3.")
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
        //let ex_type = rand::thread_rng().gen_range(0..4);
        let ex_type = rand::thread_rng().gen_range(1..=2);
        match ex_type {
            1 => generate_excersise_type1(),
            _ => generate_excersise_type2(),
        }
    }

    fn select_subexcersise() -> Message {
        Message::SelectedSubExcersise(3, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(3)
    }

    fn exercise_number() -> u8 {
        3
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(3)
    }
}



// Answer: exact number
fn generate_excersise_type1() -> ExerciseData {
    let or_excersise: bool = rand::random(); // Упражнение с ИЛИ?
    let number_in_not = rand::thread_rng().gen_range(0..=150); // Число, перед которым стоит НЕ
    let number_in_normal = number_in_not + 1; // Число, перед которым нет НЕ

    let first_is_not: bool = rand::random(); // Стоит ли число с НЕ первым? 
    let title;

    if or_excersise {
        if first_is_not {
            title =
                format!("Напишите целое число X, для которого ложно высказывание:\nНЕ (X > {}) ИЛИ (X > {})", number_in_not, number_in_normal);
        } else {
            title =
                format!("Напишите целое число X, для которого ложно высказывание:\n(X > {}) ИЛИ НЕ (X > {})", number_in_normal, number_in_not);
        }

        let data = ExerciseData {
            title,
            right_answer: number_in_normal.to_string(),
            input_field_text: "".into(),
            state: ExcerciseState::NotDone,
            additional_data: Vec::new(),
        };

        data
    } else {
        if first_is_not {
            title =
                format!("Напишите целое число X, для которого истинно высказывание:\nНЕ (X < {}) И (X < {})", number_in_not, number_in_normal);
        } else {
            title =
                format!("Напишите целое число X, для которого истинно высказывание:\n(X < {}) И НЕ (X < {})", number_in_normal, number_in_not);
        }

        let data = ExerciseData {
            title,
            right_answer: number_in_not.to_string(),
            input_field_text: "".into(),
            state: ExcerciseState::NotDone,
            additional_data: Vec::new(),
        };

        data
    }
}

// Answer: max value
fn generate_excersise_type2() -> ExerciseData {
    let random_variant: Type2Variations = rand::random(); 
    let right_answer;
    let title: String;
    let max = rand::thread_rng().gen_range(4..=150);

    match random_variant {
        Type2Variations::EvenLess => {
            if max % 2 == 0 {
                right_answer = max - 2;
            } else {
                right_answer = max - 1;
            }

            title = 
                format!("Напишите наибольшее целое число x, для которого истинно высказывание:\n(X четное) И (X < {}).", max);
        },
        Type2Variations::EvenLessequals => {
            if max % 2 == 0 {
                right_answer = max;
            } else {
                right_answer = max - 1;
            }

            title = 
                format!("Напишите наибольшее целое число x, для которого истинно высказывание:\n(X четное) И (X <= {}).", max);
        },
        Type2Variations::NotevenLess => {
            if max % 2 == 0 {
                right_answer = max - 1;
            } else {
                right_answer = max - 2;
            }

            title = 
                format!("Напишите наибольшее целое число x, для которого истинно высказывание:\nНЕ (X четное) И (X < {}).", max);
        },
        Type2Variations::NotevenLessequals => {
            if max % 2 == 0 {
                right_answer = max - 1;
            } else {
                right_answer = max;
            }

            title = 
                format!("Напишите наибольшее целое число x, для которого истинно высказывание:\nНЕ (X четное) И (X <= {}).", max);
        },
    }

    let data = ExerciseData {
        title,
        right_answer: right_answer.to_string(),
        input_field_text: "".into(),
        state: ExcerciseState::NotDone,
        additional_data: Vec::new(),
    };

    return data;
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

