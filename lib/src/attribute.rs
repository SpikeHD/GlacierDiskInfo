use std::ffi::CStr;

use libatasmart::Disk;
use libatasmart_sys::{SkDisk, SkSmartAttributeParsedData};

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

pub fn get_attribute(name: impl AsRef<str>, disk: &mut Disk) -> Option<Attribute> {
  let mut attribute = Attribute::default();
  attribute.name = name.as_ref().to_string();
  // Create poitner to attribute
  let mut a = Box::new(attribute);
  let result = disk.parse_attributes(fetch_attribute, &mut *a as *mut Attribute as *mut std::ffi::c_void);

  if result.is_ok() {
    return Some(*a);
  }

  None
}

// TODO move ALL of this stuff to libglacierdisk
pub fn get_all_attributes(disk: &mut Disk) -> Vec<Attribute> {
  let attributes: Vec<Attribute> = Vec::new();
  let mut a = Box::new(attributes);

  let result = disk.parse_attributes(
    fetch_all_attributes,
    &mut *a as *mut Vec<Attribute> as *mut std::ffi::c_void,
  );

  if result.is_ok() {
    return *a;
  }

  Vec::new()
}

extern "C" fn fetch_attribute(_disk: *mut SkDisk, a: *const SkSmartAttributeParsedData, ah: *mut std::ffi::c_void) {
  let name = unsafe { CStr::from_ptr((*a).name) }.to_str().unwrap();
  let attribute = unsafe { &mut *(ah as *mut Attribute) };

  if name == attribute.name {
    attribute.id = unsafe { (*a).id };
    attribute.threshold = unsafe { (*a).threshold };
    attribute.warn = unsafe { (*a).warn == 1 };
    attribute.current = unsafe { (*a).current_value };
    attribute.worst = unsafe { (*a).worst_value };
    attribute.raw = unsafe { (*a).raw };
  }
}

extern "C" fn fetch_all_attributes(
  _disk: *mut SkDisk,
  a: *const SkSmartAttributeParsedData,
  ah: *mut std::ffi::c_void,
) {
  let attributes = unsafe { &mut *(ah as *mut Vec<Attribute>) };
  let name = unsafe { CStr::from_ptr((*a).name) }.to_str().unwrap();

  attributes.push(Attribute {
    name: name.to_string(),
    id: unsafe { (*a).id },
    threshold: unsafe { (*a).threshold },
    warn: unsafe { (*a).warn == 1 },
    current: unsafe { (*a).current_value },
    worst: unsafe { (*a).worst_value },
    raw: unsafe { (*a).raw },
  });
}
