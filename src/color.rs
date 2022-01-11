#[derive(Copy, Clone, Debug, Default)]
pub(crate) struct Palette {
    pub(crate) key: styled::Style,
    pub(crate) value: styled::Style,
}

impl Palette {
    #[cfg(feature = "color")]
    pub(crate) fn current() -> Self {
        if concolor::get(concolor::Stream::Either).ansi_color() {
            Self {
                key: styled::Style(yansi::Style::new(yansi::Color::Blue).bold()),
                value: styled::Style(yansi::Style::new(yansi::Color::Yellow).bold()),
            }
        } else {
            Self::default()
        }
    }

    #[cfg(not(feature = "color"))]
    pub(crate) fn current() -> Self {
        Self::default()
    }
}

#[cfg(feature = "color")]
mod styled {
    #[derive(Copy, Clone, Debug, Default)]
    pub(crate) struct Style(pub(crate) yansi::Style);

    impl Style {
        pub(crate) fn paint<T: std::fmt::Display>(self, item: T) -> impl std::fmt::Display {
            self.0.paint(item)
        }
    }
}

#[cfg(not(feature = "color"))]
mod styled {
    #[derive(Copy, Clone, Debug, Default)]
    pub(crate) struct Style;

    impl Style {
        pub(crate) fn paint<T: std::fmt::Display>(self, item: T) -> impl std::fmt::Display {
            item
        }
    }
}
