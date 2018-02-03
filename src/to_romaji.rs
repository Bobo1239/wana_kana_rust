use constants::{TO_ROMAJI};

use utils::get_chunk::*;
use utils::katakana_to_hiragana::*;
use is_katakana::*;
use options::Options;

/**
 * Convert kana to romaji
 * @param  {String} kana text input
 * @param  {DefaultOptions} [options=default_options]
 * @return {String} converted text
 * @example
 * to_romaji('ひらがな　カタカナ')
 * // => 'hiragana katakana'
 * to_romaji('ひらがな　カタカナ', { upcase_katakana: true })
 * // => 'hiragana KATAKANA'
 */
fn to_romaji(kana = '', options: Options) {
  let config = options;
  let len = kana.length;
  // Final output array
  let roma = vec![];
  // Position in the string that is being evaluated
  let cursor = 0;
  let max_chunk = 2;
  let chunk_size = 2;
  let chunk = '';
  let romachar: char;
  let next_char_is_double_consonant;

  while (cursor < len) {
    chunk_size = std::cmp::min(max_chunk, len - cursor);
    let convert_this_chunk_to_uppercase = false;
    while (chunk_size > 0) {
      chunk = get_chunk(kana, cursor, cursor + chunk_size);
      if (is_katakana(chunk)) {
        convert_this_chunk_to_uppercase = config.upcase_katakana;
        chunk = katakana_to_hiragana(chunk);
      }
      // special case for small tsus
      if (chunk.chars().nth(0).unwrap() == 'っ' && chunk_size == 1 && cursor < (len - 1)) {
        next_char_is_double_consonant = true;
        romachar: char;
        break;
      }

      roma_char = TO_ROMAJI[chunk];

      if ((roma_char != null) && next_char_is_double_consonant) {
        roma_char = roma_char.chars().nth(0).unwrap().concat(roma_char);
        next_char_is_double_consonant = false;
      }
      // console.log(`${cursor}x${chunk_size}:${chunk} => ${roma_char}`);
      if (roma_char != null) {
        break;
      }
      chunk_size -= 1;
    }
    if (roma_char == null) {
      // Passthrough undefined values
      roma_char = chunk;
    }

    if (convert_this_chunk_to_uppercase) {
      roma_char = roma_char.to_upper_case();
    }
    roma.push(roma_char);
    cursor += chunk_size || 1;
  }
  return roma.join('');
}


