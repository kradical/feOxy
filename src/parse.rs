pub trait Parser {
    /// Returns if the string still has characters in it.
    fn has_chars(&self) -> bool;

    /// Returns the current first character without consuming it.
    fn peek(&self) -> Option<char>;

    /// Consumes the first character and returns it.
    fn consume(&mut self) -> Option<char>;

    /// Consumes characters until condition is false or there are no more chars left.
    /// Returns a string of the consumed characters.
    fn consume_while<F>(&mut self, condition: F) -> String where F : Fn(char) -> bool;
}