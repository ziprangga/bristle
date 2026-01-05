use iced::widget::{Column, Container, Text, container, scrollable};
use iced::{Color, Element, Length, Padding, Theme, alignment};
use std::sync::Arc;

const DEFAULT_PADDING: Padding = Padding {
    top: 5.0,
    bottom: 5.0,
    right: 5.0,
    left: 5.0,
};

pub enum RowContent<M> {
    Text(TextContent),
    Widget(WidgetContent<M>),
}

pub enum HeaderContent<M> {
    Text(TextContent),
    Widget(HeaderWidget<M>),
}

pub struct TextContent {
    label: String,
    text_align_x: alignment::Horizontal,
    text_align_y: alignment::Vertical,
    text_size: u32,
    text_color: Option<Color>,
}

impl TextContent {
    pub fn new(label: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            text_align_x: alignment::Horizontal::Center,
            text_align_y: alignment::Vertical::Center,
            text_size: 12,
            text_color: None,
        }
    }
}

pub struct WidgetContent<M> {
    element: Box<dyn Fn(bool) -> Element<'static, M>>,
}

impl<M: 'static> WidgetContent<M> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(bool) -> Element<'static, M> + 'static,
    {
        Self {
            element: Box::new(f),
        }
    }
}

pub struct HeaderWidget<M> {
    element: Box<dyn Fn(bool) -> Element<'static, M>>,
}

impl<M: 'static> HeaderWidget<M> {
    pub fn new<F>(f: F) -> Self
    where
        F: Fn(bool) -> Element<'static, M> + 'static,
    {
        Self {
            element: Box::new(f),
        }
    }
}

type WidgetStyleFn = dyn Fn(usize, &Theme) -> container::Style;

type ColumnStyleFn = dyn Fn(&Theme) -> container::Style;

pub struct ListView<M> {
    items: Vec<RowContent<M>>,
    headers: Vec<HeaderContent<M>>,
    row_selected: Option<usize>,
    header_selected: Option<usize>,
    spacing: u32,
    width: Length,
    height: Option<Length>,
    padding: Padding,
    row_style: Option<Arc<WidgetStyleFn>>,
    column_style: Option<Arc<ColumnStyleFn>>,
}

impl<M: 'static + Clone> ListView<M> {
    pub fn new(items: Vec<RowContent<M>>) -> Self {
        Self {
            items,
            headers: Vec::new(),
            row_selected: None,
            header_selected: None,
            spacing: 0,
            width: Length::Fill,
            height: None,
            padding: DEFAULT_PADDING,
            row_style: None,
            column_style: None,
        }
    }

    // =============widget====================
    pub fn row_style<F>(mut self, f: F) -> Self
    where
        F: Fn(usize, &Theme) -> container::Style + 'static,
    {
        self.row_style = Some(Arc::new(f));
        self
    }

    // =============header====================
    pub fn header(mut self, header: impl Into<HeaderContent<M>>) -> Self {
        self.headers.push(header.into());
        self
    }

    pub fn headers<I>(mut self, headers: I) -> Self
    where
        I: IntoIterator<Item = HeaderContent<M>>,
    {
        self.headers.extend(headers);
        self
    }

    #[inline]
    pub fn header_selected(mut self, index: Option<usize>) -> Self {
        self.header_selected = index;
        self
    }

    #[inline]
    pub fn is_header_selected(&self, index: usize) -> bool {
        self.header_selected == Some(index)
    }

    // =============Column====================
    #[inline]
    pub fn row_selected(mut self, index: Option<usize>) -> Self {
        self.row_selected = index;
        self
    }

    #[inline]
    pub fn is_row_selected(&self, index: usize) -> bool {
        self.row_selected == Some(index)
    }

    pub fn spacing(mut self, spacing: u32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn width(mut self, width: Length) -> Self {
        self.width = width;
        self
    }

    pub fn height(mut self, height: Length) -> Self {
        self.height = Some(height);
        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    pub fn column_style<F>(mut self, f: F) -> Self
    where
        F: Fn(&Theme) -> container::Style + 'static,
    {
        self.column_style = Some(Arc::new(f));
        self
    }

    pub fn view(self) -> Element<'static, M> {
        let mut scroll_col = Column::new().spacing(self.spacing);

        for (i, row) in self.items.iter().enumerate() {
            let selected = self.is_row_selected(i);

            let row_element: Element<'static, M> = match row {
                RowContent::Text(t) => {
                    let mut txt = Text::new(t.label.clone())
                        .size(t.text_size)
                        .align_x(t.text_align_x)
                        .align_y(t.text_align_y)
                        .width(Length::Fill);

                    if let Some(color) = t.text_color {
                        txt = txt.color(color);
                    }
                    txt.into()
                }
                RowContent::Widget(w) => (w.element)(selected),
            };

            let mut row_container = Container::new(row_element).padding(self.padding);

            if let Some(style_fn) = self.row_style.clone() {
                row_container = row_container.style(move |theme| style_fn(i, theme));
            }

            scroll_col = scroll_col.push(row_container);
        }

        let mut content_col = Column::new();

        for (col, header) in self.headers.iter().enumerate() {
            let selected = self.is_header_selected(col);

            let header_element: Element<'static, M> = match header {
                HeaderContent::Text(t) => {
                    let mut txt = Text::new(t.label.clone())
                        .size(t.text_size)
                        .align_x(t.text_align_x)
                        .align_y(t.text_align_y)
                        .width(Length::Fill);

                    if let Some(color) = t.text_color {
                        txt = txt.color(color);
                    }

                    txt.into()
                }
                HeaderContent::Widget(w) => (w.element)(selected),
            };

            content_col = content_col.push(Container::new(header_element).padding(self.padding));
        }

        content_col = content_col.push(scrollable(scroll_col));

        let mut parent_col = Container::new(content_col)
            .width(self.width)
            .padding(self.padding);

        if let Some(h) = self.height {
            parent_col = parent_col.height(h);
        }

        if let Some(style_fn) = self.column_style.clone() {
            parent_col = parent_col.style(move |theme| style_fn(theme));
        }

        parent_col.into()
    }
}
