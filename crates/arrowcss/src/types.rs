use lightningcss::{properties::Property, traits::Parse};

pub use lightningcss::properties::PropertyId;
pub use lightningcss::values::{
    color::CssColor,
    ident::Ident,
    image::Image,
    length::{Length, LengthPercentage},
    number::CSSNumber,
    percentage::Percentage,
    time::Time,
};

pub trait TypeValidator {
    fn validate(&self, value: &str) -> bool;
}

impl TypeValidator for PropertyId<'_> {
    fn validate(&self, value: &str) -> bool {
        !matches!(
            Property::parse_string(self.clone(), value, Default::default()),
            Ok(Property::Unparsed(_)) | Err(_)
        )
    }
}

#[allow(dead_code)]
pub enum CssDataType {
    Color,
    Length,
    LengthPercentage,
    Percentage,
    Number,
    Ident,
    Image,
    Time,
    Any,
}

impl TypeValidator for CssDataType {
    fn validate(&self, value: &str) -> bool {
        match self {
            CssDataType::Color => CssColor::parse_string(value).is_ok(),
            CssDataType::Length => Length::parse_string(value).is_ok(),
            CssDataType::LengthPercentage => {
                LengthPercentage::parse_string(value).is_ok()
            }
            CssDataType::Percentage => Percentage::parse_string(value).is_ok(),
            CssDataType::Number => CSSNumber::parse_string(value).is_ok(),
            CssDataType::Ident => Ident::parse_string(value).is_ok(),
            CssDataType::Image => Image::parse_string(value).is_ok(),
            CssDataType::Time => Time::parse_string(value).is_ok(),
            CssDataType::Any => true,
        }
    }
}

macro_rules! impl_type_validator {
    ($($ty:ty),*) => {
        $(
            impl TypeValidator for $ty {
                fn validate(&self, value: &str) -> bool {
                    <$ty>::parse_string(value).is_ok()
                }
            }
        )*
    };
}

impl_type_validator!(
    CssColor,
    Length,
    LengthPercentage,
    Percentage,
    CSSNumber,
    Ident<'_>,
    Image<'_>,
    Time
);
