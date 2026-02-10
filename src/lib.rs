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

mod fmt_by;
mod fmt_by_holder;
mod fmt_with_wrapper;
mod utils;

pub use fmt_by::*;
pub use fmt_by_holder::*;
pub use fmt_with_wrapper::*;
pub use utils::*;

#[cfg(test)]
mod tests {
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
}
