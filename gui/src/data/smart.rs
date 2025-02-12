use libatasmart_sys::SkSmartOverall;

pub fn smart_to_string(sk: SkSmartOverall) -> String {
  match sk {
    SkSmartOverall::SK_SMART_OVERALL_GOOD => "Good",
    SkSmartOverall::SK_SMART_OVERALL_BAD_ATTRIBUTE_IN_THE_PAST => "Bad Attribute In The Past",
    SkSmartOverall::SK_SMART_OVERALL_BAD_SECTOR => "Bad Sector",
    SkSmartOverall::SK_SMART_OVERALL_BAD_ATTRIBUTE_NOW => "Bad Attribute Now",
    SkSmartOverall::SK_SMART_OVERALL_BAD_SECTOR_MANY => "Bad Sector Many",
    SkSmartOverall::SK_SMART_OVERALL_BAD_STATUS => "Bad Status",
    _ => "Unknown",
  }.to_string()
}