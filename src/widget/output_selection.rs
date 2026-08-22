use lingmo::iced::Limits;
use lingmo::iced::core::layout::Node;
use lingmo::iced::core::renderer::Quad;
use lingmo::iced::core::widget::Tree;
use lingmo::iced::core::widget::tree::{self, State};
use lingmo::iced::core::{Background, Border, Color, Length, Renderer, Shadow, Size, mouse};
use lingmo::widget::Widget;

pub struct OutputSelection<Msg> {
    on_enter: Msg,
    on_press: Msg,
}

impl<Msg> OutputSelection<Msg> {
    pub fn new(on_enter: Msg, on_press: Msg) -> Self {
        Self { on_enter, on_press }
    }
}

impl<Msg: Clone + 'static> Widget<Msg, lingmo::Theme, lingmo::Renderer> for OutputSelection<Msg> {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn state(&self) -> lingmo::iced::core::widget::tree::State {
        State::new(MyState::default())
    }

    fn tag(&self) -> lingmo::iced::core::widget::tree::Tag {
        tree::Tag::of::<MyState>()
    }

    fn layout(&mut self, _tree: &mut Tree, _renderer: &lingmo::Renderer, limits: &Limits) -> Node {
        let limits = limits.width(Length::Fill).height(Length::Fill);
        Node::new(limits.resolve(Length::Fill, Length::Fill, Size::ZERO))
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut lingmo::Renderer,
        theme: &lingmo::Theme,
        _style: &lingmo::iced::core::renderer::Style,
        layout: lingmo::iced::core::Layout<'_>,
        _cursor: lingmo::iced::core::mouse::Cursor,
        _viewport: &lingmo::iced::core::Rectangle,
    ) {
        let cosmic = theme.cosmic();
        let radius_s = cosmic.radius_s();
        let mut accent = Color::from(cosmic.accent_color());
        // draw two rectangles if hovered
        let should_draw = {
            let my_state = tree.state.downcast_ref::<MyState>();
            my_state.hovered || my_state.focused
        };

        if !should_draw {
            return;
        }

        let bounds = layout.bounds();
        accent.a = 0.7;
        renderer.fill_quad(
            Quad {
                bounds,
                border: Border {
                    radius: radius_s.into(),
                    width: 12.0,
                    color: accent,
                },
                shadow: Shadow::default(),
                snap: true,
            },
            Background::Color(Color::TRANSPARENT),
        );

        accent.a = 1.0;

        renderer.fill_quad(
            Quad {
                bounds,
                border: Border {
                    radius: radius_s.into(),
                    width: 4.0,
                    color: accent,
                },
                ..Default::default()
            },
            Background::Color(Color::TRANSPARENT),
        );
    }

    fn mouse_interaction(
        &self,
        _state: &Tree,
        layout: lingmo::iced::core::Layout<'_>,
        cursor: lingmo::iced::core::mouse::Cursor,
        _viewport: &lingmo::iced::core::Rectangle,
        _renderer: &lingmo::Renderer,
    ) -> lingmo::iced::core::mouse::Interaction {
        if cursor.is_over(layout.bounds()) {
            lingmo::iced::core::mouse::Interaction::Pointer
        } else {
            lingmo::iced::core::mouse::Interaction::default()
        }
    }

    fn update(
        &mut self,
        state: &mut Tree,
        event: &lingmo::iced::core::Event,
        layout: lingmo::iced::core::Layout<'_>,
        cursor: lingmo::iced::core::mouse::Cursor,
        _renderer: &lingmo::Renderer,
        _clipboard: &mut dyn lingmo::iced::core::Clipboard,
        shell: &mut lingmo::iced::core::Shell<'_, Msg>,
        _viewport: &lingmo::iced::core::Rectangle,
    ) {
        // update hover state
        let my_state = state.state.downcast_mut::<MyState>();
        let hovered = cursor.is_over(layout.bounds());
        let changed = my_state.hovered != hovered;
        my_state.hovered = hovered;

        if let lingmo::iced::core::Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) =
            event
        {
            shell.publish(self.on_press.clone());
            shell.capture_event();
        }
        if changed
            && let lingmo::iced::core::Event::Mouse(mouse::Event::CursorMoved { .. })
            | lingmo::iced::core::Event::Mouse(mouse::Event::CursorEntered) = event
        {
            shell.publish(self.on_enter.clone());
            shell.capture_event();
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct MyState {
    pub hovered: bool,
    pub focused: bool,
}

impl<'a, Message> From<OutputSelection<Message>> for lingmo::Element<'a, Message>
where
    Message: 'static + Clone,
{
    fn from(w: OutputSelection<Message>) -> lingmo::Element<'a, Message> {
        lingmo::Element::new(w)
    }
}
