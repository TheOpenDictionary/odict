#[derive(uniffi::Record, Debug, Clone, PartialEq)]
pub struct EnumWrapper {
    pub name: String,
    pub variant: String,
    pub value: String,
}

impl From<internal::EnumWrapper> for EnumWrapper {
    fn from(w: internal::EnumWrapper) -> Self {
        Self {
            name: w.name,
            variant: w.variant,
            value: w.value,
        }
    }
}

impl From<odict::schema::PronunciationKind> for EnumWrapper {
    fn from(p: odict::schema::PronunciationKind) -> Self {
        use internal::ToEnumWrapper;
        p.to_enum_wrapper().into()
    }
}

impl From<odict::schema::FormKind> for EnumWrapper {
    fn from(f: odict::schema::FormKind) -> Self {
        use internal::ToEnumWrapper;
        f.to_enum_wrapper().into()
    }
}
