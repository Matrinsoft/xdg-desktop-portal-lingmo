use lingmo::iced::core::event::Event;
use lingmo::iced::core::widget::{Operation, Tree};
use lingmo::iced::core::{
    Clipboard, Element, Layout, Length, Rectangle, Shell, Size, Widget, keyboard, layout, mouse,
    overlay, renderer,
};

#[allow(missing_debug_implementations)]
pub struct KeyboardWrapper<'a, Message> {
    content: Element<'a, Message, lingmo::Theme, lingmo::Renderer>,
    handler: fn(keyboard::Key, keyboard::Modifiers) -> Option<Message>,
}

impl<'a, Message> KeyboardWrapper<'a, Message> {
    /// Creates a [`KeyboardWrapper`] with the given content.
    pub fn new(
        content: impl Into<Element<'a, Message, lingmo::Theme, lingmo::Renderer>>,
        handler: fn(keyboard::Key, keyboard::Modifiers) -> Option<Message>,
    ) -> Self {
        KeyboardWrapper {
            content: content.into(),
            handler,
        }
    }
}

impl<'a, Message> Widget<Message, lingmo::Theme, lingmo::Renderer> for KeyboardWrapper<'a, Message>
where
    Message: Clone,
{
    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&mut self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_mut(&mut self.content));
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &lingmo::Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content
            .as_widget_mut()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &lingmo::Renderer,
        operation: &mut dyn Operation<()>,
    ) {
        self.content
            .as_widget_mut()
            .operate(&mut tree.children[0], layout, renderer, operation);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &lingmo::Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
        if shell.is_event_captured() {
            return;
        }

        #[allow(clippy::single_match)]
        match event {
            Event::Keyboard(keyboard::Event::KeyPressed { key, modifiers, .. }) => {
                if let Some(message) = (self.handler)(key.clone(), *modifiers) {
                    shell.publish(message.clone());
                    shell.capture_event();
                }
            }
            /*
                keyboard::key::Named::Escape => {
                    event::Status::Ignored
                }
                keyboard::key::Named::Enter => {
                    event::Status::Ignored
                }
                _ => event::Status::Ignored
            },
            */
            _ => (),
        };
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &lingmo::Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut lingmo::Renderer,
        theme: &lingmo::Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            renderer_style,
            layout,
            cursor,
            viewport,
        );
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &lingmo::Renderer,
        viewport: &Rectangle,
        translation: lingmo::iced::Vector,
    ) -> Option<overlay::Element<'b, Message, lingmo::Theme, lingmo::Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }

    fn drag_destinations(
        &self,
        state: &Tree,
        layout: Layout<'_>,
        renderer: &lingmo::Renderer,
        dnd_rectangles: &mut lingmo::iced::core::clipboard::DndDestinationRectangles,
    ) {
        if let Some(state) = state.children.first() {
            self.content
                .as_widget()
                .drag_destinations(state, layout, renderer, dnd_rectangles);
        }
    }
}

impl<'a, Message> From<KeyboardWrapper<'a, Message>>
    for Element<'a, Message, lingmo::Theme, lingmo::Renderer>
where
    Message: 'a + Clone,
{
    fn from(
        area: KeyboardWrapper<'a, Message>,
    ) -> Element<'a, Message, lingmo::Theme, lingmo::Renderer> {
        Element::new(area)
    }
}
