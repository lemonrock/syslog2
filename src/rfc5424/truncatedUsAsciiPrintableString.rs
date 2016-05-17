// This file is part of syslog2. It is subject to the license terms in the COPYRIGHT file found in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT. No part of syslog2, including this file, may be copied, modified, propagated, or distributed except according to the terms contained in the COPYRIGHT file.
// Copyright © 2016 The developers of syslog2. See the COPYRIGHT file in the top-level directory of this distribution and at https://raw.githubusercontent.com/lemonrock/syslog2/master/COPYRIGHT.


extern crate string_utilities;
use rfc5424::StructuredDataParameter;
use self::string_utilities::DefaultUsAsciiReplacementCharacter;
use self::string_utilities::to_8bit_encoding_replacement_function_us_ascii_printable;
use self::string_utilities::to_8bit_encoding_string;
use std::io::Write;
use std::borrow::Cow;

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct TruncatedUsAsciiPrintableString
{
	value: Vec<u8>
}

const LogglyIanaPrivateEnterpriseNumber: u32 = 41058;

impl TruncatedUsAsciiPrintableString
{
	pub fn new(string: &str, maximum_length: usize) -> TruncatedUsAsciiPrintableString
	{
		TruncatedUsAsciiPrintableString
		{
			value: TruncatedUsAsciiPrintableString::truncate_us_ascii_printable(string, maximum_length).into_bytes()
		}
	}
	
	pub fn new_sd_name(sd_name: &str) -> TruncatedUsAsciiPrintableString
	{
		validate_sd_name(sd_name);
		TruncatedUsAsciiPrintableString::new(sd_name, 32)
	}
	
	pub fn new_registered_structured_data_element_id(sd_name_for_structured_element_id: &str) -> TruncatedUsAsciiPrintableString
	{
		validate_sd_name_for_structured_element_id(sd_name_for_structured_element_id);
		TruncatedUsAsciiPrintableString::new(sd_name_for_structured_element_id, 32)
	}
	
	pub fn new_private_structured_data_element_id(sd_name_for_structured_element_id: &str, iana_private_enterprise_number: u32) -> TruncatedUsAsciiPrintableString
	{
		validate_sd_name_for_structured_element_id(sd_name_for_structured_element_id);
		TruncatedUsAsciiPrintableString::new(&format!("{}@{}", sd_name_for_structured_element_id, iana_private_enterprise_number), 32)
	}
	
	/// This is formatted as an UUID, eg  01234567-89ab-cdef-0123-456789abcdef
	pub fn new_private_structured_data_element_id_for_loggly(loggly_customer_token: &str) -> TruncatedUsAsciiPrintableString
	{
		TruncatedUsAsciiPrintableString::new_private_structured_data_element_id(loggly_customer_token, LogglyIanaPrivateEnterpriseNumber)
	}

	fn truncate_us_ascii_printable(string: &str, maximum_length: usize) -> String
	{
	    to_8bit_encoding_string(string, maximum_length, |character| to_8bit_encoding_replacement_function_us_ascii_printable(character, DefaultUsAsciiReplacementCharacter))
	}
	
	pub fn parameter<'a>(&'a self, value: Cow<'a, str>) -> StructuredDataParameter<'a>
	{
		StructuredDataParameter::new(self, value)
	}
}

fn validate_sd_name(sd_name: &str) -> bool
{
	for character in sd_name.chars()
	{
		match character
		{
			'"' | '\\' | ']' | '\x00' ... '\x20' | '\x7F' => panic!("A SDNAME can not contain control codes, spaces, DEL, double quotes, slashes or closing square brackets"),
			'\x21' ... '\x7E' => {},
			_ => panic!("A SDNAME can not contain Unicode characters that can not be converted to printable US-ASCII"),
		}
	}
	true
}

fn validate_sd_name_for_structured_element_id(sd_name_for_structured_element_id: &str) -> bool
{
	validate_sd_name(sd_name_for_structured_element_id);
	
	for character in sd_name_for_structured_element_id.chars()
	{
		match character
		{
			'@' => panic!("A SDNAME for a structured element id can contain an '@' character"),
			_ => {},
		}
	}
	true
}

pub trait WriteTruncatedUsAsciiPrintableString
{
    fn write_truncated(&mut self, value: &TruncatedUsAsciiPrintableString);
}

impl WriteTruncatedUsAsciiPrintableString for Vec<u8>
{
    fn write_truncated(&mut self, value: &TruncatedUsAsciiPrintableString)
	{
		self.write(value.value.as_slice());
	}
}
