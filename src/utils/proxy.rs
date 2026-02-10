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

macro_rules! define_fmt_proxy {
    ($trait_name: path, $proxy_name: ident) => {
        #[derive(Debug, Clone, Copy, Default)]
        pub struct $proxy_name;

        impl<'a, T: ?::core::marker::Sized + $trait_name> FmtHandler<T> for $proxy_name {
            #[inline(always)]
            fn fmt(&self, data: &T, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                data.fmt(f)
            }
        }
    };
}

define_fmt_proxy!(::core::fmt::Debug, DebugProxy);
define_fmt_proxy!(::core::fmt::Display, DisplayProxy);
define_fmt_proxy!(::core::fmt::Binary, BinaryProxy);
define_fmt_proxy!(::core::fmt::LowerExp, LowerExpProxy);
define_fmt_proxy!(::core::fmt::UpperExp, UpperExpProxy);
define_fmt_proxy!(::core::fmt::LowerHex, LowerHexProxy);
define_fmt_proxy!(::core::fmt::UpperHex, UpperHexProxy);
define_fmt_proxy!(::core::fmt::Octal, OctalProxy);
define_fmt_proxy!(::core::fmt::Pointer, PointerProxy);

#[derive(Debug, Clone, Copy, Default)]
pub struct FmtFnProxy;

impl<T: ?::core::marker::Sized + Fn(&mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result>
    FmtHandler<T> for FmtFnProxy
{
    #[inline(always)]
    fn fmt(&self, fmt_fn: &T, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        fmt_fn(f)
    }
}

#[cfg(test)]
mod tests {
    use crate::FmtBy;

    use super::*;
    #[test]
    fn test_macro_defined_proxies() {
        // 测试DebugProxy
        let value = "42\t";
        let debug_proxy = (&value).fmt_as::<DebugProxy>();
        assert_eq!(format!("{:?}", debug_proxy), "\"42\\t\"");

        // 测试DisplayProxy
        let display_proxy = (&value).fmt_as::<DisplayProxy>();
        assert_eq!(format!("{}", display_proxy), "42\t");

        let value = 42;
        // 测试BinaryProxy
        let binary_proxy = value.fmt_as::<BinaryProxy>();
        assert_eq!(format!("{:b}", binary_proxy), "101010");

        // 测试LowerHexProxy
        let lower_hex_proxy = value.fmt_as::<LowerHexProxy>();
        assert_eq!(format!("{:x}", lower_hex_proxy), "2a");

        // 测试UpperHexProxy
        let upper_hex_proxy = value.fmt_as::<UpperHexProxy>();
        assert_eq!(format!("{:X}", upper_hex_proxy), "2A");

        // 测试OctalProxy
        let octal_proxy = value.fmt_as::<OctalProxy>();
        assert_eq!(format!("{:o}", octal_proxy), "52");

        let ptr = &value;
        // 测试PointerProxy
        let ptr_proxy = (&ptr).fmt_as::<PointerProxy>();
        let output = format!("{:p}", ptr_proxy);
        assert!(output.starts_with("0x"));
    }

    #[test]
    fn test_format_with_basic() {
        let value = 42;
        let r#fn = |f: &mut std::fmt::Formatter| write!(f, "The answer is: {}", value);

        assert_eq!(
            format!("{}", r#fn.fmt_as::<FmtFnProxy>()),
            "The answer is: 42"
        );
        assert_eq!(
            format!("{:?}", r#fn.fmt_as::<FmtFnProxy>()),
            "The answer is: 42"
        );
    }

    #[test]
    fn test_format_with_multiple_traits() {
        let wrapper = |f: &mut std::fmt::Formatter| write!(f, "0x{:x}", 255);

        // 所有trait都使用相同的格式化逻辑
        assert_eq!(format!("{}", wrapper.fmt_as::<FmtFnProxy>()), "0xff");
        assert_eq!(format!("{:?}", wrapper.fmt_as::<FmtFnProxy>()), "0xff");
        assert_eq!(format!("{:b}", wrapper.fmt_as::<FmtFnProxy>()), "0xff");
        assert_eq!(format!("{:x}", wrapper.fmt_as::<FmtFnProxy>()), "0xff");
        assert_eq!(format!("{:X}", wrapper.fmt_as::<FmtFnProxy>()), "0xff");
        assert_eq!(format!("{:o}", wrapper.fmt_as::<FmtFnProxy>()), "0xff");
    }
}
