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

pub trait FmtAs: FmtBy {
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
    fn fmt_deref_as<Handler: FmtHandler<Self> + ::core::default::Default>(
        self,
    ) -> DerefHolder<Self, Handler>
    where
        Self: ::core::ops::Deref + ::core::marker::Sized,
    {
        self.fmt_deref_by(Handler::default())
    }
}
impl<T: ?::core::marker::Sized> FmtAs for T {}

#[cfg(test)]
mod tests {
    use crate::{UpperExpProxy, UpperHexProxy, test_types::{NumberDisplayProxy, UppercaseStringProxy}};

    use super::*;

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
}
