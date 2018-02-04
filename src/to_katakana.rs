use is_romaji::*;
use is_mixed::*;
use utils::hiragana_to_katakana::*;
use utils::romaji_to_hiragana::*;
use options::Options;

/**
 * Convert input to [Katakana](https://en.wikipedia.org/wiki/Katakana)
 * @param  {String} [input=''] text
 * @param  {DefaultOptions} [options=default_options]
 * @return {String} converted text
 * @example
 * to_katakana('toukyou, おおさか')
 * // => 'トウキョウ、　オオサカ'
 * to_katakana('only かな', { pass_romaji: true })
 * // => 'only カナ'
 * to_katakana('wi')
 * // => 'ウィ'
 * to_katakana('wi', { use_obsolete_kana: true })
 * // => 'ヰ'
*/
pub fn to_katakana(input: &str, options: Options) -> String {
    let config = options;
    if config.pass_romaji {
        return hiragana_to_katakana(input);
    }
    if is_romaji(input) || is_mixed(input) {
        let romaji = romaji_to_hiragana(input, config);
        return hiragana_to_katakana(&romaji);
    }
    return hiragana_to_katakana(input);
}

#[test]
fn check_to_katakana() {
    assert_eq!(
        to_katakana("toukyou,おおさか", Options::default()),
        "トウキョウ、オオサカ"
    );
    assert_eq!(
        to_katakana(
            "only かな",
            Options {
                pass_romaji: true,
                ..Default::default()
            }
        ),
        "only カナ"
    );
    assert_eq!(to_katakana("wi", Options::default()), "ウィ");
    assert_eq!(
        to_katakana(
            "wi",
            Options {
                use_obsolete_kana: true,
                ..Default::default()
            }
        ),
        "ヰ"
    );
}
