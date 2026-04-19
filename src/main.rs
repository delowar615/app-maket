use iced::widget::{column, container, text, Space, stack, Image, row};
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
    // На будущее, пока балду пинаем
}

fn update(_state: &mut VanaheimApp, _message: Message) {}

fn view(_state: &VanaheimApp) -> Element<'_, Message, Theme, iced::Renderer> {
    let main_bg = Color::from_rgb(25.0 / 255.0, 27.0 / 255.0, 40.0 / 255.0);

    // 1. Оплодотворяем Инфо-панель (тот самый row, который "не найден")
    let info_panel = row![
        text("⋮").size(12),
        Space::new().width(Length::Fixed(10.0)),
        text("♡").size(12),
        Space::new().width(Length::Fixed(10.0)),
        text("🚀").size(10),
        Space::new().width(Length::Fill),
        text("⭐").size(10).color(Color::from_rgb(1.0, 0.6, 0.0)),
        text(" 4.5/5.0").size(8),
    ]
    .align_y(iced::Alignment::Center);

    // 2. Оплодотворяем Navbar (который тоже "не найден")
    let navbar = row![
        text("🏠").size(20),
        Space::new().width(Length::Fill),
        text("🔍").size(20),
        Space::new().width(Length::Fill),
        text("🤍").size(20),
        Space::new().width(Length::Fill),
        text("👤").size(20),
    ]
    .width(Length::Fill)
    .align_y(iced::Alignment::Center);

    // 3. ФИНАЛЬНЫЙ СТЭК (Картинка ПОД текстом, как в Иллюстраторе)
    let main_stack = stack![
        // Слой 0: Горы на весь ствол
    Image::<iced::widget::image::Handle>::new(
            iced::widget::image::Handle::from_path("assets/icons/rock.png")
        )
        .width(Length::Fill)
        .height(Length::Fill) // Растянули до самого Navbar!
        .content_fit(iced::ContentFit::Cover),

        // Слой 1: УСИЛЕННЫЙ ГРАДИЕНТ (чтобы спрятать стык)
        container(Space::new())
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(iced::Gradient::Linear(
                    iced::gradient::Linear::new(std::f32::consts::FRAC_PI_2 * 4.0)
                    .add_stop(0.0, main_bg) // Внизу плотно
                    .add_stop(0.3, main_bg) // До середины держим синеву для текста
                    .add_stop(1.0, Color::TRANSPARENT) // К верху плавно открываем горы
                ).into()),
                ..Default::default()
            }),

        // Слой 2: Твой текст и иконки ПОВЕРХ всего этого созидания
        column![
            Space::new().height(Length::Fill), // Пружина выдавливает всё вниз
            text("Vanaheim\nNational Park")
                .size(18)
                .line_height(1.5)
                .font(Font { weight: Weight::Bold, ..Font::DEFAULT })
                .color(Color::WHITE),
            text("Lorem ipsum dolor sit amet, consectetuer adipiscing.")
                .size(7)
                .font(Font { weight: Weight::Medium, ..Font::DEFAULT })
                .color(Color::from_rgb(0.7, 0.7, 0.7)),
            
            Space::new().height(Length::Fixed(15.0)),
            info_panel, // Теперь он в скоупе!
            
            Space::new().height(Length::Fixed(20.0)),
            text("INFORMATIONS")
                .size(9)
                .font(Font { weight: Weight::Bold, ..Font::DEFAULT })
                .color(Color::WHITE),
            Space::new().height(Length::Fixed(5.0)),
            text("Lorem ipsum dolor sit amet, consectetuer adipiscing elit, sed diam nonummy nibh euismod tincidunt ut laoreet dolore magna aliquam erat volutpat. Ut wisi enim ad minim veniam, quis nostrud exerci tation ullamcorper suscipit lobortis nisl ut aliquip ex ea commodo consequat. Duis autem vel eum iriure dolor in hendrerit in vulputate.
")
                .size(7)
                .color(Color::from_rgb(0.5, 0.5, 0.5)),
            
            Space::new().height(Length::Fixed(40.0)),
            navbar, // Теперь он в скоупе!
        ]
        .padding(25)
    ];

    container(main_stack)
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
}

