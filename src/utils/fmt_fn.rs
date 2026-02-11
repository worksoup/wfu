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

/// 通用格式化函数包装器
///
/// 这个结构体包装一个闭包，该闭包接收一个 [`Formatter`](::core::fmt::Formatter) 并返回格式化结果。
/// 通过为 [`FmtFn`] 实现各种格式化特型（[`Display`](::core::fmt::Display)、[`Debug`](::core::fmt::Debug)、[`Binary`](::core::fmt::Binary) 等），
/// 允许自定义的格式化逻辑通过统一的接口被调用。
///
/// # 类型参数
/// - `F`: 闭包类型，必须实现 `for<'fmt> Fn(&mut Formatter<'fmt>) -> fmt::Result`
///
/// # 示例
/// ```rust
/// use wfu::FmtFn;
///
/// let value = 42;
/// let wrapper = FmtFn(|f: &mut std::fmt::Formatter| write!(f, "The answer is: {}", value));
///
/// // 各种格式化输出只与闭包有关。
/// assert_eq!(format!("{}", wrapper), "The answer is: 42");
/// assert_eq!(format!("{:?}", wrapper), "The answer is: 42");
/// ```
#[derive(Clone, Copy)]
pub struct FmtFn<F>(pub F);

macro_rules! fmt_impl_for_fmt_fn {
    ($trait_name: path) => {
        #[doc = concat!("为 [`FmtFn`] 实现 [`", stringify!($trait_name), "`](::core::fmt::", stringify!($trait_name), ") 格式化特型，使之可作用于", stringify!($trait_name), "输出。")]
        impl<F> $trait_name for FmtFn<F>
        where
            F: for<'fmt> Fn(&mut ::core::fmt::Formatter<'fmt>) -> ::core::fmt::Result,
        {
            #[inline(always)]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                (self.0)(f)
            }
        }
    };
}
crate::all_fmt_impl!(fmt_impl_for_fmt_fn);

#[cfg(test)]
mod tests {
    use super::*;

    /// 测试基础包装器功能
    #[test]
    fn test_format_with_basic() {
        let value = 42;
        let wrapper = FmtFn(|f: &mut std::fmt::Formatter| write!(f, "The answer is: {}", value));

        assert_eq!(format!("{}", wrapper), "The answer is: 42");
        assert_eq!(format!("{:?}", wrapper), "The answer is: 42");
    }

    /// 测试包装器支持多种格式化trait
    #[test]
    fn test_format_with_multiple_traits() {
        let wrapper = FmtFn(|f: &mut std::fmt::Formatter| write!(f, "0x{:x}", 255));

        // 所有trait都使用相同的格式化逻辑
        assert_eq!(format!("{}", wrapper), "0xff");
        assert_eq!(format!("{:?}", wrapper), "0xff");
        assert_eq!(format!("{:b}", wrapper), "0xff");
        assert_eq!(format!("{:x}", wrapper), "0xff");
        assert_eq!(format!("{:X}", wrapper), "0xff");
        assert_eq!(format!("{:o}", wrapper), "0xff");
    }
}
