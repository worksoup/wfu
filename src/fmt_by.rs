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

use crate::{DerefHolder, TargetHolder};
pub trait FmtHandler<Inner: ?::core::marker::Sized>: ::core::marker::Copy {
    fn fmt(&self, inner: &Inner, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result;
}
impl<Inner: ?::core::marker::Sized, T: FmtHandler<Inner>> FmtHandler<Inner> for &T {
    #[inline]
    fn fmt(&self, inner: &Inner, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        (*self).fmt(inner, f)
    }
}
pub trait FmtBy {
    #[inline(always)]
    fn fmt_by<'s: 'a, 'a, Handler: FmtHandler<Self>>(
        &'s self,
        handler: Handler,
    ) -> TargetHolder<'a, Self, Handler>
    where
        Self: 's,
    {
        TargetHolder {
            inner: self,
            handler,
        }
    }
    #[inline(always)]
    fn fmt_deref_by<Handler: FmtHandler<Self>>(self, handler: Handler) -> DerefHolder<Self, Handler>
    where
        Self: ::core::ops::Deref + ::core::marker::Sized,
    {
        DerefHolder {
            inner: self,
            handler,
        }
    }
}
impl<T: ?::core::marker::Sized> FmtBy for T {}

#[cfg(test)]
pub(crate) mod test_types {
    use crate::WithInner;

    use super::*;

    use std::{fmt::Display, ops::Deref};
    #[derive(Debug, Clone, Copy, Default)]
    pub struct UppercaseStringProxy;
    impl FmtHandler<&str> for UppercaseStringProxy {
        #[inline]
        fn fmt(&self, data: &&str, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            for c in data.chars() {
                write!(f, "{}", c.to_uppercase())?;
            }
            Ok(())
        }
    }
    impl FmtHandler<str> for UppercaseStringProxy {
        #[inline]
        fn fmt(&self, data: &str, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            for c in data.chars() {
                write!(f, "{}", c.to_uppercase())?;
            }
            Ok(())
        }
    }
    impl FmtHandler<String> for UppercaseStringProxy {
        #[inline(always)]
        fn fmt(&self, data: &String, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            <Self as FmtHandler<str>>::fmt(self, data, f)
        }
    }
    #[derive(Debug, Clone, Copy, Default)]
    pub struct NumberDisplayProxy;
    impl FmtHandler<i32> for NumberDisplayProxy {
        fn fmt(&self, data: &i32, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
            if *data < 0 {
                write!(f, "负{}", data.checked_abs().unwrap_or(i32::MAX))
            } else if *data == 0 {
                write!(f, "零")
            } else {
                write!(f, "正{}", data)
            }
        }
    }
    impl<'a, Handler> FmtHandler<TargetHolder<'a, i32, Handler>> for NumberDisplayProxy
    where
        TargetHolder<'a, i32, Handler>: Display,
    {
        fn fmt(
            &self,
            data: &TargetHolder<'a, i32, Handler>,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            if **data < 0 {
                write!(f, "负{}", data.checked_abs().unwrap_or(i32::MAX))
            } else if **data == 0 {
                write!(f, "零")
            } else {
                write!(f, "正{}", data)
            }
        }
    }
    impl<T: Deref<Target = i32>, Handler> FmtHandler<DerefHolder<T, Handler>> for NumberDisplayProxy
    where
        DerefHolder<T, Handler>: Display + for<'a> WithInner<'a, i32>,
    {
        fn fmt(
            &self,
            data: &DerefHolder<T, Handler>,
            f: &mut ::core::fmt::Formatter<'_>,
        ) -> ::core::fmt::Result {
            if **data < 0 {
                let n = data.checked_abs().unwrap_or(i32::MAX);
                write!(f, "负{}", data.with_inner(&n))
            } else if **data == 0 {
                write!(f, "零")
            } else {
                write!(f, "正{}", data)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_custom_extra() {
        // 定义一个接受复杂Extra类型的代理
        #[derive(Debug, Clone, Copy, Default)]
        struct CustomExtraProxy(char, usize);

        impl FmtHandler<str> for CustomExtraProxy {
            fn fmt(&self, data: &str, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                write!(f, "{}{}{}", self.0, data, self.0.to_string().repeat(self.1))
            }
        }

        // 测试使用
        let s = "test";
        let proxy = s.fmt_by(CustomExtraProxy('*', 3));
        assert_eq!(format!("{}", proxy), "*test***");
    }
}
