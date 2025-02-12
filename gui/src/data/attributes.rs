use std::ffi::CStr;

use dioxus::Ok;
use libatasmart_sys::{SkDisk, SkSmartAttributeParsedData};
use libminidisk::libatasmart::Disk;

#[derive(Default, Debug)]
pub struct Attribute {
  pub id: u8,
  pub name: String,
  pub threshold: u8,
  pub warn: bool,
  pub current: u8,
  pub worst: u8,
  pub raw: [u8; 6],
}

pub fn raw_to_string(raw: [u8; 6]) -> String {
  let mut s = String::new();
  for i in 0..6 {
    s.push_str(&format!("{:02x}", raw[i]));
  }
  s
}

// pub fn get_attribute(name: impl AsRef<str>, disk: &mut Disk) -> Option<Attribute> {
//   let mut attribute = Attribute::default();
//   attribute.name = name.as_ref().to_string();
//   // Create poitner to attribute
//   let a = Box::into_raw(Box::new(attribute));
//   let result = disk.parse_attributes(fetch_attribute, a as *mut std::ffi::c_void);

//   if result.is_ok() {
//     return Some(unsafe { *Box::from_raw(a) });
//   }

//   None
// }

pub fn get_all_attributes(disk: &mut Disk) -> Vec<Attribute> {
  let attributes: Vec<Attribute> = Vec::new();
  let mut a = Box::new(attributes);

  let result = disk.parse_attributes(fetch_all_attributes, &mut *a as *mut Vec<Attribute> as *mut std::ffi::c_void);

  if result.is_ok() {
    return *a
  }

  Vec::new()
}

// extern "C" fn fetch_attribute(_disk: *mut SkDisk, a: *const SkSmartAttributeParsedData, ah: *mut std::ffi::c_void) {
//   let name = unsafe { CStr::from_ptr((*a).name) }.to_str().unwrap();
//   let attribute = unsafe { &mut *(ah as *mut Attribute) };

//   if name == attribute.name {
//     attribute.id = unsafe { (*a).id };
//     attribute.threshold = unsafe { (*a).threshold };
//     attribute.warn = unsafe { (*a).warn == 1 };
//     attribute.current = unsafe { (*a).current_value };
//     attribute.worst = unsafe { (*a).worst_value };
//     attribute.raw = unsafe { (*a).raw };
//   }
// }

extern "C" fn fetch_all_attributes(_disk: *mut SkDisk, a: *const SkSmartAttributeParsedData, ah: *mut std::ffi::c_void) {
  let attributes = unsafe { &mut *(ah as *mut Vec<Attribute>) };
  let name = unsafe { CStr::from_ptr((*a).name) }.to_str().unwrap();

  attributes.push(Attribute {
    name: name.to_string(),
    id: unsafe { (*a).id },
    threshold: unsafe { if (*a).threshold_valid == 1 { (*a).threshold } else { 0 } },
    warn: unsafe { (*a).warn == 1 },
    current: unsafe { if (*a).current_value_valid == 1 { (*a).current_value } else { 0 } },
    worst: unsafe { if (*a).worst_value_valid == 1 { (*a).worst_value } else { 0 } },
    raw: unsafe { (*a).raw },
  });
}
