/*
 * atlas-core by Gipson62
 *
 * The core of a tool designed to help you make programming language
 *
 * There will be a lexer macro that build up a fully fledged lexer for you with no efforts.
 * It's currently in development, more information coming soon
*/

//! # atlas-Core
//!
//! `atlas-core` is the foundational library for a language creation tool designed to assist users in developing languages.
//! This core library currently serves as the building block for the creation of Atlas77, a functional programming language.
//!
//! Currently, it's only purpose is to generate a Lexer and the way to do it is pretty straightforward

/// Contain a powerful macro to generate a fully fledge lexer tailored to the user needs
pub mod lexer;
mod tests;
/// TODO
pub mod utils;

#[doc = "Used to import the base set of features of this tool"]
pub mod prelude {
    pub use crate::keywords;
    pub use crate::lexer;
    pub use crate::lexer::lexer_state::LexerState;
    pub use crate::lexer_builder;
    pub use crate::map;
    pub use crate::tokens;
    pub use crate::utils::{case::Case, span::*};
}

/// A macro to create a `HashMap` with the given key-value pairs.
#[macro_export]
macro_rules! map {
    ($name:ident, &key: ty, &val: ty) => {
        let mut $name: HashMap<&key, &val> = HashMap::new();
    };
    ($($key:expr => $val:expr),* $(,)?) => {
        {
            let mut map = HashMap::new();
            $(map.insert($key, $val);)*
            map
        }
    }
}
