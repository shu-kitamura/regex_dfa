use thiserror::Error;

/// パースエラーを表す型
///
/// 正規表現パターンの解析（パース）中に発生するエラーを表現する
/// 各エラーケースは、入力されたパターンのどの部分でどのような問題があったかを示すために、
/// 位置情報や不正な文字などの補足情報を含む。
#[derive(Debug, Error, PartialEq)]
pub enum ParseError {
    #[error("ParseError: invalid escape : position = {0}, character = '{1}'")]
    InvalidEscape(usize, char),
    #[error("ParseError: invalid right parenthesis : position = {0}")]
    InvalidRightParen(usize),
    #[error("ParseError: no previous expression : position = {0}")]
    NoPrev(usize),
    #[error("ParseError: no right parenthesis")]
    NoRightParen,
    #[error("ParseError: empty expression")]
    Empty,
}
