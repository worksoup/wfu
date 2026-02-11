// MIT License
//
// Copyright (c) 2026 worksoup <https://github.com/worksoup/>
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use crate::FmtHandler;

/// 用于连接迭代器元素的格式化处理器
///
/// 这个结构体是一个适配器，它通过引用来迭代集合（不会克隆集合）
///
/// # 类型参数
/// - `Delim`: 分隔符类型，必须实现 [`Display`](::core::fmt::Display) 和 [`Copy`](::core::marker::Copy) ( 可以是引用 ) 特型。
///
/// # 示例
/// ``` rust
/// use wfu::{FmtBy, Joined};
/// let vec = vec!["a", "b", "c"];
/// let proxy = vec.fmt_by(Joined(", "));
/// assert_eq!(format!("{}", proxy), "a, b, c");
/// ```
/// # 注意
/// - 要求 &Iter 实现 [`IntoIterator`];
/// - 对于 &Iter 没有实现 [`IntoIterator`] 的类型（如 [`Range`](::core::ops::Range)），请使用 [`CloneIterJoined`], 它要求 Iter 实现 [`Clone`].
#[derive(Debug, Clone, Copy, Default)]
pub struct Joined<Delim>(pub Delim)
where
    Delim: ::core::fmt::Display;

impl<Iter, Delim> FmtHandler<Iter> for Joined<Delim>
where
    for<'a> &'a Iter: ::core::iter::IntoIterator,
    Iter: ?::core::marker::Sized,
    for<'a> <&'a Iter as ::core::iter::IntoIterator>::Item: ::core::fmt::Display,
    Delim: ::core::fmt::Display + ::core::marker::Copy,
{
    #[inline]
    fn fmt(&self, data: &Iter, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        CloneIterJoined(self.0).fmt(&data, f)
    }
}

/// 用于连接迭代器元素的格式化处理器，适用于需克隆才能迭代的类型（如 [`Range`](::core::ops::Range)）
///
/// 这个结构体是一个适配器，它通过克隆来迭代集合
///
/// # 类型参数
/// - `Delim`: 分隔符类型，必须实现 [`Display`](::core::fmt::Display) 和 [`Copy`](::core::marker::Copy) ( 可以是引用 ) 特型。
///
/// # 示例
/// ``` rust
/// use wfu::{FmtBy, CloneIterJoined};
/// let range = 1..5;
/// let proxy = range.fmt_by(CloneIterJoined(", "));
/// assert_eq!(format!("{}", proxy), "1, 2, 3, 4");
/// ```
/// # 注意
/// - 要求 Iter 实现 [`Clone`];
/// - 对于 &Iter 实现了 [`IntoIterator`] 的类型，可以使用 [`Joined`], 它不会克隆 Iter.
#[derive(Debug, Clone, Copy, Default)]
pub struct CloneIterJoined<Delim>(pub Delim)
where
    Delim: ::core::fmt::Display;

impl<Iter, Delim> FmtHandler<Iter> for CloneIterJoined<Delim>
where
    Iter: ::core::iter::IntoIterator + ::core::clone::Clone,
    <Iter as ::core::iter::IntoIterator>::Item: ::core::fmt::Display,
    Delim: ::core::fmt::Display + ::core::marker::Copy,
{
    fn fmt(&self, data: &Iter, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        use ::core::fmt::Display;
        let mut iter = data.clone().into_iter();
        let Some(first) = iter.next() else {
            return Ok(());
        };
        first.fmt(f)?;
        for item in iter {
            self.0.fmt(f)?;
            item.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::FmtBy;

    #[test]
    fn test_joined_proxy() {
        // 测试基础功能
        let vec = vec!["a", "b", "c"];
        let proxy = vec.fmt_by(Joined(", "));
        assert_eq!(format!("{}", proxy), "a, b, c");

        // 测试字符分隔符
        let proxy_char = vec.fmt_by(Joined('-'));
        assert_eq!(format!("{}", proxy_char), "a-b-c");

        // 测试空迭代器
        let empty_vec: Vec<i32> = Vec::new();
        let proxy_empty = empty_vec.fmt_by(Joined(", "));
        assert_eq!(format!("{}", proxy_empty), "");

        // 测试单个元素
        let single_vec = vec![42];
        let proxy_single = single_vec.fmt_by(Joined(", "));
        assert_eq!(format!("{}", proxy_single), "42");
    }

    #[test]
    fn test_joined_proxy_ref() {
        // 测试基础功能
        let vec = vec!["a", "b", "c"];
        let delim = ", ".to_string();
        let proxy = vec.fmt_by(Joined(&delim));
        assert_eq!(format!("{}", proxy), "a, b, c");
    }

    /// 测试Joined与不同容器类型的兼容性
    #[test]
    fn test_joined_with_different_containers() {
        // 测试Vec
        let vec = vec!["x", "y", "z"];
        let proxy_vec = vec.fmt_by(Joined("-"));
        assert_eq!(format!("{}", proxy_vec), "x-y-z");

        // 测试数组
        let array = ["x", "y", "z"];
        let proxy_array = array.fmt_by(Joined("-"));
        assert_eq!(format!("{}", proxy_array), "x-y-z");

        // 测试切片
        let slice: &[&str] = &["x", "y", "z"];
        let proxy_slice = slice.fmt_by(Joined("-"));
        assert_eq!(format!("{}", proxy_slice), "x-y-z");

        // 对于 &Iter 没有实现 IntoIterator 的类型，使用 CloneIterJoined.
        let range = 1..5;
        let proxy_range = range.fmt_by(CloneIterJoined("-"));
        assert_eq!(format!("{}", proxy_range), "1-2-3-4");
    }
}
