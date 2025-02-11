use drive_tabs::DriveTabs;
use iced::widget::{column, Column};
use libminidisk::list_disks;

mod drive_tabs;

#[derive(Clone)]
pub struct MainView {
  pub drives: Vec<String>,
  pub drive: String,

  // ui
  tabs: DriveTabs,
}

#[derive(Debug, Clone)]
pub enum Message {
  SwitchDrive(String),
  RefreshDrives(Vec<String>),
}

impl Default for MainView {
  fn default() -> Self {
    Self::new()
  }
}

impl MainView {
  pub fn new() -> Self {
    let drives = list_disks().expect("Failed to list disks");

    Self {
      drives: drives.clone(),
      drive: drives[0].clone(),
      tabs: DriveTabs::new(drives),
    }
  }

  pub fn view(&self) -> Column<Message> {
    column![
      self.tabs.view(),
    ]
  }

  pub fn update(&mut self, message: Message) {
    match message {
      Message::SwitchDrive(index) => {
        println!("Switch drive to {}", index);
      }
      Message::RefreshDrives(drives) => {
        self.drives = drives;
      }
    }
  }
}