use std::fmt;
use std::str::FromStr;
use std::num::ParseIntError;
use serde::{de::{self, Visitor}, Deserialize, Deserializer, Serialize, Serializer};


pub const WHITE: Color = Color {red: 0xFF, green: 0xFF, blue: 0xFF};
pub const BLACK: Color = Color {red: 0x00, green: 0x00, blue: 0x00};



#[derive(Clone, PartialEq, Eq)]
pub struct Color {
    pub red: u8,
    pub green: u8,
    pub blue: u8,
}

impl fmt::Display for Color {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			&WHITE => f.write_str("white"),
			&BLACK => f.write_str("black"),
			color  => {
				write!(f, "#{:02X}{:02X}{:02X}", color.red, color.green, color.blue)
			},
		}
	}
}

impl FromStr for Color {
	type Err = ColorError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"white" => Ok(WHITE.to_owned()),
			"black" => Ok(BLACK.to_owned()),
			s if s.starts_with("#") && s.len() == 7 => {
				let red = u8::from_str_radix(&s[1..3], 16)?;
				let green = u8::from_str_radix(&s[3..5], 16)?;
				let blue = u8::from_str_radix(&s[5..7], 16)?;
				Ok(Color {red, green, blue})
			},
			other => {
				Err(ColorError::InvalidValue {value: other.to_owned()})
			},
		}
	}
}

impl Serialize for Color {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
	    S: Serializer,
	{
		serializer.serialize_str(&self.to_string())
	}
}