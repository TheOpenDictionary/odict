use crate::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

/* -------------------------------------------------------------------------- */
/*                                Split Options                               */
/* -------------------------------------------------------------------------- */

pub struct SplitOptions {
    min_length: usize,
}

impl AsRef<SplitOptions> for SplitOptions {
    fn as_ref(&self) -> &SplitOptions {
        self
    }
}

impl SplitOptions {
    pub fn default() -> Self {
        Self { min_length: 2 }
    }

    pub fn min_length(mut self, min_length: usize) -> Self {
        self.min_length = min_length;
        self
    }
}

/* -------------------------------------------------------------------------- */
/*                               Implementation                               */
/* -------------------------------------------------------------------------- */

macro_rules! split {
    ($t:ident, $r:ident) => {
        impl $t {
            pub fn split<Options: AsRef<SplitOptions>>(
                &self,
                query: &str,
                options: Options,
            ) -> crate::Result<Vec<&$r>> {
                let mut entries: Vec<&$r> = Vec::new();
                let mut start = 0;
                let chars: Vec<_> = query.chars().collect();
                let mut end = chars.len();

                let SplitOptions { min_length } = options.as_ref();

                while start < end {
                    let substr: String = chars[start..end].iter().collect();
                    let entry = self.entries.get(substr.as_str());

                    if entry.is_some() {
                        entries.push(entry.unwrap());
                    }

                    if entry.is_some() || substr.len() <= *min_length {
                        start = end;
                        end = chars.len();
                    } else {
                        end -= 1;
                    }
                }

                Ok(entries)
            }
        }
    };
}

split!(Dictionary, Entry);
split!(ArchivedDictionary, ArchivedEntry);
