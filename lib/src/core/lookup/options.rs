#[derive(Debug, PartialEq, Clone)]
pub enum LookupStrategy {
    Exact,
    Split(usize),
}

#[derive(Debug, Clone)]
pub struct LookupOptions {
    /// Maximum number of redirects to follow via see_also links.
    /// None means no following, Some(u32::MAX) provides infinite following (old behavior).
    pub follow: Option<u32>,
    pub strategy: LookupStrategy,
    pub insensitive: bool,
}

impl AsRef<LookupOptions> for LookupOptions {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl LookupOptions {
    pub fn default() -> Self {
        Self {
            follow: None,
            strategy: LookupStrategy::Exact,
            insensitive: false,
        }
    }

    pub fn follow(mut self, follow: u32) -> Self {
        self.follow = Some(follow);
        self
    }

    pub fn strategy(mut self, strategy: LookupStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    pub fn insensitive(mut self, insensitive: bool) -> Self {
        self.insensitive = insensitive;
        self
    }
}
