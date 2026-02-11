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

#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

mod fmt_as;
mod fmt_by;
mod fmt_by_holder;
mod fmt_with;
mod utils;

pub use fmt_as::*;
pub use fmt_by::*;
pub use fmt_by_holder::*;
pub use fmt_with::*;
pub use utils::*;

#[cfg(test)]
mod tests {
    use crate::test_types::{NumberDisplayProxy, UppercaseStringProxy};

    use super::*;
    use ::core::fmt;

    #[test]
    fn test_formatting_trait_consistency() {
        // 确保所有格式化trait都使用相同的代理逻辑
        let value = "test";

        // 使用FormatWithFn
        let wrapper = FmtFn(|f: &mut fmt::Formatter| write!(f, "[{}]", value));

        // 所有格式化trait应该输出相同的内容
        assert_eq!(format!("{}", wrapper), "[test]");
        assert_eq!(format!("{:?}", wrapper), "[test]");
        assert_eq!(format!("{:b}", wrapper), "[test]");
        assert_eq!(format!("{:x}", wrapper), "[test]");
        assert_eq!(format!("{:X}", wrapper), "[test]");
        assert_eq!(format!("{:o}", wrapper), "[test]");
        assert_eq!(format!("{:p}", wrapper), "[test]");
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
