//! Hi, my name is Aden, a backend programmer in Shanghai/Chain, who had a big
//! interest in technology, digital products.
//!
//! I have been using Golang as my main programming-language for two years, and
//! Rust is my second favorite language. I'm looking for a backend-development
//! job which using golang and rust.
//!
//! You can find me on [GitHub](https://github.com/HaHadaxigua)
//!
//! You can also send me an
//! [email](hahadaxigua@gmail.com).

// Jobs

/// Bachelor of BNetwork Engineering from 2017 to 2021, in Nantong University
pub mod education {}

/// Junior Golang backend developer.
pub mod current {}

// Personal Projects

/// A CLI tool that allows you to create a temporary new Rust project using
/// cargo with already installed dependencies.
/// [\[Repository\]](<https://github.com/rustminded/cargo-temp>)
#[macro_export]
macro_rules! cargo_temp {
    () => {};
}

/// A crate to create and handle processes on Windows (similar too `std::process`).
/// [\[Repository\]](<https://github.com/yozhgoor/CreateProcessW>)
#[macro_export]
macro_rules! create_process_w {
    () => {};
}

/// My vision of the Rust Programming Language as an introduction to Rust.
/// [\[Repository\]](<https://github.com/yozhgoor/intro-to-rust>)
#[macro_export]
macro_rules! intro_to_rust {
    () => {};
}

/// Yet Another To-Do App in your Terminal. (Under development)
/// [\[Repository\]](<https://github.com/yozhgoor/tui-do>)
#[macro_export]
macro_rules! tui_do {
    () => {};
}

/// A Yew and Yewprint playground using wasm-run.
/// [\[Repository\]](<https://github.com/yozhgoor/yewprint-playground>)
#[macro_export]
macro_rules! yewprint_playground {
    () => {};
}

// Technical skills

/// Ubuntu, Darwin.
pub const OS: () = ();

/// Golang, Rust, Java.
pub const PROGRAMMING_LANGUAGES: () = ();

/// Chinese(5.Native) English (3. Minimum Professional Proficiency).
pub const SPOKEN_LANGUAGES: () = ();

/// Git, Docker, .
pub const TECHNOLOGIES: () = ();

// Personal traits

pub trait Autodidact {}

pub trait Enthusiast {}

pub trait Flexible {}

pub trait Passionate {}

pub trait TeamPlayer {}

pub trait WantToLearn {}
