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

/// 用于将迭代器格式化为调试映射的格式化处理器
///
/// 这个结构体适配迭代器，使用 `debug_map()` 方法将其格式化，适用于键值对类型的迭代器。
///
/// # 类型参数
/// - `Iter`: 迭代器类型，`&Iter` 必须实现 [`IntoIterator<Item = (K, V)>`]
/// - `K`: 键类型，必须实现 [`Debug`](::core::fmt::Debug)
/// - `V`: 值类型，必须实现 [`Debug`](::core::fmt::Debug)
///
/// # 示例
/// ``` rust
/// use wfu::{FmtBy, DebugMap};
/// let mut map = std::collections::HashMap::new();
/// map.insert("key1", 1);
/// map.insert("key2", 2);
/// map.insert("key3", 3);
/// let proxy_map = map.fmt_as::<DebugMap>();
/// let output = proxy_map.to_string();
/// assert!(output.contains(r#""key1": 1"#),);
/// assert!(output.contains(r#""key2": 2"#),);
/// assert!(output.contains(r#""key3": 3"#),);
/// ```
/// # 注意
/// - 要求 `&Iter` 实现 `IntoIterator<Item = (&K, &V)>`，即通过引用来迭代集合
/// - 对于需要克隆才能迭代的类型（如 [`Map`](::core::iter::Map)），请使用 [`CloneIterDebugMap`]
/// - 对于元素为引用类型 `&(K, V)` 的集合，请使用 [`RefItemDebugMap`]
#[derive(Debug, Clone, Copy, Default)]
pub struct DebugMap;

impl<Iter, K, V> FmtHandler<Iter> for DebugMap
where
    for<'a> &'a Iter: ::core::iter::IntoIterator<Item = (&'a K, &'a V)>,
    Iter: ?::core::marker::Sized,
    K: ::core::fmt::Debug,
    V: ::core::fmt::Debug,
{
    #[inline]
    fn fmt(&self, data: &Iter, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        // 使用 Formatter 的 debug_map 方法来格式化键值对迭代器
        f.debug_map().entries(data).finish()
    }
}

/// 用于将元素为引用的迭代器格式化为调试映射的格式化处理器
///
/// 这个结构体适配元素类型为 `&(K, V)` 的迭代器，使用 `debug_map()` 方法将其格式化。
///
/// # 类型参数
/// - `Iter`: 迭代器类型，`&Iter` 必须实现 [`IntoIterator<Item = &'a (K, V)>`]
/// - `K`: 键类型，必须实现 [`Debug`](::core::fmt::Debug)
/// - `V`: 值类型，必须实现 [`Debug`](::core::fmt::Debug)
///
/// # 示例
/// ``` rust
/// use wfu::{FmtBy, RefItemDebugMap};
/// let map = vec![("key1", 1), ("key2", 2), ("key3", 3)];
/// let proxy = map.fmt_as::<RefItemDebugMap>();
/// assert_eq!(format!("{:?}", proxy), r#"{"key1": 1, "key2": 2, "key3": 3}"#);
/// ```
/// # 注意
/// - 要求 `&Iter` 实现 `IntoIterator<Item = &'a (K, V)>`，即迭代元素为引用类型
/// - 适用于包含引用类型元素的集合，如切片 `&[(K, V)]`
#[derive(Debug, Clone, Copy, Default)]
pub struct RefItemDebugMap;

impl<Iter, K, V> FmtHandler<Iter> for RefItemDebugMap
where
    for<'a> &'a Iter: ::core::iter::IntoIterator<Item = &'a (K, V)>,
    Iter: ?::core::marker::Sized,
    K: ::core::fmt::Debug,
    V: ::core::fmt::Debug,
{
    #[inline]
    fn fmt(&self, data: &Iter, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        // 由于迭代元素是引用类型，需要先映射为 (K, V) 引用
        let mut debug_map = f.debug_map();
        for (k, v) in data {
            debug_map.entry(k, v);
        }
        debug_map.finish()
    }
}

/// 用于克隆后迭代并格式化为调试映射的格式化处理器
///
/// 这个结构体适配需要克隆才能迭代的集合，使用 `debug_map()` 方法将其格式化。
///
/// # 类型参数
/// - `Iter`: 迭代器类型，必须实现 [`IntoIterator<Item = (K, V)>`] 和 [`Clone`]
/// - `K`: 键类型，必须实现 [`Debug`](::core::fmt::Debug)
/// - `V`: 值类型，必须实现 [`Debug`](::core::fmt::Debug)
///
/// # 示例
/// ``` rust
/// use wfu::{FmtBy, CloneIterDebugMap};
/// let range = (1..4).map(|i| (format!("key{}", i), i * 10));
/// let proxy = range.fmt_by(CloneIterDebugMap);
/// assert_eq!(format!("{:?}", proxy), r#"{"key1": 10, "key2": 20, "key3": 30}"#);
/// ```
/// # 注意
/// - 要求 `Iter` 实现 [`Clone`] 和 [`IntoIterator<Item = (K, V)>`]
/// - 适用于需要克隆才能迭代的类型，如某些迭代器适配器
#[derive(Debug, Clone, Copy, Default)]
pub struct CloneIterDebugMap;

impl<Iter, K, V> FmtHandler<Iter> for CloneIterDebugMap
where
    Iter: ::core::iter::IntoIterator<Item = (K, V)> + ::core::clone::Clone,
    K: ::core::fmt::Debug,
    V: ::core::fmt::Debug,
{
    #[inline]
    fn fmt(&self, data: &Iter, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
        // 克隆数据后创建调试映射
        f.debug_map().entries(data.clone()).finish()
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::FmtBy;

    #[test]
    fn test_debug_map() {
        let mut map = HashMap::new();
        map.insert("key1", 1);
        map.insert("key2", 2);
        map.insert("key3", 3);

        let proxy_map = map.fmt_as::<DebugMap>();
        let output = proxy_map.to_string();
        assert!(output.contains(r#""key1": 1"#),);
        assert!(output.contains(r#""key2": 2"#),);
        assert!(output.contains(r#""key3": 3"#),);
        // 测试 DebugMap 基础功能
        let map = vec![("key1", 1), ("key2", 2), ("key3", 3)];
        let proxy = map.fmt_as::<RefItemDebugMap>();
        assert_eq!(
            format!("{:?}", proxy),
            r#"{"key1": 1, "key2": 2, "key3": 3}"#
        );

        // 测试空迭代器
        let empty_map: Vec<(&str, i32)> = Vec::new();
        let proxy_empty = empty_map.fmt_by(RefItemDebugMap);
        assert_eq!(format!("{:?}", proxy_empty), "{}");

        // 测试单个元素
        let single_map = vec![("key", 42)];
        let proxy_single = single_map.fmt_by(RefItemDebugMap);
        assert_eq!(format!("{:?}", proxy_single), r#"{"key": 42}"#);
    }

    #[test]
    fn test_ref_item_debug_map() {
        // 测试 RefItemDebugMap 基础功能
        let map = vec![("key1", 1), ("key2", 2), ("key3", 3)];
        let slice: &[(&str, i32)] = &map;
        let proxy = slice.fmt_by(RefItemDebugMap);
        assert_eq!(
            format!("{:?}", proxy),
            r#"{"key1": 1, "key2": 2, "key3": 3}"#
        );
    }

    #[test]
    fn test_clone_iter_debug_map() {
        // 测试 CloneIterDebugMap 基础功能
        let range = (1..4).map(|i| (format!("key{}", i), i * 10));
        let proxy = range.fmt_by(CloneIterDebugMap);
        assert_eq!(
            format!("{:?}", proxy),
            r#"{"key1": 10, "key2": 20, "key3": 30}"#
        );
    }

    #[test]
    fn test_custom_debug_map() {
        // 测试 CustomDebugMap 基础功能
        let data = [1, 2, 3];

        let binding = data.iter().map(|x: &i32| (format!("key{}", x), x * 10));
        let proxy = binding.fmt_as::<CloneIterDebugMap>();
        assert_eq!(
            format!("{:?}", proxy),
            r#"{"key1": 10, "key2": 20, "key3": 30}"#
        );

        // 测试空数据
        let empty_data: Vec<i32> = Vec::new();
        let binding = empty_data
            .iter()
            .map(|x: &i32| (format!("key{}", x), x * 10));
        let proxy_empty = binding.fmt_as::<CloneIterDebugMap>();
        assert_eq!(format!("{:?}", proxy_empty), "{}");
    }
}
