#![allow(unused)]

use std::{time::Duration, future::Future, ops::Range};

use rand::{random, thread_rng, seq::SliceRandom, Rng};
use tokio::{task::JoinHandle, time};

mod pagination;
pub use pagination::Pagination;

pub mod math;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Color {
    Hex(String),
    Rgb(u32, u32, u32),
    Integer(u32),
}

impl Color {
    pub const WHITE: Color = Color::Rgb(255, 255, 255);
    pub const VERY_LIGHT_GRAY: Color = Color::Rgb(213, 212, 217);
    pub const LIGHT_GRAY: Color = Color::Rgb(157, 156, 161);
    pub const GRAY: Color = Color::Rgb(101, 101, 102);
    pub const DARK_GRAY: Color = Color::Rgb(73, 73, 74);
    pub const ALMOST_BLACK: Color = Color::Rgb(32, 32, 33);
    pub const BLACK: Color = Color::Rgb(1, 1, 1);

    pub const VERY_LIGHT_BLUE: Color = Color::Rgb(189, 198, 240);
    pub const LIGHT_BLUE: Color = Color::Rgb(168, 183, 230);
    pub const BLUE: Color = Color::Rgb(108, 139, 235);
    pub const DARK_BLUE: Color = Color::Rgb(67, 104, 217);
    pub const BLURPLE: Color = Color::Rgb(131, 118, 204);
    pub const VERY_DARK_BLUE: Color = Color::Rgb(34, 31, 166);

    pub const VERY_LIGHT_RED: Color = Color::Rgb(242, 162, 170);
    pub const LIGHT_RED: Color = Color::Rgb(230, 106, 118);
    pub const RED: Color = Color::Rgb(240, 41, 60);
    pub const DARK_RED: Color = Color::Rgb(148, 27, 38);
    pub const VERY_DARK_RED: Color = Color::Rgb(69, 5, 11);

    pub const VERY_LIGHT_GREEN: Color = Color::Rgb(177, 240, 192);
    pub const LIGHT_GREEN: Color = Color::Rgb(126, 242, 154);
    pub const GREEN: Color = Color::Rgb(56, 242, 102);
    pub const DARK_GREEN: Color = Color::Rgb(24, 161, 58);
    pub const VERY_DARK_GREEN: Color = Color::Rgb(6, 74, 23);

    pub const VERY_LIGHT_YELLOW: Color = Color::Rgb(242, 209, 116);
    pub const LIGHT_YELLOW: Color = Color::Rgb(240, 209, 84);
    pub const YELLOW: Color = Color::Rgb(245, 207, 37);
    pub const DARK_YELLOW: Color = Color::Rgb(199, 165, 14);
    pub const VERY_DARK_YELLOW: Color = Color::Rgb(120, 98, 34);

    pub const LIGHT_BEIGE: Color = Color::Rgb(240, 200, 177);
    pub const BEIGE: Color = Color::Rgb(245, 194, 164);
    pub const LIGHT_ORANGE: Color = Color::Rgb(242, 147, 87);
    pub const ORANGE: Color = Color::Rgb(242, 113, 31);
    pub const DARK_ORANGE: Color = Color::Rgb(207, 84, 6);
    pub const BROWN: Color = Color::Rgb(145, 84, 45);
    pub const DARK_BROWN: Color = Color::Rgb(61, 31, 12);

    pub fn to_u32(&self) -> u32 {
        match self {
            Self::Hex(code) => u32::from_str_radix(code.replace("#", "").as_str(), 16).unwrap_or(0),
            Self::Rgb(r, g, b) => (r << 16u32) + (g << 8u32) + b,
            Self::Integer(int) => *int,
        }
    }
}

pub fn set_tokio_timeout<T>(duration: Duration, future: T) -> JoinHandle<T::Output> 
where
    T: Future + Send + 'static,
    T::Output: Send + 'static,
{
    tokio::spawn(async move {
        tokio::time::sleep(duration).await;

        future.await
    })
}

// https://users.rust-lang.org/t/setinterval-in-rust/41664
pub fn set_tokio_interval<F, Fut>(mut f: F, duration: Duration)
where
    F: Send + 'static + FnMut() -> Fut,
    Fut: Future<Output = ()> + Send + 'static,
{
    // Create stream of intervals.
    let mut interval = time::interval(duration);
    
    tokio::spawn(async move {
        // Skip the first tick at 0ms.
        interval.tick().await;
        loop {
            // Wait until next tick.
            interval.tick().await;
            // Spawn a task for this tick.
            tokio::spawn(f());
        }
    });
}