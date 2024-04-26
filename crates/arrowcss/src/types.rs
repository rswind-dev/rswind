use lightningcss::{properties::Property, traits::Parse, values::angle::Angle};
pub use lightningcss::{
    properties::PropertyId,
    values::{
        color::CssColor,
        ident::Ident,
        image::Image,
        length::{Length, LengthPercentage},
        number::CSSNumber,
        percentage::Percentage,
        time::Time,
    },
};

pub trait TypeValidator: Sync + Send {
    fn validate(&self, value: &str) -> bool;
}

impl TypeValidator for CssProperty {
    fn validate(&self, value: &str) -> bool {
        !matches!(
            Property::parse_string(self.clone(), value, Default::default()),
            Ok(Property::Unparsed(_)) | Err(_)
        )
    }
}

pub type CssProperty = PropertyId<'static>;

#[derive(Clone, Copy, Debug)]
pub enum CssDataType {
    Color,
    Length,
    LengthPercentage,
    Percentage,
    Number,
    Ident,
    Image,
    Time,
    Angle,
    Any,
}

impl TypeValidator for CssDataType {
    fn validate(&self, value: &str) -> bool {
        match self {
            CssDataType::Color => CssColor::parse_string(value).is_ok(),
            CssDataType::Length => Length::parse_string(value).is_ok(),
            CssDataType::LengthPercentage => LengthPercentage::parse_string(value).is_ok(),
            CssDataType::Percentage => Percentage::parse_string(value).is_ok(),
            CssDataType::Number => CSSNumber::parse_string(value).is_ok(),
            CssDataType::Ident => Ident::parse_string(value).is_ok(),
            CssDataType::Image => Image::parse_string(value).is_ok(),
            CssDataType::Time => Time::parse_string(value).is_ok(),
            CssDataType::Angle => Angle::parse_string(value).is_ok(),
            CssDataType::Any => true,
        }
    }
}

impl<T: TypeValidator, const N: usize> TypeValidator for [T; N] {
    fn validate(&self, value: &str) -> bool {
        self.iter().any(|validator| validator.validate(value))
    }
}

#[derive(Debug, Clone, Copy)]
struct Vector;

impl<'i> Parse<'i> for Vector {
    fn parse<'t>(
        input: &mut cssparser::Parser<'i, 't>,
    ) -> Result<Self, cssparser::ParseError<'i, lightningcss::error::ParserError<'i>>> {
        input.try_parse(|input| {
            (f32::parse(input)?, f32::parse(input)?, f32::parse(input)?);
            Ok(Self)
        })
    }
}
