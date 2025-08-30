// use odict::alias::AliasManager;
// use structural_convert::StructuralConvert;

// #[napi(object)]
// #[derive(PartialEq, Default, Clone, Eq)]
// pub struct AliasLoadOptions {
//   pub path: Option<String>,
// }

// impl CompressOptions {
//   pub fn with_path(mut self, path: String) -> Self {
//     self.path = Some(path);
//     self
//   }
// }

// #[napi(object)]
// #[derive(PartialEq, StructuralConvert, Default, Clone, Eq)]
// #[convert(from(odict::write::CompressOptions))]
// pub struct CompressOptions {
//   pub alias: Option<AliasLoadOptions>,
// }

// impl TryFrom<LoadOptions> for internal::LoadDictionaryOptions {
//   type Error = odict::Error;

//   fn try_from(opts: LoadOptions) -> Result<Self, Self::Error> {
//     let mut options = internal::LoadDictionaryOptions::default();

//     if let Some(path) = opts.alias.and_then(|a| a.path) {
//       options = options.with_alias_manager(AliasManager::new(&path)?);
//     }

//     Ok(options)
//   }
// }
