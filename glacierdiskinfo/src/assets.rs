use base64::prelude::*;

// TODO I want to go back to using asset!() but it doesn't work for me right now
pub const CSS: [&str; 5] = [
  include_str!("../assets/main.css"),
  include_str!("../assets/drive.css"),
  include_str!("../assets/driveattrtable.css"),
  include_str!("../assets/driveinfotable.css"),
  include_str!("../assets/drivetabs.css"),
];

pub const BAD_ICO: &[u8; 1150] = include_bytes!("../assets/img/bad.ico");
pub const CAUTION_ICO: &[u8; 1150] = include_bytes!("../assets/img/caution.ico");
pub const GOOD_ICO: &[u8; 1150] = include_bytes!("../assets/img/good.ico");

pub fn ico_to_data_uri(ico: &[u8; 1150]) -> String {
  let b64 = BASE64_STANDARD.encode(ico);
  format!("data:image/x-icon;base64,{b64}")
}
