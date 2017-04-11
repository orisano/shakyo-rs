//! # Rust標準ライブラリ
//!
//! Rust標準ライブラリはポータグルなRustソフトウェアをビルドするために不可欠な
//! ランタイム関数を提供する。

fn main() {
    // 行コメントは「//」以降のすべての文字であり、行末まで続く

    let x = 5; // this is also a line comment.

    // もし何かのために長い説明を書くのであれば、行コメントを複数行に渡って書くこと
    // ができる。//とコメントの間にスペースを置くことで、より読みやすくなる
}

/// 与えられた数値に1を加える
///
/// # Examples
/// ```
/// let five = 5;
///
/// assert_eq!(6, add_one(5));
/// # fn add_one(x: i32) -> i32 {
/// #       x + 1
/// # }
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}
