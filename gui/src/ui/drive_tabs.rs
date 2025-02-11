use iced::{widget::{button, row, text, Button, Row}, Element};
use libminidisk::libatasmart::Disk;

use super::Message;

#[derive(Clone, Default)]
pub struct DriveTabs {
  pub disks: Vec<String>,
}

impl DriveTabs {
  pub fn new(disks: Vec<String>) -> Self {
    Self { disks }
  }

  pub fn view(&self) -> Row<Message> {
    let mut tabs = vec![];

    for disk in &self.disks {
      tabs.push(tab_button(disk.clone()));
    }

    Row::with_children(tabs)
      .spacing(10)
  }
}

fn tab_button(disk: String) -> Element<'static, Message> {
  button(text(disk.clone()))
    .width(80)
    .height(60)
    .on_press(Message::SwitchDrive(disk))
    .into()
}