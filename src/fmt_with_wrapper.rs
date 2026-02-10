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
