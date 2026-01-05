use iced::{
    Border, Color, Element, Length, Padding, Theme, alignment,
    widget::{Container, Text, container},
};

const DEFAULT_PADDING: Padding = Padding {
    top: 5.0,
    bottom: 5.0,
    right: 5.0,
    left: 5.0,
};

enum DropContent<M> {
    Text(DropText),
    Image(DropImage),
    Widget(DropWidget<M>),
}

struct DropText {
    label: String,
    text_align_x: alignment::Horizontal,
    text_align_y: alignment::Vertical,
    text_size: u32,
    text_color: Option<Color>,
}

impl DropText {
    fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            text_size: 14,
            text_color: None,
            text_align_x: alignment::Horizontal::Center,
            text_align_y: alignment::Vertical::Center,
        }
    }
}

struct DropImage {
    image: iced::widget::Image,
    img_width: Length,
    img_height: Length,
}

impl DropImage {
    fn new(image: iced::widget::Image) -> Self {
        Self {
            image,
            img_width: Length::Fill,
            img_height: Length::Fill,
        }
    }
}

struct DropWidget<M> {
    element: Box<dyn Fn() -> Element<'static, M>>,
}

impl<M: 'static> DropWidget<M> {
    fn new<F>(f: F) -> Self
    where
        F: Fn() -> Element<'static, M> + 'static,
    {
        Self {
            element: Box::new(f),
        }
    }
}

type DropStyleFn = dyn Fn(&Theme) -> container::Style;

pub struct DropFile<M> {
    content: DropContent<M>,
    on_drop: Option<M>,
    width: Length,
    height: Option<Length>,
    align_x: alignment::Horizontal,
    align_y: alignment::Vertical,
    padding: Option<Padding>,
    style_fn: Option<Box<DropStyleFn>>,
}

impl<M: 'static + Clone> DropFile<M> {
    // This not item that drop to the widget.
    // this is just element decoration of the widget, like background
    // drop "new" will be show text in the widget before drop the item
    // drop image will be use image as element showing before item dropped just like text
    // drop widget eill be use what widget is pass as element ike button so
    // it can be use to do something when clicked

    pub fn new(label: impl Into<String>) -> Self {
        Self {
            content: DropContent::Text(DropText::new(label)),
            on_drop: None,
            width: Length::Fill,
            height: Some(Length::Fill),
            align_x: alignment::Horizontal::Center,
            align_y: alignment::Vertical::Center,
            padding: Some(DEFAULT_PADDING),
            style_fn: Some(Box::new(default_drop_style)),
        }
    }

    pub fn image(image: iced::widget::Image) -> Self {
        Self {
            content: DropContent::Image(DropImage::new(image)),
            on_drop: None,
            width: Length::Fill,
            height: Some(Length::Fill),
            align_x: alignment::Horizontal::Center,
            align_y: alignment::Vertical::Center,
            padding: None,
            style_fn: Some(Box::new(default_drop_style)),
        }
    }

    pub fn widget<F>(f: F) -> Self
    where
        F: Fn() -> Element<'static, M> + 'static,
    {
        Self {
            content: DropContent::Widget(DropWidget::new(f)),
            on_drop: None,
            width: Length::Fill,
            height: Some(Length::Fill),
            align_x: alignment::Horizontal::Center,
            align_y: alignment::Vertical::Center,
            padding: None,
            style_fn: Some(Box::new(default_drop_style)),
        }
    }

    // =================Text==============
    pub fn text_size(mut self, size: u32) -> Self {
        if let DropContent::Text(ref mut t) = self.content {
            t.text_size = size;
        }
        self
    }
    pub fn text_color(mut self, color: Color) -> Self {
        if let DropContent::Text(ref mut t) = self.content {
            t.text_color = Some(color);
        }
        self
    }

    pub fn text_align_x(mut self, align: alignment::Horizontal) -> Self {
        if let DropContent::Text(ref mut t) = self.content {
            t.text_align_x = align;
        }
        self
    }

    pub fn text_align_y(mut self, align: alignment::Vertical) -> Self {
        if let DropContent::Text(ref mut t) = self.content {
            t.text_align_y = align;
        }
        self
    }

    // ==============Image=================

    pub fn img_width(mut self, width: Length) -> Self {
        if let DropContent::Image(ref mut img) = self.content {
            img.img_width = width;
        }
        self
    }

    pub fn img_height(mut self, height: Length) -> Self {
        if let DropContent::Image(ref mut img) = self.content {
            img.img_height = height;
        }
        self
    }

    // =============widget====================

    pub fn on_drop(mut self, msg: M) -> Self {
        self.on_drop = Some(msg);
        self
    }

    pub fn width(mut self, w: Length) -> Self {
        self.width = w;
        self
    }

    pub fn height(mut self, h: Length) -> Self {
        self.height = Some(h);
        self
    }

    pub fn align_x(mut self, x: impl Into<alignment::Horizontal>) -> Self {
        self.align_x = x.into();
        self
    }

    pub fn align_y(mut self, y: impl Into<alignment::Vertical>) -> Self {
        self.align_y = y.into();
        self
    }

    pub fn padding(mut self, p: impl Into<Padding>) -> Self {
        self.padding = Some(p.into());
        self
    }

    pub fn style<F>(mut self, f: F) -> Self
    where
        F: Fn(&Theme) -> container::Style + 'static,
    {
        self.style_fn = Some(Box::new(f));
        self
    }

    // =====================viewer===============
    pub fn view(self) -> Element<'static, M> {
        let content: Element<'static, M> = match self.content {
            DropContent::Text(t) => {
                let mut txt = Text::new(t.label)
                    .size(t.text_size)
                    .align_x(t.text_align_x)
                    .align_y(t.text_align_y)
                    .width(self.width);

                if let Some(color) = t.text_color {
                    txt = txt.color(color);
                }

                txt.into()
            }

            DropContent::Image(img) => img.image.width(img.img_width).height(img.img_height).into(),

            DropContent::Widget(w) => (w.element)(),
        };

        let mut container = Container::new(content)
            .align_x(self.align_x)
            .align_y(self.align_y)
            .width(self.width);

        if let Some(h) = self.height {
            container = container.height(h);
        }

        if let Some(p) = self.padding {
            container = container.padding(p)
        }

        if let Some(style_fn) = self.style_fn {
            container = container.style(style_fn);
        }

        container.into()
    }
}

fn default_drop_style(_theme: &Theme) -> container::Style {
    container::Style {
        background: None,
        text_color: None,
        border: Border {
            color: Color {
                r: 1.0,
                g: 1.0,
                b: 1.0,
                a: 0.06,
            },
            width: 1.0,
            radius: 8.0.into(),
        },
        snap: false,
        shadow: Default::default(),
    }
}
