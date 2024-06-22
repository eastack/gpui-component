use core::fmt;
use std::fmt::{Display, Formatter};

use gpui::{
    div, prelude::FluentBuilder as _, px, AnyElement, ElementId, IntoElement, ParentElement,
    Render, RenderOnce, SharedString, Styled as _, View, ViewContext, VisualContext, WindowContext,
};

mod button_story;
mod input_story;

use crate::{button::Button, disableable::Clickable as _, label::Label};

use button_story::ButtonStory;
use input_story::InputStory;

pub fn story_case(name: &'static str, description: &'static str) -> StoryContainer {
    StoryContainer::new(name, description)
}

#[derive(IntoElement)]
pub struct StoryContainer {
    name: SharedString,
    description: SharedString,
    children: Vec<AnyElement>,
}

impl ParentElement for StoryContainer {
    fn extend(&mut self, elements: impl IntoIterator<Item = AnyElement>) {
        self.children.extend(elements)
    }
}

impl StoryContainer {
    pub fn new(name: impl Into<SharedString>, description: impl Into<SharedString>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            children: Vec::new(),
        }
    }
}

impl RenderOnce for StoryContainer {
    fn render(self, _cx: &mut WindowContext) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(Label::new(self.name).text_size(px(24.0)))
                    .child(Label::new(self.description).text_size(px(16.0))),
            )
            .children(self.children)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum StoryType {
    Button,
    Input,
}

impl Display for StoryType {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::Button => write!(f, "Button"),
            Self::Input => write!(f, "Input"),
        }
    }
}

pub struct Stories {
    active: StoryType,
}

impl Stories {
    fn new() -> Self {
        Self {
            active: StoryType::Button,
        }
    }

    pub fn view(cx: &mut WindowContext) -> View<Self> {
        cx.new_view(|_cx| Self::new())
    }

    fn set_active(&mut self, ty: StoryType, cx: &mut ViewContext<Self>) {
        self.active = ty;
        dbg!("--------------------- set_active: {}", ty);
        cx.notify();
    }

    fn render_story_buttons(&self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .gap_4()
            .child(self.swith_button("story-button", StoryType::Button, cx))
            .child(self.swith_button("story-input", StoryType::Input, cx))
    }

    fn swith_button(
        &self,
        id: &str,
        ty: StoryType,
        cx: &mut ViewContext<Self>,
    ) -> impl IntoElement {
        let name = format!("{}", ty);
        Button::new(SharedString::from(id.to_string()), name)
            .on_click(move |_e, cx| {
                dbg!("--------------------- on_click: {}", ty);
                // cx.update_view(self, |this| {
                //     this.set_active(ty, cx);
                // });
            })
            .style(crate::button::ButtonStyle::Secondary)
    }
}

impl Default for Stories {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for Stories {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(self.render_story_buttons(cx))
            .map(|this| match self.active {
                StoryType::Button => this.child(cx.new_view(|_cx| ButtonStory {})),
                StoryType::Input => this.child(cx.new_view(|_cx| InputStory {})),
            })
    }
}
