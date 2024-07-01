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
        #[archive_attr(derive(Debug))]
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
        #[archive_attr(derive(Debug))]
        $i
    };
}

#[macro_export]
macro_rules! serializable_custom {
    ($i:item) => {
        #[derive(
            PartialEq, Eq, Debug, Clone, rkyv::Archive, rkyv::Serialize, rkyv::Deserialize,
        )]
        #[archive_attr(derive(Debug))]
        $i
    };
}
