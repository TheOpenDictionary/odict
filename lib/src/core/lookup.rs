//! Advanced dictionary lookup operations for ODict.
//!
//! This module provides sophisticated search capabilities over dictionaries with
//! configurable matching strategies, redirect following via see_also links, and
//! case-insensitive fallback options. It supports both single and batch lookups
//! with parallel processing for optimal performance.
//!
//! # Overview
//!
//! The lookup system offers multiple layers of functionality:
//!
//! ## Matching Strategies
//! - **Exact matching**: Direct term-to-entry mapping
//! - **Split strategy**: Progressive substring matching for compound terms
//!
//! ## Advanced Features
//! - **Redirect following**: Automatic traversal of see_also links with cycle protection
//! - **Case-insensitive fallback**: Automatic retry with lowercase when exact match fails
//! - **Parallel processing**: Concurrent lookup of multiple queries for performance
//! - **Configurable limits**: Control redirect depth and matching behavior
//!
//! ## Performance Characteristics
//! - Single lookups: O(1) average case for exact matches
//! - Split strategy: O(n²) worst case where n is query length
//! - Parallel lookups: Scales with available CPU cores
//! - Memory efficient: Zero-copy results with lifetime management
//!
//! # Examples
//!
//! ## Basic Exact Lookup
//!
//! ```rust
//! use odict::{OpenDictionary, LookupOptions};
//!
//! let dict = OpenDictionary::from_path("dictionary.odict")?;
//! let archived = dict.contents()?;
//!
//! let queries = vec!["hello"];
//! let results = archived.lookup(&queries, LookupOptions::default())?;
//!
//! for result in results {
//!     println!("Found: {}", result.entry.term.as_str());
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Advanced Lookup with Options
//!
//! ```rust
//! use odict::{OpenDictionary, LookupOptions, LookupStrategy};
//!
//! let dict = OpenDictionary::from_path("dictionary.odict")?;
//! let archived = dict.contents()?;
//!
//! let options = LookupOptions::default()
//!     .insensitive(true)           // Enable case-insensitive fallback
//!     .follow(3)                   // Follow up to 3 redirects
//!     .strategy(LookupStrategy::Split(2)); // Split to minimum 2 chars
//!
//! let queries = vec!["Hello", "compound-word"];
//! let results = archived.lookup(&queries, options)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Handling Redirects
//!
//! ```rust
//! use odict::{OpenDictionary, LookupOptions};
//!
//! let dict = OpenDictionary::from_path("dictionary.odict")?;
//! let archived = dict.contents()?;
//!
//! let options = LookupOptions::default().follow(5);
//! let queries = vec!["abbreviation"]; // Might redirect to full form
//! let results = archived.lookup(&queries, options)?;
//!
//! for result in results {
//!     if let Some(redirect_from) = result.directed_from {
//!         println!("'{}' redirected from '{}'",
//!                  result.entry.term.as_str(),
//!                  redirect_from.term.as_str());
//!     }
//! }
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! ## Split Strategy for Compound Terms
//!
//! ```rust
//! use odict::{OpenDictionary, LookupOptions, LookupStrategy};
//!
//! let dict = OpenDictionary::from_path("dictionary.odict")?;
//! let archived = dict.contents()?;
//!
//! // This will try "compound-word", then "compound", then "word"
//! let options = LookupOptions::default()
//!     .strategy(LookupStrategy::Split(3)); // Minimum 3 characters
//!
//! let queries = vec!["compound-word"];
//! let results = archived.lookup(&queries, options)?;
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
use crate::schema::{ArchivedDictionary, ArchivedEntry, Dictionary, Entry};

use rayon::prelude::*;
use rkyv::option::ArchivedOption;
use std::marker::{Send, Sync};

/// Strategy for matching query terms against dictionary entries.
///
/// This enum defines the different approaches available for finding matches
/// when performing dictionary lookups. Each strategy has different performance
/// characteristics and use cases.
#[derive(Debug, PartialEq, Clone)]
pub enum LookupStrategy {
    /// Match queries exactly against entry terms.
    ///
    /// This is the fastest strategy, performing direct hash map lookups.
    /// It requires the query to exactly match an entry term (case-sensitive
    /// unless the `insensitive` option is enabled).
    ///
    /// **Performance**: O(1) average case
    /// **Use case**: When you know the exact term you're looking for
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::{LookupStrategy, LookupOptions};
    ///
    /// let options = LookupOptions::default()
    ///     .strategy(LookupStrategy::Exact);
    /// ```
    Exact,

    /// Split the query into progressively smaller substrings down to `min_length`,
    /// attempting to match each substring from left to right.
    ///
    /// This strategy is useful for compound words or when you want to find
    /// partial matches. It starts with the full query and progressively
    /// shortens it from the right until a match is found or the minimum
    /// length is reached.
    ///
    /// **Performance**: O(n²) worst case where n is query length
    /// **Use case**: Compound words, partial matching, morphological analysis
    ///
    /// # Algorithm
    ///
    /// For a query "compound-word" with min_length=3:
    /// 1. Try "compound-word" (full query)
    /// 2. Try "compound-wor", "compound-wo", etc.
    /// 3. Try "compound" (if found, move to next segment)
    /// 4. Try "word", "wor" (down to min_length)
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::{LookupStrategy, LookupOptions};
    ///
    /// // Split down to minimum 3 characters
    /// let options = LookupOptions::default()
    ///     .strategy(LookupStrategy::Split(3));
    /// ```
    Split(usize),
}

/// Configuration options for dictionary lookup operations.
///
/// This struct provides fine-grained control over lookup behavior, including
/// redirect following, matching strategies, and case sensitivity. All options
/// have sensible defaults for common use cases.
///
/// # Default Behavior
///
/// - **No redirect following**: Prevents infinite loops and improves performance
/// - **Exact matching**: Most predictable and fastest lookup strategy
/// - **Case-sensitive search**: Preserves linguistic distinctions
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use odict::LookupOptions;
///
/// // Use all defaults
/// let options = LookupOptions::default();
/// ```
///
/// ## Custom Configuration
///
/// ```rust
/// use odict::{LookupOptions, LookupStrategy};
///
/// let options = LookupOptions::default()
///     .follow(5)                           // Follow up to 5 redirects
///     .insensitive(true)                   // Enable case-insensitive fallback
///     .strategy(LookupStrategy::Split(2)); // Split strategy with min length 2
/// ```
#[derive(Debug, Clone)]
pub struct LookupOptions {
    /// Whether to follow see_also links until finding an entry with etymologies.
    pub follow: bool,
    pub strategy: LookupStrategy,

    /// Whether to fall back to case-insensitive search if exact match fails.
    ///
    /// When enabled, if an exact (case-sensitive) match fails, the system
    /// will automatically retry with a lowercase version of the query.
    /// This is useful for handling user input that may have incorrect
    /// capitalization.
    ///
    /// **Note**: The fallback only occurs if the lowercase version differs
    /// from the original query, preventing unnecessary duplicate lookups.
    pub insensitive: bool,
}

impl AsRef<LookupOptions> for LookupOptions {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl LookupOptions {
    /// Construct default lookup options with safe, predictable settings.
    ///
    /// The default configuration prioritizes safety and performance:
    /// - **No redirect following**: Prevents infinite loops and improves performance
    /// - **Exact matching strategy**: Most predictable and fastest lookup method
    /// - **Case-sensitive search**: Preserves linguistic distinctions
    ///
    /// # Returns
    ///
    /// A new `LookupOptions` instance with default settings.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::LookupOptions;
    ///
    /// let options = LookupOptions::default();
    /// assert_eq!(options.follow, None);
    /// assert_eq!(options.strategy, odict::LookupStrategy::Exact);
    /// assert_eq!(options.insensitive, false);
    /// ```
    pub fn default() -> Self {
        Self {
            follow: false,
            strategy: LookupStrategy::Exact,
            insensitive: false,
        }
    }

    pub fn follow(mut self, follow: bool) -> Self {
        self.follow = follow;
        self
    }

    /// Set the matching strategy for query processing.
    ///
    /// The strategy determines how queries are matched against dictionary entries.
    /// Different strategies have different performance characteristics and use cases.
    ///
    /// # Arguments
    ///
    /// * `strategy` - The [`LookupStrategy`] to use for matching
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::{LookupOptions, LookupStrategy};
    ///
    /// // Use exact matching (fastest)
    /// let exact = LookupOptions::default()
    ///     .strategy(LookupStrategy::Exact);
    ///
    /// // Use split strategy for compound words
    /// let split = LookupOptions::default()
    ///     .strategy(LookupStrategy::Split(3));
    /// ```
    pub fn strategy(mut self, strategy: LookupStrategy) -> Self {
        self.strategy = strategy;
        self
    }

    /// Enable or disable case-insensitive fallback matching.
    ///
    /// When enabled, if an exact (case-sensitive) match fails, the system
    /// automatically retries with a lowercase version of the query. This is
    /// useful for handling user input with incorrect capitalization.
    ///
    /// # Arguments
    ///
    /// * `insensitive` - Whether to enable case-insensitive fallback
    ///
    /// # Performance Impact
    ///
    /// - Minimal impact when exact matches succeed
    /// - Adds one additional lookup when exact match fails and query contains uppercase
    /// - No additional lookup if the query is already lowercase
    ///
    /// # Examples
    ///
    /// ```rust
    /// use odict::LookupOptions;
    ///
    /// // Enable case-insensitive fallback
    /// let options = LookupOptions::default().insensitive(true);
    ///
    /// // This will try "Hello" first, then "hello" if not found
    /// // let results = dict.lookup(&["Hello"], options)?;
    /// ```
    pub fn insensitive(mut self, insensitive: bool) -> Self {
        self.insensitive = insensitive;
        self
    }
}

/// Result of a dictionary lookup operation.
///
/// This struct encapsulates the result of a successful lookup, including
/// the matched entry and optional redirect information. It provides context
/// about how the match was found, which is useful for understanding the
/// lookup path and handling redirects.
///
/// # Generic Parameter
///
/// * `E` - The entry type (either `&Entry` or `&ArchivedEntry`)
///
/// # Examples
///
/// ## Basic Usage
///
/// ```rust
/// use odict::{OpenDictionary, LookupOptions};
///
/// let dict = OpenDictionary::from_path("dictionary.odict")?;
/// let archived = dict.contents()?;
/// let queries = vec!["hello"];
/// let results = archived.lookup(&queries, LookupOptions::default())?;
///
/// for result in results {
///     println!("Found: {}", result.entry.term.as_str());
///
///     if let Some(redirect_from) = result.directed_from {
///         println!("  (redirected from: {})", redirect_from.term.as_str());
///     }
/// }
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
///
/// ## Checking for Redirects
///
/// ```rust
/// use odict::{OpenDictionary, LookupOptions};
///
/// # fn example(results: Vec<odict::LookupResult<&odict::ArchivedEntry>>) {
/// for result in results {
///     match result.directed_from {
///         Some(original) => {
///             println!("'{}' is an alias for '{}'",
///                      original.term.as_str(),
///                      result.entry.term.as_str());
///         }
///         None => {
///             println!("Direct match: {}", result.entry.term.as_str());
///         }
///     }
/// }
/// # }
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct LookupResult<E> {
    /// The matched dictionary entry.
    ///
    /// This is the final entry that was found, either through direct matching
    /// or by following redirects. It contains all the linguistic data
    /// (definitions, etymologies, pronunciations, etc.) for the term.
    pub entry: E,

    /// The entry that originally directed to this match via see_also links.
    ///
    /// This field is `Some(entry)` when the result was found by following
    /// a redirect chain, containing the entry that started the redirect.
    /// It's `None` for direct matches without any redirects.
    ///
    /// # Use Cases
    ///
    /// - Displaying "redirected from" information to users
    /// - Understanding alias relationships in the dictionary
    /// - Debugging lookup paths and redirect chains
    /// - Analytics on which redirects are commonly followed
    pub directed_from: Option<E>,
}

/* -------------------------------------------------------------------------- */
/*                                   Methods                                  */
/* -------------------------------------------------------------------------- */

macro_rules! lookup {
    ($tys:ident, $ret:ident, $opt:ident) => {
        impl $tys {
            #[doc = r#"Attempt to find a single entry by term.

This helper supports optional redirect following and an optional
case-insensitive retry (lowercasing the query) when configured.

Returns Some(LookupResult) on a match, or None if not found."#]
            fn find_entry<'a>(
                &'a self,
                follow: &bool,
                insensitive: &bool,
                query: &str,
                directed_from: Option<&'a $ret>,
                path: &mut Vec<String>,
            ) -> crate::Result<$opt<LookupResult<&'a $ret>>> {
                // Check for redirect loop
                if path.contains(&query.to_string()) {
                    // Build the loop chain from the path in order
                    let mut chain = path.clone();
                    chain.push(query.to_string());
                    return Err(crate::Error::RedirectLoop(chain.join(" -> ")));
                }

                // Add current query to path
                path.push(query.to_string());

                // Try exact match first
                if let Some(entry) = self.entries.get(query) {
                    // Follow an alias if follow is true, entry has no etymologies, and see_also exists
                    if *follow && entry.etymologies.is_empty() {
                        if let Option::Some(also) = &entry.see_also.as_ref() {
                            if also.len() > 0 {
                                // Recursively follow the redirect
                                let result = self.find_entry(
                                    follow,
                                    insensitive,
                                    also,
                                    directed_from.or(Some(entry)),
                                    path,
                                );

                                path.pop();

                                return result;
                            }
                        }
                    }

                    path.pop();

                    return Ok($opt::Some(LookupResult {
                        entry,
                        directed_from,
                    }));
                }

                // If insensitive flag is true and exact match failed, try with lowercase
                if *insensitive {
                    let query_lower = query.to_lowercase();

                    // Only try lowercase if it's different from the original query
                    if query_lower != query {
                        if let Ok($opt::Some(result)) =
                            self.find_entry(follow, insensitive, &query_lower, directed_from, path)
                        {
                            path.pop();
                            return Ok($opt::Some(result));
                        }
                    }
                }

                // Remove from path since we're not following any redirects
                path.pop();

                Ok($opt::None)
            }

            #[doc = r#"Perform lookup for a single query using the provided options.

Depending on the strategy, this may return zero or more results."#]
            fn perform_lookup<'a, Options>(
                &'a self,
                query: &str,
                options: Options,
            ) -> crate::Result<Vec<LookupResult<&'a $ret>>>
            where
                Options: AsRef<LookupOptions> + Send + Sync,
            {
                let LookupOptions {
                    follow,
                    strategy,
                    insensitive,
                } = options.as_ref();

                let mut path = Vec::new();

                if let $opt::Some(result) =
                    self.find_entry(follow, insensitive, query, None, &mut path)?
                {
                    return Ok(vec![result]);
                }

                let mut results: Vec<LookupResult<&$ret>> = Vec::new();

                if let LookupStrategy::Split(min_length) = strategy {
                    let chars: Vec<_> = query.chars().collect();
                    let mut start = 0;
                    let mut end = chars.len();

                    while start < end {
                        let substr: String = chars[start..end].iter().collect();
                        let mut substr_path = Vec::new();
                        let maybe_entry = self.find_entry(
                            follow,
                            insensitive,
                            substr.as_str(),
                            None,
                            &mut substr_path,
                        );

                        match maybe_entry {
                            Ok($opt::Some(result)) => {
                                results.push(result);
                                start = end;
                                end = chars.len();
                                continue;
                            }
                            Ok($opt::None) => {
                                if substr.len() <= *min_length {
                                    start = end;
                                    end = chars.len();
                                    continue;
                                }
                            }
                            Err(e) => return Err(e),
                        }

                        end -= 1;
                    }
                }

                Ok(results)
            }

            #[doc = r#"Lookup multiple queries in parallel.

Each query is processed independently with the provided options.

Returns all matches without a guaranteed order.

Examples
--------
```rust
use odict::{OpenDictionary, LookupOptions, LookupStrategy};
# fn demo(dict: &odict::OpenDictionary) -> odict::Result<()> {
let archived = dict.contents()?;
let queries = vec!["hello", "world"];
let options = LookupOptions::default()
    .insensitive(true)
    .strategy(LookupStrategy::Exact);
let results = archived.lookup(&queries, options)?;
# Ok(())
# }
```"#]
            pub fn lookup<'a, 'b, Query, Options>(
                &'a self,
                queries: &'b Vec<Query>,
                options: Options,
            ) -> crate::Result<Vec<LookupResult<&'a $ret>>>
            where
                Query: AsRef<str> + Send + Sync,
                Options: AsRef<LookupOptions> + Send + Sync,
            {
                let results = queries
                    .par_iter()
                    .map(|query| self.perform_lookup(query.as_ref(), &options))
                    .collect::<crate::Result<Vec<_>>>()?
                    .into_iter()
                    .flatten()
                    .collect::<Vec<LookupResult<&'a $ret>>>();

                Ok(results)
            }
        }
    };
}

lookup!(Dictionary, Entry, Option);
lookup!(ArchivedDictionary, ArchivedEntry, ArchivedOption);

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use crate::{lookup::LookupOptions, schema::Dictionary};

    #[test]
    fn test_lookup_follow_limit() {
        // Create a dictionary with a chain of redirects: alias1 -> alias2 -> target
        let xml = r#"
        <dictionary>
            <entry term="target">
                <ety>
                    <sense pos="n">
                        <definition value="The final destination" />
                    </sense>
                </ety>
            </entry>
            <entry term="alias2" see="target" />
            <entry term="alias1" see="alias2" />
        </dictionary>
        "#;

        let dict = Dictionary::from_str(xml).unwrap();

        // Test with follow=false (no following)
        let result = dict
            .lookup(&vec!["alias1"], LookupOptions::default().follow(false))
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "alias1");
        assert_eq!(result[0].directed_from.is_none(), true);

        // Test with follow=true (follows until entry with etymologies found)
        // Should follow alias1 -> alias2 -> target and stop at target since it has etymologies
        let result = dict
            .lookup(&vec!["alias1"], LookupOptions::default().follow(true))
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias1");

        // Test starting from alias2 should also reach target
        let result = dict
            .lookup(&vec!["alias2"], LookupOptions::default().follow(true))
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias2");
    }

    #[test]
    fn test_lookup_redirect_loop_detection() {
        // Create a dictionary with circular redirects: loop1 -> loop2 -> loop1
        let xml = r#"
        <dictionary>
            <entry term="loop1" see="loop2" />
            <entry term="loop2" see="loop1" />
        </dictionary>
        "#;

        let dict = Dictionary::from_str(xml).unwrap();

        // Test that circular redirects are detected and return an error
        let result = dict.lookup(&vec!["loop1"], LookupOptions::default().follow(true));

        assert!(result.is_err());

        let error_message = result.unwrap_err().to_string();

        assert_eq!(
            error_message,
            "Redirect loop detected: loop1 -> loop2 -> loop1"
        );

        assert!(error_message.contains("loop1"));
        assert!(error_message.contains("loop2"));
    }

    #[test]
    fn test_lookup_redirect_case_insensitive() {
        // Create a dictionary with redirects where case differs in queries
        let xml = r#"
        <dictionary>
            <entry term="target">
                <ety>
                    <sense pos="n">
                        <definition value="The final destination" />
                    </sense>
                </ety>
            </entry>
            <entry term="alias" see="target" />
        </dictionary>
        "#;

        let dict = Dictionary::from_str(xml).unwrap();

        // Test case insensitive redirect following with uppercase query
        let result = dict
            .lookup(
                &vec!["ALIAS"],
                LookupOptions::default().follow(true).insensitive(true),
            )
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias");

        // Test case insensitive redirect following with mixed case query
        let result = dict
            .lookup(
                &vec!["Alias"],
                LookupOptions::default().follow(true).insensitive(true),
            )
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
        assert_eq!(result[0].directed_from.is_some(), true);
        assert_eq!(result[0].directed_from.unwrap().term, "alias");

        // Test that case sensitive mode doesn't find mismatched case
        let result = dict
            .lookup(
                &vec!["ALIAS"],
                LookupOptions::default().follow(true).insensitive(false),
            )
            .unwrap();

        assert_eq!(result.len(), 0);

        // Test that exact case match works in case sensitive mode
        let result = dict
            .lookup(
                &vec!["alias"],
                LookupOptions::default().follow(true).insensitive(false),
            )
            .unwrap();

        assert_eq!(result.len(), 1);
        assert_eq!(result[0].entry.term, "target");
    }
}
