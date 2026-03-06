use crate::app::app::Message;
use crate::app::SpaceNavCockpit;
use iced::widget::{container, text};
use iced::{widget, Element};
use iced_aw::{TabBar, TabLabel};

pub fn configuration_view(app: &SpaceNavCockpit) -> Element<'_, Message> {

    let tab_bar = {
        let mut tab_bar = app.profiles.profiles.iter()
            .map(|(id, profile)| (id.to_owned(), TabLabel::Text(Clone::clone(&profile.title))))
            .fold(TabBar::new(Message::TabSelected), |tab_bar, (key, tab)| tab_bar.push(key, tab));
        if let Some((profile, _)) = &app.selected_profile {
            tab_bar = tab_bar.set_active_tab(profile)
        };
        tab_bar
    };

    let content = {
        let content = widget::Column::new()
            .spacing(10)
            .padding(10)
            .push(tab_bar);
        match &app.selected_profile {
            None => {
                content
            }
            Some((_, profile)) => {
                content.push(widget::Row::new()
                    .spacing(10)
                    .push(text("Profile:"))
                    .push(text(Clone::clone(&profile.title)))
                )
            }
        }
    };

    container(content).into()
}
