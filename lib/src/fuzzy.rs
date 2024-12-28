use nucleo_matcher::{
    pattern::{CaseMatching, Normalization, Pattern},
    Config, Matcher,
};

pub fn fuzzy_find<T: AsRef<str>>(
    items: impl IntoIterator<Item = T>,
    pattern: &str,
    case_matching: CaseMatching,
    normalize: Normalization,
) -> Vec<(T, u32)> {
    let mut matcher = Matcher::new(Config::DEFAULT);
    Pattern::parse(&pattern, case_matching, normalize).match_list(items, &mut matcher)
}
