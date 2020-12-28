#![allow(dead_code)]

extern crate elrond_codec_derive;
use elrond_codec_derive::*;

// This test doesn't run any code, the fact that it compiles is the actual testing.
// It checks that defining other types with the same name as elrond-codec types does not break the derive macro generation.
// The derive macro must generate fully qualified type names everywhere to avoid this hurdle.

// All exported traits:
struct TopEncode;
struct TopDecode;
struct NestedEncode;
struct NestedDecode;
struct EncodeError;
struct DecodeError;
struct TopDecodeInput;
struct TopEncodeOutput;
struct NestedDecodeInput;
struct NestedEncodeOutput;
struct NestedEncodeNoErr;

// Making sure derive explicitly only works with core::result::Result
// and doesn't get tricked by other enums with the same name.
enum Result {
	Ok,
	Err,
}

// This one will interfere with any improperly generated `Ok` and `Err` expressions.
#[allow(unused_imports)]
use crate::Result::{Err, Ok};

// Also adding all public functions exposed by elrond-codec.
// They are not used in the derive, but just to make sure:
fn bytes_to_number() {}
fn top_encode_number_to_output() {}
fn using_encoded_number() {}
fn dep_decode_from_byte_slice() {}
fn dep_decode_from_byte_slice_or_exit() {}
fn dep_encode_to_vec() {}
fn top_decode_from_nested() {}
fn top_decode_from_nested_or_exit() {}
fn top_encode_from_nested() {}
fn top_encode_from_nested_or_exit() {}
fn top_encode_to_vec() {}
fn boxed_slice_into_vec() {}
fn vec_into_boxed_slice() {}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Debug)]
pub struct Struct {
	pub int: u16,
	pub seq: Vec<u8>,
	pub another_byte: u8,
	pub uint_32: u32,
	pub uint_64: u64,
}

#[derive(TopEncode, TopDecode, NestedEncode, NestedDecode, PartialEq, Clone, Debug)]
enum DayOfWeek {
	Monday,
	Tuesday,
	Wednesday,
	Thursday,
	Friday,
	Saturday,
	Sunday,
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Debug)]
enum EnumWithEverything {
	Quit,
	Today(DayOfWeek),
	Write(Vec<u8>, u16),
	Struct {
		int: u16,
		seq: Vec<u8>,
		another_byte: u8,
		uint_32: u32,
		uint_64: u64,
	},
}

trait SimpleTrait {
	fn simple_function(&self);
}

#[derive(NestedEncode, NestedDecode, TopEncode, TopDecode, PartialEq, Clone, Debug)]
struct StructWithNamedFieldsWithGeneric<ST: SimpleTrait>
where
	ST: elrond_codec::NestedEncode
		+ elrond_codec::NestedDecode
		+ elrond_codec::TopEncode
		+ elrond_codec::TopDecode,
{
	data: u64,
	trait_stuff: ST,
}
