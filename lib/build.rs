fn main() {
    #[cfg(feature = "uniffi")]
    uniffi::generate_scaffolding("src/odict.udl").unwrap();
}
