use std::{
    any::type_name,
    fmt::{Debug, Display},
};

#[derive(Debug, Clone, PartialEq)]
pub struct EnumWrapper {
    pub name: String,
    pub variant: String,
    pub value: String,
}

pub trait ToEnumWrapper {
    fn to_enum_wrapper(&self) -> EnumWrapper;
}

impl<T> ToEnumWrapper for T
where
    T: odict::schema::EnumIdentifier + Display,
{
    fn to_enum_wrapper(&self) -> EnumWrapper {
        let name = type_name::<T>();
        let value = self.to_string();
        let variant = self.id();

        EnumWrapper {
            name: name.split("::").last().unwrap_or(name).to_string(),
            variant,
            value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use odict::schema::{FormKind, PartOfSpeech};

    #[test]
    fn test_to_enum_wrapper_pos_custom() {
        let wrapper = PartOfSpeech::Other("Custom".to_string()).to_enum_wrapper();

        assert_eq!(wrapper.name, "PartOfSpeech");
        assert_eq!(wrapper.variant, "other");
        assert_eq!(wrapper.value, "Custom");
    }

    #[test]
    fn test_to_enum_wrapper_pos() {
        let wrapper = PartOfSpeech::AdjKari.to_enum_wrapper();

        assert_eq!(wrapper.name, "PartOfSpeech");
        assert_eq!(wrapper.variant, "adj_kari");
        assert_eq!(wrapper.value, "'kari' adjective (archaic)");
    }

    #[test]
    fn test_to_enum_wrapper_form_kind() {
        let wrapper = FormKind::Conjugation.to_enum_wrapper();

        assert_eq!(wrapper.name, "FormKind");
        assert_eq!(wrapper.variant, "conjugation");
        assert_eq!(wrapper.value, "conjugation");
    }

    #[test]
    fn test_to_enum_wrapper_form_kind_custom() {
        let wrapper = FormKind::Other("Custom".to_string()).to_enum_wrapper();

        assert_eq!(wrapper.name, "FormKind");
        assert_eq!(wrapper.variant, "other");
        assert_eq!(wrapper.value, "Custom");
    }
}
