#![cfg_attr(feature = "nightly", feature(custom_derive, custom_attribute, plugin))]
#![cfg_attr(feature = "nightly", plugin(serde_macros, json_str, elastic_types_macros, elastic_date_macros))]

pub mod mapping;
pub mod formats;

#[test]
fn can_change_point_mapping() {
	panic!("implement")
}

#[test]
fn serialise_elastic_point() {
	panic!("implement")
}

#[test]
fn deserialise_elastic_point() {
	panic!("implement")
}