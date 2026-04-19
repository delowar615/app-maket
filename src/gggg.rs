text("Vanaheim\nNational Park")
            .size(16)
            .line_height(1.5)
            .font(Font {
                weight: Weight::Bold,
                ..Font::DEFAULT // Используем их константу для остальной шляпы
            })
            .color(Color::WHITE),
        Space::new().height(Length::Fixed(10.0)),
        text("Lorem ipsum dolor sit amet, consectetuer adipiscing.")
            .size(7)
            .font(Font {
                weight: Weight::Medium,
                ..Font::DEFAULT // Используем их константу для остальной шляпы
            })
            .color(Color::from_rgb(0.7, 0.7, 0.7)),