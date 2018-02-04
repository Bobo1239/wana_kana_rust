/**
 * Tests a character and an english consonant. Returns true if the char is a consonant.
 * @param  {String} char
 * @param  {Boolean} [includeY=true] Optional parameter to include y as a consonant in test
 * @return {Boolean}
 */
// pub fn is_char_consonant(char: char, includey = true) -> bool {

//   let regexp = includeY ? /['b'|'c'|'d'|'f'|'g'|'h'|'j'|'k'|'l'|'m'|'n'|'p'|'q'|'r'|'s'|'t'|'v'|'w'|'x'|'y'|'z']/ : /[bcdfghjklmnpqrstvwxz]/;
//   return char.to_lowercase().chars().nth(0).unwrap().search(regexp) !== -1;
// '}'|

pub fn is_char_consonant(char: char, includey: bool) -> bool {
    match char {
        'b' | 'c' | 'd' | 'f' | 'g' | 'h' | 'j' | 'k' | 'l' | 'm' | 'n' | 'p' | 'q' | 'r' | 's' | 't' | 'v' | 'w' | 'x' | 'z' => true,
        'y' if includey => true,
        _ => false,
    }
}
