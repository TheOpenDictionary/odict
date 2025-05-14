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
        #[rkyv(derive(Debug, PartialEq, Eq))]
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
        #[derive(
            Hash,
            Ord,
            PartialOrd,
            PartialEq,
            Eq,
            Debug,
            Clone,
            strum::VariantNames,
            strum::Display,
            rkyv::Archive,
            rkyv::Serialize,
            rkyv::Deserialize,
            serde::Serialize,
            serde::Deserialize,
        )]
        #[rkyv(derive(Debug, PartialEq, Eq, Hash))]
        #[strum(ascii_case_insensitive, serialize_all = "snake_case")]
        #[serde(rename_all = "snake_case")]
        $i
    };
}
