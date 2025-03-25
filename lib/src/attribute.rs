use std::ffi::CStr;

use libatasmart::Disk;
use libatasmart_sys::{SkDisk, SkSmartAttributeParsedData, SkSmartAttributeUnit};

/// Handles SkSmartAttributUnit, so that we can convert whatever number to the "base" and then do formatting on that
pub trait Convertable {
  fn to_base_number(&self) -> u64;
  fn convert_to_base(&self, value: u64) -> u64;
}

impl Convertable for SkSmartAttributeUnit {
  /// Take a value and return what would need to be multiplied by the base to get the "right" value
  fn to_base_number(&self) -> u64 {
    match self {
      SkSmartAttributeUnit::SK_SMART_ATTRIBUTE_UNIT_MB => 1000 * 1000,
      SkSmartAttributeUnit::_SK_SMART_ATTRIBUTE_UNIT_MAX => 0,
      _ => 1,
    }
  }

  /// Convert a number of some unit to the "base" unit
  fn convert_to_base(&self, value: u64) -> u64 {
    let base = self.to_base_number();
    value * base
  }
}

/// A struct representing a SMART attribute
#[derive(Debug, Clone)]
pub struct Attribute {
  /// Numerical ID of the attribute
  pub id: u8,
  /// Name of the attribute
  pub name: String,
  pub threshold: u8,
  pub warn: bool,
  pub current: u8,
  pub worst: u8,

  // Pretty unit and value are sometimes the "right" thing to use
  pub pretty_unit: SkSmartAttributeUnit,
  pub pretty_value: u64,

  pub raw: [u8; 6],
}

impl PartialEq for Attribute {
  fn eq(&self, other: &Self) -> bool {
    self.id == other.id
      && self.name == other.name
      && self.threshold == other.threshold
      && self.warn == other.warn
      && self.current == other.current
      && self.worst == other.worst
      && self.pretty_value == other.pretty_value
  }
}

impl Default for Attribute {
  fn default() -> Self {
    Attribute {
      id: 0,
      name: "".to_string(),
      threshold: 0,
      warn: false,
      current: 0,
      worst: 0,
      pretty_unit: SkSmartAttributeUnit::SK_SMART_ATTRIBUTE_UNIT_UNKNOWN,
      pretty_value: 0,
      raw: [0; 6],
    }
  }
}

impl Attribute {
  /// Convert the raw attribute value to a string
  pub fn raw_str(&self) -> String {
    let mut s = String::new();
    for r in self.raw {
      s.push_str(&format!("{:02x}", r));
    }
    s
  }
}

/// Get an [`Attribute`] from a [`Disk`] by name
pub fn get_attribute(disk: &mut Disk, name: impl AsRef<str>) -> Option<Attribute> {
  let attribute = Attribute {
    name: name.as_ref().to_string(),
    ..Default::default()
  };
  // Create poitner to attribute
  let mut a = Box::new(attribute);
  let result = unsafe {
      disk.parse_attributes(
      fetch_attribute,
      &mut *a as *mut Attribute as *mut std::ffi::c_void,
    )
  };

  if result.is_ok() {
    return Some(*a);
  }

  None
}

/// Get all [`Attribute`]s from a [`Disk`]
pub fn get_all_attributes(disk: &mut Disk) -> Vec<Attribute> {
  let attributes: Vec<Attribute> = Vec::new();
  let mut a = Box::new(attributes);

  let result = unsafe {
      disk.parse_attributes(
      fetch_all_attributes,
      &mut *a as *mut Vec<Attribute> as *mut std::ffi::c_void,
    )
  };

  if result.is_ok() {
    return *a;
  }

  Vec::new()
}

extern "C" fn fetch_attribute(
  _disk: *mut SkDisk,
  a: *const SkSmartAttributeParsedData,
  ah: *mut std::ffi::c_void,
) {
  let name = unsafe { CStr::from_ptr((*a).name) }.to_str().unwrap();
  let attribute = unsafe { &mut *(ah as *mut Attribute) };

  if name == attribute.name {
    attribute.id = unsafe { (*a).id };
    attribute.threshold = unsafe { (*a).threshold };
    attribute.warn = unsafe { (*a).warn() == 1 };
    attribute.current = unsafe { (*a).current_value };
    attribute.worst = unsafe { (*a).worst_value };
    attribute.pretty_unit = unsafe { (*a).pretty_unit };
    attribute.pretty_value = unsafe { (*a).pretty_value };
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
    warn: unsafe { (*a).warn() == 1 },
    current: unsafe { (*a).current_value },
    worst: unsafe { (*a).worst_value },

    pretty_unit: unsafe { (*a).pretty_unit },
    pretty_value: unsafe { (*a).pretty_value },

    raw: unsafe { (*a).raw },
  });
}
