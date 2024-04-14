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
use lightningcss::{properties::Property, traits::Parse};

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

// impl<T: IntoIterator<Item: TypeValidator> + Clone + Send + Sync> TypeValidator
//     for T
// {
//     fn validate(&self, value: &str) -> bool {
//         self.clone()
//             .into_iter()
//             .any(|validator| validator.validate(value))
//     }
// }
