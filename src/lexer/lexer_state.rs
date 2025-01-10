use crate::prelude::*;
use std::{iter::Peekable, str::Chars};

/// `LexerState` represents the state of the lexer during the process of
/// tokenizing a source string. It keeps track of the current position within
/// the string and provides methods to iterate over and peek at the upcoming characters.
///
/// The lexer state is generic over the lifetime `'lex`, which represents the
/// lifetime of the source string being tokenized.
#[derive(Clone, Debug)]
pub struct LexerState<'lex> {
    /// The current position in the source string, represented as a `BytePos`.
    /// This tracks the byte offset within the string, rather than the character position,
    /// which is important for handling multi-byte UTF-8 characters.
    pub current_pos: BytePos,

    /// The iterator over the characters of the source string, wrapped in a `Peekable`
    /// to allow lookahead operations. `Peekable` enables efficient peeking at the
    /// next character without advancing the iterator.
    txt: Peekable<Chars<'lex>>,
    pub path: &'lex str,
}

impl<'lex> LexerState<'lex> {
    /// Creates a new `LexerState` initialized with the starting position and the source string.
    ///
    /// # Parameters
    /// - `current_pos`: The initial position in the source string, typically set to the start (0).
    /// - `txt`: A reference to the source string that the lexer will process.
    ///
    /// # Returns
    /// A new `LexerState` instance ready for use.
    pub fn new(current_pos: BytePos, txt: &'lex str, path: &'lex str) -> Self {
        Self {
            current_pos,
            txt: txt.chars().peekable(),
            path,
        }
    }

    /// Advances the iterator to the next character in the source string, updating the
    /// current position accordingly.
    ///
    /// # Returns
    /// - `Some(char)`: The next character if one exists.
    /// - `None`: If the iterator has reached the end of the source string.
    pub fn next(&mut self) -> Option<char> {
        self.current_pos = self.current_pos.shift_by(1);
        self.txt.next()
    }

    /// Peeks at the next character in the source string without advancing the iterator.
    ///
    /// # Returns
    /// - `Some(&char)`: A reference to the next character if one exists.
    /// - `None`: If the iterator has reached the end of the source string.
    ///
    /// Peeking allows lookahead operations, which are often necessary in lexical analysis
    /// to decide how to parse the next tokens without consuming them.
    pub fn peek(&mut self) -> Option<&char> {
        self.txt.peek()
    }
}
