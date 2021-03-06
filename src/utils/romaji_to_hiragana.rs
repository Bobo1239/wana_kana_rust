use to_kana::*;
use options::Options;

/// Convert [Romaji](https://en.wikipedia.org/wiki/Romaji) to [Hiragana](https://en.wikipedia.org/wiki/Hiragana)
///
/// @param  {String} [input=''] text
///
/// @param  {Object} options used internally to pass along default options
///
/// romaji_to_hiragana("hiragana", Options::default())
/// => "ひらがな"

pub fn romaji_to_hiragana(input: &str, options: Options) -> String {
    let text = input.to_lowercase(); // ensure hiragana
    to_kana_with_opt(&text, options)
}
