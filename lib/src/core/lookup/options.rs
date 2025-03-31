#[derive(Debug, PartialEq, Clone)]
pub enum LookupStrategy {
    Exact,
    Split,
    Tokenize,
}

#[derive(Debug, Clone)]
pub struct LookupOptions {
    pub follow: bool,
    pub strategy: LookupStrategy,
}

impl AsRef<LookupOptions> for LookupOptions {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl LookupOptions {
    pub fn default() -> Self {
        Self {
            follow: true,
            strategy: LookupStrategy::Tokenize,
        }
    }

    pub fn follow(mut self, follow: bool) -> Self {
        self.follow = follow;
        self
    }

    pub fn strategy(mut self, strategy: LookupStrategy) -> Self {
        self.strategy = strategy;
        self
    }
}
