use iced::widget::text;
use rand::Rng;
use crate::{Message, ExcersiseData, ExcersiseState};
use super::Exercise;


pub struct Excersise5 { }

impl Exercise for Excersise5 {
    fn learning_view<'a>() -> iced::Element<'a, Message> {
        text("ad;slkfjfjasdlk;fjfj")
            .size(Self::text_size())
            .into()
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
        Message::SelectedSubExcersise(5, 1, Self::generate_random_excersise())
    }

    fn select_excersise() -> Message {
        Message::SelectedExcersise(5)
    }

    fn select_learning() -> Message {
        Message::SelectedLearningExcersise(5)
    }

    fn excersise_number() -> u8 {
        5
    }
}



// From < number to >
fn generate_excersise_type1() -> ExcersiseData {
    let base_number = rand::thread_rng().gen_range(1..=150);
    let plus_number = rand::thread_rng().gen_range(1..=25);
    let rounded_half_plus = ((plus_number / 2 + 1) as f32).round();
    let multiply_number = rand::thread_rng().gen_range(2..=(rounded_half_plus + 1.0) as i32);
    let plus_count = rand::thread_rng().gen_range(1..=4);
    let mut operations: [i32; 5] = [0, 0, 0, 0, 0];

    for i in 0..plus_count {
        operations[i] = 1;
        println!("i = {}", i);
    }
    operations[plus_count] = 2;
    for i in plus_count + 1..5 {
        operations[i] = 1;
    }


    let mut value = base_number;
    let mut program: String = String::new();
    for i in operations {
        if i == 1 {
            value += plus_number;
            program += "1";
        } else if i == 2 {
            value *= multiply_number;
            program += "2";
        }
    }

    dbg!(base_number, plus_number, multiply_number, plus_count + 1, value);

    let title = format!("У исполнителя Альфа две команды, которым присвоены номера:
1. прибавь {};
2. умножь на b
(b - неизвестное натуральное число; b ≥ 2).

Выполняя первую из них, Альфа увеличивает число на экране на {}, а выполняя вторую, умножает это число на b. Программа для исполнителя Альфа - это последовательность номеров команд. Известно, что программа {} переводит число {} в число {}. Определите значение b.", plus_number, plus_number, program, base_number, value);

    ExcersiseData {
        title,
        right_answer: multiply_number.to_string(),
        input_field_text: "".into(),
        state: ExcersiseState::NotDone,
    }
}

// From > number to <
fn generate_excersise_type2() -> ExcersiseData {
    let mut base_number = rand::thread_rng().gen_range(100..=250);
    if base_number % 2 != 0 {
        base_number = rand::thread_rng().gen_range(100..=250);
    }
    let mut minus_number = rand::thread_rng().gen_range(1..=(base_number as f32 / 8 as f32).round() as i32);
    if minus_number % 2 != 0 {
        minus_number = rand::thread_rng().gen_range(1..=(base_number as f32 / 8 as f32).round() as i32);
    }

    let mut division_number = rand::thread_rng().gen_range(2..=20);
    for _i in 0..50 { 
        if base_number % division_number != 0 { 
            division_number = rand::thread_rng().gen_range(2..=base_number / 25);
            break;
        }
    }

    if base_number % division_number != 0 {  
        return generate_excersise_type2();
    }

    let minus_count = rand::thread_rng().gen_range(1..=4);
    let mut operations: [i32; 5] = [0, 0, 0, 0, 0];

    for i in 0..minus_count {
        operations[i] = 1;
        println!("i = {}", i);
    }
    operations[minus_count] = 2;
    for i in minus_count + 1..5 {
        operations[i] = 1;
    }


    let mut value = base_number;
    let mut program: String = String::new();
    for i in operations {
        if i == 1 {
            value -= minus_number;
            program += "1";
        } else if i == 2 {
            value /= division_number;
            program += "2";
        }
    }

    if value <= 0 {
        return generate_excersise_type2();
    }

    dbg!(base_number, minus_number, division_number, minus_count + 1, value);

    let title = format!("У исполнителя Альфа две команды, которым присвоены номера:
1. вычти {};
2. раздели на b
(b - неизвестное натуральное число; b ≥ 2).

Выполняя первую из них, Альфа уменьшает число на экране на {}, а выполняя вторую, делит это число на b. Программа для исполнителя Альфа - это последовательность номеров команд. Известно, что программа {} переводит число {} в число {}. Определите значение b.", minus_number, minus_number, program, base_number, value);

    ExcersiseData {
        title,
        right_answer: division_number.to_string(),
        input_field_text: "".into(),
        state: ExcersiseState::NotDone,
    }
}

