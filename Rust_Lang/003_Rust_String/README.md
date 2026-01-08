# link

<hr />

# Rust #8: Strings 
- https://dev.to/cthutu/rust-8-strings-53o

# Rust, C, and C++ strings | The Big Book of Rust Interop
- https://nrc.github.io/big-book-ffi/reference/strings.html

- C하고 러스트 잘 비교한글
  - [C vs Rust비교가 잘 되게 잘 정리됨.](https://kornel.ski/rust-c-speed) 
  - In short:
    - Rust's abstractions are a double-edged sword. They can hide suboptimal code, but also make it easier to make algorithmic improvements and take advantage of highly optimized libraries.
    - I'm never worried that I'm going to hit a performance dead-end with Rust. There's always the unsafe escape hatch that allows very low-level optimizations (and it's not needed often).
    - Fearless concurrency is real. The occasional awkwardness of the borrow checker pays off in making parallel programming practical.

# How Long Is a String? | A Rust Brain Teaser | Herbert Wolverson | Mar 23, 2022
- https://medium.com/pragmatic-programmers/how-long-is-a-string-c25a086afe31

# Does `OsString`’s `From<String>` allocate? | post by zeta12ti on May 19, 2020 | zeta12ti
- https://users.rust-lang.org/t/does-osstring-s-from-string-allocate/42879

# HacksNews글
- [In Rust, strings are always valid UTF8, and attempting to create a string with invalid UTF8 will panic at runtime ](https://news.ycombinator.com/item?id=40382524)

# Unicode
- https://unicode-rs.github.io/unicode-segmentation/unicode_segmentation/index.html
- [A Pitfall for Beginners in Rust: Misunderstanding Strings and Unicode ](https://www.reddit.com/r/rust/comments/1gtz615/a_pitfall_for_beginners_in_rust_misunderstanding/)

# 레딧글
- [Why do strings have to be valid UTF-8?](https://www.reddit.com/r/rust/comments/1jgxh3y/why_do_strings_have_to_be_valid_utf8/)

# 잘 정리된 외국글
- [Why Rust strings seem hard April 13, 2021](https://www.brandons.me/blog/why-rust-strings-seem-hard)
- [What’s So Special About Strings in Rust? | Rahul Patil | Jun 23, 2024](https://medium.com/@rahulptl1997/whats-so-special-about-strings-in-rust-56da812aac6d)
- [String Types in Rust March 27, 2016 | Topics: Rust](https://www.alilleybrinker.com/blog/string-types-in-rust/)
- [Understanding The UTF-8 Encoding Algorithm in Rust | April 05, 2025](https://css-plus.com/2025/understanding-the-utf-8-encoding-algorithm-in-rust/)
- [Answering Rust Strings, UTF-8, Variable Encoding, Clone On Write (COW), String Trait methods, and why Strings can't be indexed Explore Strings in general and how Rust protects us from invalid string operations.An In-Depth Introduction to Strings in Rust and Their Distinctions Updated January 12, 2024](https://sanjeevi.hashnode.dev/answering-rust-strings-utf-8-variable-encoding-clone-on-write-cow-string-trait-methods-and-why-strings-cant-be-indexed)
- [You probably don't need to validate UTF-8 strings | Written 2024-05-16](https://viralinstruction.com/posts/utf8/)

# Rust커뮤티니글
- [How to return a simple str (Unsized UTF-8 sequence of Unicode string slices) from a function](https://users.rust-lang.org/t/how-to-return-a-simple-str-unsized-utf-8-sequence-of-unicode-string-slices-from-a-function/43957)
  - [(유튜브 영상)140816 Memory Safety in Rust ibnatfatil (Memory Safety in Rust by Nicholas Matsakis
)](https://youtu.be/WQbg6ZMQJvQ?si=qkzlEtebKn-tLOB7) 

# Rust Tutorial같은 외국글
- [Rust String Fundamentals: Memory Layout and UTF-8 Encoding | Last updated: January 03, 2025](https://www.slingacademy.com/article/rust-string-fundamentals-memory-layout-and-utf-8-encoding/)
- [Unlocking the Power of Strings in Rust | In this article, we’ll take a closer look at how Rust handles text, from different ways to store and manipulate it to more advanced features like special formatting and memory efficiency.”
Uday Hiwarale · Dec 23, 2024](https://medium.com/rustycrab/unlocking-the-power-of-strings-in-rust-4193ad56f8db)

# UTF-8이 아닌 UTF-16으로 신기하네.
- [UTF-16 Encoding : Rust | September 9, 2025](https://mojoauth.com/character-encoding-decoding/utf-16-encoding--rust/#introduction-to-utf-16)
