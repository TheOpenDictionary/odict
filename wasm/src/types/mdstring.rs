use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub enum MarkdownStrategy {
    Html,
    Plain,
}

impl Into<odict::MarkdownStrategy> for MarkdownStrategy {
    fn into(self) -> odict::MarkdownStrategy {
        match self {
            MarkdownStrategy::Html => odict::MarkdownStrategy::Html,
            MarkdownStrategy::Plain => odict::MarkdownStrategy::Plain,
        }
    }
}

#[wasm_bindgen]
pub struct String {
    mds: odict::String,
}

#[wasm_bindgen]
impl String {
    #[wasm_bindgen(constructor)]
    pub fn new(value: &str) -> Self {
        Self {
            mds: odict::String::from(value),
        }
    }

    #[wasm_bindgen(getter)]
    pub fn value(&self) -> String {
        self.mds.value().to_string()
    }

    pub fn parse(&self, strategy: MarkdownStrategy) -> String {
        let s: odict::MarkdownStrategy = strategy.into();
        self.mds.parse(s)
    }
}
