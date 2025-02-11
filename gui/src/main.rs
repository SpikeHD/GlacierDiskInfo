use ui::MainView;

mod ui;

fn main() {
  iced::application::application("MiniDisk", MainView::update, MainView::view).run().expect("Failed to run application"); 
}
