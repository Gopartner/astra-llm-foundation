// pub fn add(left: u64, right: u64) -> u64 {
//     left + right
// }

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn it_works() {
//         let result = add(2, 2);
//         assert_eq!(result, 4);
//     }
// }
pub mod normalizer;
pub mod cleaner;
pub mod pretokenizer;
pub mod byte_encoder;
pub mod bpe;
pub mod vocab;
pub mod encoder;
pub mod decoder;
pub mod special_tokens;
pub mod utils;