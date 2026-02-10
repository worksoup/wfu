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

#[derive(Debug, Clone, Copy, Default)]
pub struct Repeat(pub usize);

impl<T> FmtHandler<T> for Repeat
where
    T: ?::core::marker::Sized + ::core::fmt::Display,
{
    fn fmt(&self, data: &T, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        for _ in 0..self.0 {
            data.fmt(f)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use crate::FmtBy;

    use super::*;

    #[test]
    fn test_repeat_proxy() {
        // 测试基本功能
        let s = "abc";
        let proxy = s.fmt_by(Repeat(3));
        assert_eq!(format!("{}", proxy), "abc".repeat(3));

        // 测试数字
        let num = 42;
        let proxy_num = num.fmt_by(Repeat(5));
        assert_eq!(format!("{}", proxy_num), "4242424242");

        // 测试0次和1次
        assert_eq!(format!("{}", s.fmt_by(Repeat(0))), "");
        assert_eq!(format!("{}", s.fmt_by(Repeat(1))), "abc");

        // 测试多次重复
        let proxy_many = s.fmt_by(Repeat(10));
        assert_eq!(format!("{}", proxy_many).len(), "abc".len() * 10);
    }
}
