pub struct SplitOptions {
    pub threshold: usize,
    pub follow: bool,
    pub insensitive: bool,
}

impl AsRef<SplitOptions> for SplitOptions {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl SplitOptions {
    pub fn default() -> Self {
        Self {
            threshold: 1,
            follow: false,
            insensitive: false,
        }
    }

    pub fn threshold(mut self, threshold: usize) -> Self {
        self.threshold = threshold;
        self
    }

    pub fn follow(mut self, follow: bool) -> Self {
        self.follow = follow;
        self
    }

    pub fn insensitive(mut self, insensitive: bool) -> Self {
        self.insensitive = insensitive;
        self
    }
}
