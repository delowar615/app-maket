use iced::widget::image; 
use iced::widget::{column, container, text, Space, stack, Image, row, button};
use iced::{Color, Element, Length, Theme, Font, font::Weight};

pub fn main() -> iced::Result {
    // Оплодотворяем запуск: инициализация (||), логика, вид
    iced::application(|| VanaheimApp::default(), update, view)
        .title("Vanaheim National Park - Elite")
        .window(iced::window::Settings {
            size: [360.0, 740.0].into(),
            ..Default::default()
        })
        .run()
}

#[derive(Default)]
struct VanaheimApp;

#[derive(Debug, Clone, Copy)]
enum Message {
    ExplorePressed,
}

fn update(_state: &mut VanaheimApp, _message: Message) {}

fn view(_state: &VanaheimApp) -> Element<'_, Message, Theme, iced::Renderer> {
    // Твой элитный синий #191B28
    let main_bg = Color::from_rgb(25.0 / 255.0, 27.0 / 255.0, 40.0 / 255.0);
    let pink_accent = Color::from_rgb(235.0 / 255.0, 6.0 / 255.0, 255.0 / 255.0);

    // 1. Navbar (используем только .width() и .height())
    let navbar = row![
        Image::new(image::Handle::from_path("assets/icons/search.svg")).width(20),
        Space::new().width(Length::Fixed(15.0)),
        Image::new(image::Handle::from_path("assets/icons/bookmark.svg")).width(20),
        Space::new().width(Length::Fixed(15.0)),
        Image::new(image::Handle::from_path("assets/icons/home.svg")).width(24),
        Space::new().width(Length::Fixed(15.0)),
        Image::new(image::Handle::from_path("assets/icons/user.svg")).width(20),
        Space::new().width(Length::Fixed(15.0)),
        Image::new(image::Handle::from_path("assets/icons/menu.svg")).width(20),
    ]
    .width(Length::Fill)
    .padding(20)
    .align_y(iced::Alignment::Center);

    // 2. Инфо-панель
    let info_panel = row![
        Image::new(image::Handle::from_path("assets/icons/dots.svg")).width(14),
        Space::new().width(Length::Fixed(15.0)),
        Image::new(image::Handle::from_path("assets/icons/heart.svg")).width(14),
        Space::new().width(Length::Fill), // Если тут ошибка - замени на Space::new().width(...)
        Image::new(image::Handle::from_path("assets/icons/rocket.png")).width(14),
        Space::new().width(Length::Fill),
        text("⭐ 4.5/5.0").size(12).color(pink_accent),
    ].align_y(iced::Alignment::Center);

    // 3. Контент
    let content = column![
        Space::new().height(Length::Fill),
        text("Vanaheim\nNational Park")
            .size(32).line_height(1.2).font(Font::with_name("Vanaheim")).color(Color::WHITE),
        text("Lorem ipsum dolor sit amet, consectetuer adipiscing.")
            .size(12).color(Color::from_rgb(0.6, 0.6, 0.7)),
        Space::new().height(Length::Fixed(15.0)),
        info_panel,
        Space::new().height(Length::Fixed(25.0)),
        text("INFORMATIONS").size(14).font(Font { weight: Weight::Bold, ..Font::DEFAULT }),
        Space::new().height(Length::Fixed(8.0)),
        text("Lorem ipsum dolor sit amet, consectetuer adipiscing elit...").size(11).color(Color::from_rgb(0.5, 0.5, 0.6)),
        button(text("Read More").size(11).color(pink_accent).font(Font { weight: Weight::Bold, ..Font::DEFAULT }))
            .padding(0).on_press(Message::ExplorePressed)
            .style(|_, _| button::Style { background: Some(Color::TRANSPARENT.into()), ..Default::default() }),
        Space::new().height(Length::Fixed(30.0)),
        navbar,
    ].padding(25);

    // 4. Стэк
    stack![
        Image::new(image::Handle::from_path("assets/images/rock.png"))
            .width(Length::Fill).height(Length::Fill).content_fit(iced::ContentFit::Cover),
        container(Space::new()).width(Length::Fill).height(Length::Fill).style(move |_| container::Style {
            background: Some(iced::Gradient::Linear(
                iced::gradient::Linear::new(std::f32::consts::PI)
                    .add_stop(0.0, main_bg).add_stop(0.6, Color::TRANSPARENT)
            ).into()), ..Default::default()
        }),
        content,
    ].into()
}

