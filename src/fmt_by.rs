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

use crate::{DerefHolder, FmtWithWrapper, TargetHolder};
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
    fn fmt_as<'s: 'a, 'a, Handler: FmtHandler<Self> + ::core::default::Default>(
        &'s self,
    ) -> TargetHolder<'a, Self, Handler>
    where
        Self: 's,
    {
        self.fmt_by(Handler::default())
    }
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
    fn fmt_deref_by<Handler: FmtHandler<Self>>(self, handler: Handler) -> DerefHolder<Self, Handler>
    where
        Self: ::core::ops::Deref + ::core::marker::Sized,
    {
        DerefHolder {
            inner: self,
            handler,
        }
    }
    #[inline(always)]
    fn fmt_deref_as<Handler: FmtHandler<Self> + ::core::default::Default>(
        self,
    ) -> DerefHolder<Self, Handler>
    where
        Self: ::core::ops::Deref + ::core::marker::Sized,
    {
        self.fmt_deref_by(Handler::default())
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
impl<T: ?::core::marker::Sized> FmtBy for T {}

#[cfg(test)]
mod tests {

    use std::fmt::Formatter;

    use crate::{DebugProxy, UpperExpProxy, UpperHexProxy, WithInner};

    use super::*;
    /// 测试自定义Extra类型
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

    use std::{fmt::Display, ops::Deref};
    #[derive(Debug, Clone, Copy, Default)]
    struct UppercaseStringProxy;
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
    struct NumberDisplayProxy;
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
    #[test]
    fn test_basic_proxy() {
        // 测试字符串转大写
        let s = "hello world";
        let display_output = format!("{}", s.fmt_as::<UppercaseStringProxy>());
        let debug_output = format!("{:?}", s.fmt_as::<UppercaseStringProxy>());

        assert_eq!(display_output, "HELLO WORLD");
        assert_eq!(debug_output, "HELLO WORLD"); // 只与格式有关，而与对应的格式化特型无关。

        // 测试数字格式化
        let n = 42;
        let display_output = format!("{}", n.fmt_as::<NumberDisplayProxy>());
        let debug_output = format!("{:?}", n.fmt_as::<NumberDisplayProxy>());
        let binary_output = format!("{:b}", n.fmt_as::<NumberDisplayProxy>());
        let hex_output = format!("{:x}", n.fmt_as::<NumberDisplayProxy>());

        assert_eq!(display_output, "正42");
        assert_eq!(debug_output, "正42"); // 只与格式有关，而与对应的格式化特型无关。
        assert_eq!(binary_output, "正42"); // 只与格式有关，而与对应的格式化特型无关。
        assert_eq!(hex_output, "正42"); // 只与格式有关，而与对应的格式化特型无关。

        // 测试负数和零
        assert_eq!(format!("{}", (-10).fmt_as::<NumberDisplayProxy>()), "负10");
        assert_eq!(format!("{}", 0.fmt_as::<NumberDisplayProxy>()), "零");
    }

    /// 测试多种格式化trait
    #[test]
    fn test_multiple_formatting_traits() {
        let n = 255;

        // 测试不同的格式化方式
        assert_eq!(format!("{}", n.fmt_as::<NumberDisplayProxy>()), "正255");
        assert_eq!(format!("{:b}", n.fmt_as::<NumberDisplayProxy>()), "正255"); // 只与格式有关，而与对应的格式化特型无关。
        assert_eq!(format!("{:x}", n.fmt_as::<NumberDisplayProxy>()), "正255"); // 只与格式有关，而与对应的格式化特型无关。
        assert_eq!(format!("{:X}", n.fmt_as::<NumberDisplayProxy>()), "正255"); // 只与格式有关，而与对应的格式化特型无关。
    }

    /// 测试生命周期处理
    #[test]
    fn test_lifetime_handling() {
        // 确保代理可以正确借用
        let s = String::from("hello");
        let proxy = {
            let borrowed = &s[..];
            borrowed.fmt_as::<UppercaseStringProxy>()
        };
        assert_eq!(proxy.to_string(), "HELLO");
    }

    /// 测试引用类型
    #[test]
    fn test_reference_types() {
        // 测试直接使用引用
        let s = "hello";
        let s_ref = &s;
        let proxy = s_ref.fmt_as::<UppercaseStringProxy>();
        assert_eq!(format!("{}", proxy), "HELLO");

        // 测试双重引用
        let s_ref_ref = &&s;
        let proxy = (*s_ref_ref).fmt_as::<UppercaseStringProxy>();
        assert_eq!(format!("{}", proxy), "HELLO");
    }

    // ===== 边界情况测试 =====

    /// 测试边界情况
    #[test]
    fn test_edge_cases() {
        // 空字符串
        let empty: Vec<i32> = vec![];
        let proxy = empty.fmt_by(crate::Joined(", "));
        assert_eq!(format!("{}", proxy), "");
        assert_eq!(format!("{}", "".fmt_as::<UppercaseStringProxy>()), "");

        // 特殊字符
        let special = "hello\nworld\t!";
        let proxy = special.fmt_as::<UppercaseStringProxy>();
        assert!(format!("{}", proxy).contains("HELLO"));
        assert!(format!("{}", proxy).contains("WORLD"));

        // 大数字
        assert_eq!(
            format!(
                "{}",
                i32::MAX
                    .fmt_as::<UpperExpProxy>()
                    .fmt_as::<NumberDisplayProxy>()
            ),
            format!("正{}", i32::MAX.fmt_as::<UpperExpProxy>())
        );
        assert_eq!(
            format!(
                "{}",
                i32::MAX
                    .fmt_as::<UpperHexProxy>()
                    .fmt_as::<NumberDisplayProxy>()
            ),
            format!("正{}", i32::MAX.fmt_as::<UpperHexProxy>())
        );

        // 最小负数
        assert_eq!(
            format!("{}", i32::MIN.fmt_as::<NumberDisplayProxy>()),
            format!("负{}", i32::MIN.checked_abs().unwrap_or(i32::MAX))
        ); // 测试最小整数
        assert_eq!(
            format!("{}", i32::MIN.fmt_as::<NumberDisplayProxy>()),
            format!("负{}", i32::MIN.checked_abs().unwrap_or(i32::MAX))
        );

        // 测试溢出情况
        assert_eq!(
            format!("{}", i32::MAX.fmt_by(crate::Repeat(2))),
            format!("{}{}", i32::MAX, i32::MAX)
        );
    }

    /// 测试Unicode字符
    #[test]
    fn test_unicode_characters() {
        // 测试Unicode字符
        let s = "你好世界";
        let proxy = s.fmt_as::<UppercaseStringProxy>();
        // 中文字符的大写转换可能没有变化
        assert_eq!(format!("{}", proxy), "你好世界");

        // 测试混合字符
        let s = "Hello 世界 123";
        let proxy = s.fmt_as::<UppercaseStringProxy>();
        assert_eq!(format!("{}", proxy), "HELLO 世界 123");
    }
    /// 测试在函数中使用代理
    #[test]
    fn test_proxy_in_functions() {
        fn format_uppercase<T: AsRef<str>>(s: T) -> String {
            s.as_ref().fmt_as::<UppercaseStringProxy>().to_string()
        }

        fn format_number(n: &i32) -> String {
            n.fmt_as::<NumberDisplayProxy>().to_string()
        }

        assert_eq!(format_uppercase("hello"), "HELLO");
        assert_eq!(format_number(&-5), "负5");
    }

    #[test]
    fn test_fmt_as_inner_type() {
        // 测试为特定内部类型实现FmtAs
        let s = "hello";

        // 默认使用Self作为Inner
        let proxy1 = s.fmt_as::<UppercaseStringProxy>();
        assert_eq!(format!("{}", proxy1), "HELLO");

        // 可以为不同类型的值使用同一个代理
        let string = String::from("world");
        let proxy2 = string.fmt_as::<UppercaseStringProxy>();
        assert_eq!(format!("{}", proxy2), "WORLD");
    }
    /// 测试代理和包装器的组合使用
    #[test]
    fn test_combined_usage() {
        let data = vec![1, 2, 3];

        // 使用FmtWith包装代理
        let holder =
            data.fmt_with(&|v, f| write!(f, "数组: {}", v.as_slice().fmt_as::<DebugProxy>()));
        assert_eq!(format!("{}", holder), "数组: [1, 2, 3]");

        // 链式调用
        let result = format!("大写: {}", "test".fmt_as::<UppercaseStringProxy>());
        assert_eq!(result, "大写: TEST");
    }

    // ===== 性能模式测试 =====

    /// 测试性能模式
    #[test]
    fn test_performance_patterns() {
        // 使用引用，避免拷贝
        let data = "performance test";
        let proxy = data.fmt_as::<UppercaseStringProxy>();

        // 直接写入buffer
        let mut buffer = String::new();
        std::fmt::write(&mut buffer, format_args!("{}", proxy)).unwrap();
        assert_eq!(buffer, "PERFORMANCE TEST");

        // 测试多次使用同一个代理
        let n = 100;
        let proxy = n.fmt_as::<NumberDisplayProxy>();

        let s1 = format!("{}", proxy);
        let s2 = format!("{:b}", proxy);
        let s3 = format!("{:x}", proxy);

        assert_eq!(s1, "正100");
        assert_eq!(s2, "正100"); // 只与格式有关，而与对应的格式化特型无关。
        assert_eq!(s3, "正100"); // 只与格式有关，而与对应的格式化特型无关。
    }

    // ===== no_std兼容性测试 =====

    /// 测试no_std兼容性
    #[test]
    fn test_no_std_compatibility() {
        use ::core::fmt::Write;

        struct Buffer {
            data: [u8; 100],
            len: usize,
        }

        impl Write for Buffer {
            fn write_str(&mut self, s: &str) -> std::fmt::Result {
                let bytes = s.as_bytes();
                let end = self.len + bytes.len();

                if end > self.data.len() {
                    return Err(std::fmt::Error);
                }

                self.data[self.len..end].copy_from_slice(bytes);
                self.len = end;
                Ok(())
            }
        }

        let mut buffer = Buffer {
            data: [0; 100],
            len: 0,
        };

        let s = "hello";
        write!(buffer, "{}", s.fmt_as::<UppercaseStringProxy>()).unwrap();

        let output = ::core::str::from_utf8(&buffer.data[..buffer.len]).unwrap();
        assert_eq!(output, "HELLO");
    }
}
