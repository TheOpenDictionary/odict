use strum_macros::{EnumDiscriminants, EnumMessage, EnumString};

#[macro_export]
macro_rules! serializable {
    ($i:item) => {
        #[derive(
            PartialEq,
            Eq,
            Debug,
            Clone,
            serde::Serialize,
            serde::Deserialize,
            rkyv::Archive,
            rkyv::Serialize,
            rkyv::Deserialize,
        )]
        #[rkyv(derive(Debug))]
        $i
    };
}

#[macro_export]
macro_rules! serializable_test {
    ($i:item) => {
        #[derive(
            PartialEq,
            Eq,
            Debug,
            Clone,
            serde::Deserialize,
            rkyv::Archive,
            rkyv::Serialize,
            rkyv::Deserialize,
        )]
        #[rkyv(derive(Debug))]
        $i
    };
}

#[macro_export]
macro_rules! serializable_custom {
    ($i:item) => {
        #[derive(
            PartialEq, Eq, Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
        )]
        #[rkyv(derive(Debug))]
        $i
    };
}

#[macro_export]
macro_rules! serializable_enum {
    ($i:item) => {
        #[strum_discriminants(derive(EnumString, EnumMessage))],
        #[derive(
            Hash,
            Ord,
            PartialOrd,
            PartialEq,
            Eq,
            Debug,
            Clone,
            rkyv::Archive,
            rkyv::Serialize,
            rkyv::Deserialize,
        )]
        #[rkyv(derive(Debug, PartialEq, Eq, Hash))]
        #[repr(u8)]
        #[strum(ascii_case_insensitive, serialize_all = "snake_case")]
        $i
    };
}
