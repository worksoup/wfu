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

use crate::{FmtBy, FmtHandler, FmtWithWrapper};
trait PrivateHolderMarker {}

#[allow(private_bounds)]
pub trait WithInner<'i, Inner: ?::core::marker::Sized>: PrivateHolderMarker {
    fn with_inner(&self, inner: &'i Inner) -> Self;
}

#[allow(private_bounds)]
pub trait FmtByHolder: ::core::marker::Sized + PrivateHolderMarker {
    #[inline(always)]
    fn then_by<'s, 'f, 'fmt: 'f, Handler: FmtHandler<Self>>(
        self,
        handler: Handler,
    ) -> DerefHolder<Self, Handler>
    where
        Self: ::core::ops::Deref,
    {
        self.fmt_deref_by(handler)
    }
    #[inline(always)]
    fn then_as<'s, 'f, 'fmt: 'f, Handler: FmtHandler<Self> + ::core::default::Default>(
        self,
    ) -> DerefHolder<Self, Handler>
    where
        Self: ::core::ops::Deref + ::core::marker::Sized,
    {
        self.then_by(Handler::default())
    }
    #[inline(always)]
    fn then_with<
        FmtWith: for<'fmt> Fn(&Self, &mut ::core::fmt::Formatter<'fmt>) -> ::core::fmt::Result,
    >(
        self,
        f: &'_ FmtWith,
    ) -> DerefHolder<Self, FmtWithWrapper<'_, FmtWith>>
    where
        Self: ::core::ops::Deref + ::core::marker::Sized,
    {
        self.then_by(FmtWithWrapper(f))
    }
}

#[derive(Clone, Copy)]
pub struct TargetHolder<'a, T: ?::core::marker::Sized, D> {
    pub(crate) inner: &'a T,
    pub(crate) handler: D,
}

impl<'a, T: ?::core::marker::Sized, D> FmtByHolder for TargetHolder<'a, T, D> {}
impl<'a, T: ?::core::marker::Sized, D> PrivateHolderMarker for TargetHolder<'a, T, D> {}
impl<'a, T: ?::core::marker::Sized, D: ::core::marker::Copy> WithInner<'a, T>
    for TargetHolder<'a, T, D>
{
    #[inline]
    fn with_inner(&self, inner: &'a T) -> Self {
        Self {
            inner,
            handler: self.handler,
        }
    }
}

impl<'a, T: ?::core::marker::Sized, D> ::core::ops::Deref for TargetHolder<'a, T, D> {
    type Target = T;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.inner
    }
}

impl<'a, T: ?::core::marker::Sized, D> ::core::convert::AsRef<T> for TargetHolder<'a, T, D> {
    #[inline]
    fn as_ref(&self) -> &T {
        self.inner
    }
}

#[derive(Clone, Copy)]
pub struct DerefHolder<T: ::core::ops::Deref, D> {
    pub(crate) inner: T,
    pub(crate) handler: D,
}

impl<T: ::core::ops::Deref, D> FmtByHolder for DerefHolder<T, D> {}
impl<T: ::core::ops::Deref, D> PrivateHolderMarker for DerefHolder<T, D> {}
impl<'a, T: ::core::ops::Deref + WithInner<'a, T::Target>, D: ::core::marker::Copy> WithInner<'a, T>
    for DerefHolder<T, D>
{
    #[inline]
    fn with_inner(&self, inner: &'a T) -> Self {
        Self {
            inner: self.inner.with_inner(inner),
            handler: self.handler,
        }
    }
}
// impl<'a, T, D1: FmtHandler<T> + ::core::marker::Copy, D2: FmtHandler<TargetHolder<'a, T, D1>>>
//     FmtHandler<T> for DerefHolder<TargetHolder<'a, T, D1>, D2>
// {
//     #[inline]
//     fn fmt(&self, data: &T, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
//         let new_holder = data.fmt_by(self.inner.handler);
//         self.handler.fmt(new_holder, f)
//     }
// }

impl<T: ::core::ops::Deref, D> ::core::ops::Deref for DerefHolder<T, D> {
    type Target = T::Target;
    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T: ::core::ops::DerefMut, D> ::core::ops::DerefMut for DerefHolder<T, D> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

impl<T: ::core::ops::Deref, D> ::core::convert::AsRef<T> for DerefHolder<T, D> {
    #[inline]
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

macro_rules! fmt_impl_for_fmt_by_holder {
    ($trait_name: path) => {
        #[doc = concat!("为 [`FmtByHolder`] 实现 [`", stringify!($trait_name), "`](::core::fmt::", stringify!($trait_name), ") 格式化特型，使之可作用于调试输出。")]
        impl<'a, T, D> $trait_name for TargetHolder<'a, T, D>
        where
            T: ?::core::marker::Sized,
            D: for<'f, 'fmt> crate::FmtHandler<T>,
        {
            #[inline(always)]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.handler.fmt(self.inner, f)
            }
        }
    };
}

macro_rules! fmt_impl_for_deref_holder {
    ($trait_name: path) => {
        #[doc = concat!("为 [`DerefHolder`] 实现 [`", stringify!($trait_name), "`](::core::fmt::", stringify!($trait_name), ") 格式化特型，使之可作用于调试输出。")]
        impl< T, D> $trait_name for DerefHolder<T, D>
        where
            T: ::core::ops::Deref,
            D: crate::FmtHandler<T>,
        {
            #[inline(always)]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                self.handler.fmt(&self.inner, f)
            }
        }
    };
}

crate::all_fmt_impl!(fmt_impl_for_fmt_by_holder);
crate::all_fmt_impl!(fmt_impl_for_deref_holder);

#[cfg(test)]
mod tests {
    use crate::{FmtBy, FmtByHolder, Repeat, UpperHexProxy};

    #[test]
    fn test_chain_composition() {
        let value = 42;

        // 测试链式调用：先转换为十六进制，再添加前缀
        let chained = value
            .fmt_as::<UpperHexProxy>()
            .then_with(&|v, f| write!(f, "0x{}", v))
            .then_by(Repeat(5));

        assert_eq!(format!("{}", chained), "0x2A0x2A0x2A0x2A0x2A");
    }
}
