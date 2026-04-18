use iced::widget::{column, container, text, Space};
use iced::{Color, Element, Length, Theme};

pub fn main() -> iced::Result {
    // В 0.14.0 iced::run НЕ принимает строку первой!
    // Оплодотворяем систему через правильную сигнатуру
    iced::run(update, view)
}

// Наш общак (State)
#[derive(Default)]
struct VanaheimApp {
    // Тут будет твоя база данных из макета
}

#[derive(Debug, Clone, Copy)]
enum Message {
    // Малявы на будущее
}

// В 0.14 update принимает Стейт и Сообщение
fn update(_state: &mut VanaheimApp, _message: Message) {
    // Пока балду пинаем
}

// В 0.14 view принимает ТОЛЬКО Стейт
fn view(_state: &VanaheimApp) -> Element<Message> {
    // Твой элитный синий #191B28
    let main_bg = Color::from_rgb(
        25.0 / 255.0,
        27.0 / 255.0,
        40.0 / 255.0,
    );

    // Строим контент по твоим векторным замерам
    let content = column![
        // Space::new() теперь пустой, размеры задаем через методы!
        Space::new().height(Length::Fixed(300.0)), 
        text("Vanaheim\nNational Park")
            .size(16)
            .color(Color::WHITE),
        Space::new().height(Length::Fixed(10.0)),
        text("Lorem ipsum dolor sit amet, consectetur adipiscing elit.")
            .size(7)
            .color(Color::from_rgb(0.7, 0.7, 0.7)),
    ]
    .padding(25);

    // Забиваем фон «в масле»
    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_theme: &Theme| {
            container::Style {
                background: Some(main_bg.into()),
                text_color: Some(Color::WHITE),
                ..Default::default()
            }
        })
        .into()
}
