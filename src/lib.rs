#![feature(plugin)]
#![plugin(phf_macros)]
#![feature(slice_patterns)]

extern crate phf;
extern crate regex;

#[macro_use]
extern crate lazy_static;

mod is_kanji;
mod is_kana;
mod utils;
mod constants;
mod options;

#[cfg(test)]
mod tests {
    use is_kanji::*;
    #[test]
    fn it_works() {
        assert_eq!(is_kanji("刀"), true);
    }

    // #[test]
    // fn convert() {
    //     use std::fs::rename;
    //     use std::fs;
    //     use regex::Regex;

    //     let re = Regex::new(r"([A-Z])").unwrap();

    //     let paths = fs::read_dir("src/utils").unwrap();

    //     for path in paths {
    //         let path = path.unwrap().path();
    //         let path_str = path.to_string_lossy();
    //         println!("Name: {}", path_str);

    //         let after = re.replace_all(&path_str, "_$1").to_string().to_lowercase();
    //         println!("Name: {}", after);
    //         fs::rename(path_str.to_string(), after).unwrap();
    //         // var result = text.replace(/([A-Z])/g, "_$1");
    //     }
    // }
}
