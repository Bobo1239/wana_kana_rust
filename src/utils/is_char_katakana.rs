import {
  KATAKANA_START,
  KATAKANA_END,
} from '../constants';

use utils::is_char_inRange::*;

/**
 * Tests a character. Returns true if the character is [Katakana](https://en.wikipedia.org/wiki/Katakana).
 * @param  {String} char character string to test
 * @return {Boolean}
 */
pub fn is_char_katakana(char: char) -> bool {
  return is_char_inRange(char, KATAKANA_START, KATAKANA_END);
}

export default is_char_katakana;
