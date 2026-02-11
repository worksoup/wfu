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

use crate::{DerefHolder, FmtBy, FmtHandler, TargetHolder};

#[derive(Debug)]
pub struct FmtWithWrapper<'f, F>(pub &'f F);

impl<'f, F> ::core::clone::Clone for FmtWithWrapper<'f, F> {
    #[inline]
    fn clone(&self) -> Self {
        *self
    }
}
impl<'f, F> ::core::marker::Copy for FmtWithWrapper<'f, F> {}

impl<'f, T, F> FmtHandler<T> for FmtWithWrapper<'f, F>
where
    F: Fn(&T, &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result,
    T: ?::core::marker::Sized,
{
    #[inline(always)]
    fn fmt(&self, data: &T, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        (self.0)(data, f)
    }
}

pub trait FmtWith: FmtBy {
    #[inline(always)]
    fn fmt_with<
        's,
        'f,
        FmtWith: Fn(&Self, &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result,
    >(
        &'s self,
        f: &'f FmtWith,
    ) -> TargetHolder<'s, Self, FmtWithWrapper<'f, FmtWith>> {
        self.fmt_by(FmtWithWrapper(f))
    }
    #[inline(always)]
    fn fmt_deref_with<
        'f,
        FmtWith: for<'fmt> Fn(&Self, &mut ::core::fmt::Formatter<'fmt>) -> ::core::fmt::Result,
    >(
        self,
        f: &'f FmtWith,
    ) -> DerefHolder<Self, FmtWithWrapper<'f, FmtWith>>
    where
        Self: ::core::ops::Deref + ::core::marker::Sized,
    {
        self.fmt_deref_by(FmtWithWrapper(f))
    }
}
impl<T: ?::core::marker::Sized> FmtWith for T {}

#[cfg(test)]
mod tests {
    use std::fmt::Formatter;

    use super::*;

    #[test]
    fn test_fmt_with() {
        let value = 42;

        // 测试使用fmt_with方法
        let holder = value.fmt_with(&|v, f| write!(f, "Value: {}", v));
        assert_eq!(format!("{}", holder), "Value: 42");
        assert_eq!(format!("{:?}", holder), "Value: 42");

        // 测试不同格式化trait
        let holder = value.fmt_with(&|v, f| write!(f, "0x{:x}", v));
        assert_eq!(format!("{:x}", holder), "0x2a");
        assert_eq!(format!("{}", holder), "0x2a"); // 格式化结果只与闭包有关。
        assert_ne!(format!("{:X}", holder), "0x2A"); // 格式化结果只与闭包有关。
    }

    #[test]
    fn test_closure_lifetimes() {
        // 测试闭包中的生命周期处理
        let prefix = "Prefix: ";
        let value = 42;

        let binding = move |v: &i32, f: &mut Formatter<'_>| write!(f, "{}{}", prefix, v);
        let holder = value.fmt_with(&binding);

        assert_eq!(format!("{}", holder), "Prefix: 42");
    }
}
