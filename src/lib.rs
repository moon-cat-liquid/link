//! 这是一个rust的单向链表的实现，本链表实现了集合的基本功能。
/// 链表结构体
#[derive(Clone)]
pub struct Link<T> {
    data: Option<Box<Node<T>>>,
    len: usize,
}
///节点结构体
#[derive(Clone)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}
impl<T> Link<T> {
    /// 获取链表的尾节点的可变引用
    fn end_node(mut node: &mut Box<Node<T>>) -> &mut Box<Node<T>> {
        while let Some(ref mut t) = node.next {
            node = t;
        }
        node
    }
    /// 获取链表的某一位置的节点的不可变引用
    /// # 输入
    /// node: 起始节点
    /// 
    /// i: 目标节点相对起始节点的索引
    /// # 输出
    /// Option<&Box<Node<T>>>: some(目标节点的不可变引用)，当目标节点获取失败(输入错误)时为None
    fn index_node(node: Option<&Box<Node<T>>>, i: usize) -> Option<&Box<Node<T>>> {
        let mut node = node?;
        for _ in 0..i {
            node = node.next.as_ref()?;
        }
        Some(node)
    }
    /// 获取链表的某一位置的节点的可变引用
    /// # 输入
    /// node: 起始节点
    /// 
    /// i: 目标节点相对起始节点的索引
    /// # 输出
    /// Option<&mut Box<Node<T>>>: some(目标节点的可变引用)，当目标节点获取失败(输入错误)时为None
    fn index_node_mut(node: Option<&mut Box<Node<T>>>, i: usize) -> Option<&mut Box<Node<T>>> {
        let mut node = node?;
        for _ in 0..i {
            node = node.next.as_mut()?;
        }
        Some(node)
    }
    /// 引发超出链表的范围的恐慌
    fn out_of_range(index: usize, len: usize) -> ! {
        panic!("index {} out of range for Link of length {}", index, len);
    }
    /// 创建空链表
    /// # 例子
    /// ```
    /// use link::*;
    /// let l: Link<isize> = Link::new();
    /// assert_eq!(format!("{:?}", l), "[]");
    /// ```
    pub fn new() -> Self {
        Link {data: None, len: 0}
    }
    /// 获取链表长度
    pub fn len(&self) -> usize {
        self.len
    }
    /// 拼接a, b两个链表，相当于a = a + b
    /// # 例子
    /// ```
    /// use link::*;
    /// let mut a: Link<isize> = link![1, 2];
    /// let b: Link<isize> = link![3, 4];
    /// a.concat(b);
    /// assert_eq!(format!("{:?}", a), "[1, 2, 3, 4]");
    /// ```
    pub fn concat(&mut self, other: Self) {
        match self.data.as_mut() {
            //空链表
            None => {
                self.data = other.data;
                self.len = other.len;
            },
            //非空链表
            Some(node) => {
                Self::end_node(node).next = other.data;
                self.len += other.len;
            }
        }
    }
    /// 转移链表，转移后原链表为空链表
    /// # 例子
    /// ```
    /// use link::*;
    /// let mut a: Link<isize> = link![0, 1, 2];
    /// let b = a.take();
    /// assert_eq!(format!("{:?}", a), "[]");
    /// assert_eq!(format!("{:?}", b), "[0, 1, 2]");
    /// ```
    pub fn take(&mut self) -> Self {
        let len = self.len;
        self.len = 0;
        Link {data: self.data.take(), len}
    }
    /// 在链表的尾部追加元素
    pub fn push_back(&mut self, val: T) {
        let n = Node {value: val, next: None};
        match self.data.as_mut() {
            //空链表
            None => {
                self.data = Some(Box::new(n));
            },
            //非空链表
            Some(node) => {
                Self::end_node(node).next = Some(Box::new(n));
            },
        }
        self.len += 1;
    }
    /// 弹出最后第一个元素，当链表为空时返回None
    /// # 例子
    /// ```
    /// use link::*;
    /// let mut l: Link<isize> = link![0, 1, 2];
    /// let v = l.pop_back();
    /// assert_eq!(format!("{:?}", l), "[0, 1]");
    /// assert_eq!(v, Some(2));
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        // 不利用长度的实现
        // let node = &mut self.data;
        // if node.as_ref()?.next.is_none() {
        //     self.len -= 1;
        //     return Some(node.take()?.value)
        // }
        // let mut node = node.as_mut()?;
        // while node.next.as_ref().and_then(|s| s.next.as_ref()).is_some() {
        //     node = node.next.as_mut()?;
        // }
        // self.len -= 1;
        // Some(node.next.take()?.value)

        // 利用长度的实现
        match self.len {
            0 => None,
            1 => self.pop(),
            len => {
                let node = Self::index_node_mut(self.data.as_mut(), len-2)?;
                let n = node.next.take()?;
                self.len -= 1;
                Some(n.value)
            }
        }
    }
    /// 在链表的头部压入一个元素
    /// # 例子
    /// ```
    /// use link::*;
    /// let mut l: Link<isize> = link![0, 1, 2];
    /// l.push(-1);
    /// assert_eq!(format!("{:?}", l), "[-1, 0, 1, 2]");
    /// ```
    pub fn push(&mut self, val: T) {           
        let n = Node {value: val, next: self.data.take()};
        self.data = Some(Box::new(n));
        self.len += 1;
    }
    /// 弹出第一个元素，当链表为空时返回None
    /// # 例子
    /// ```
    /// use link::*;
    /// let mut l: Link<isize> = link![0, 1, 2];
    /// let v = l.pop();
    /// assert_eq!(format!("{:?}", l), "[1, 2]");
    /// assert_eq!(v, Some(0));
    /// ```
    pub fn pop(&mut self) -> Option<T> {
        let node = self.data.take()?;
        self.data = node.next;
        self.len -= 1;
        Some(node.value)
    }
    /// 获取链表的第一个元素的不可变引用，当链表为空时返回None
    /// # 例子
    /// ```
    /// use link::*;
    /// let mut l: Link<isize> = link![0, 1, 2];
    /// let v = l.front();
    /// assert_eq!(v, Some(&0));
    /// ```
    pub fn front(&self) -> Option<&T> {
        //等价
        // match self.data {
        //     Some(ref n) => Some(&n.value),
        //     None => None,
        // }
        Some(&self.data.as_ref()?.value)
    }
    /// 获取链表的第一个元素的可变引用，当链表为空时返回None
    pub fn front_mut(&mut self) -> Option<&mut T> {
        Some(&mut self.data.as_mut()?.value)
    }
    /// 获取链表的最后一个元素的不可变引用，当链表为空时返回None
    pub fn back(&self) -> Option<&T> {
        let mut p = self.data.as_ref()?;
        while let Some(t) = p.next.as_ref() {
            p = t;
        }
        Some(&p.value)
    }
    /// 获取链表的最后一个元素的可变引用，当链表为空时返回None
    pub fn back_mut(&mut self) -> Option<&mut T> {
        let mut p = self.data.as_mut()?;
        while let Some(t) = p.next.as_mut() {
            p = t;
        }
        Some(&mut p.value)
    }
    /// 在指定位置插入元素，返回被插入元素的不可变引用，当插入失败时返回None
    /// # 例子
    /// ```
    /// use link::*;
    /// let mut l: Link<isize> = link![0, 1, 2];
    /// let v = l.insert(2, 3);
    /// assert_eq!(v, Some(&3));
    /// assert_eq!(format!("{:?}", l), "[0, 1, 3, 2]");
    /// ```
    pub fn insert(&mut self, i: usize, val: T) -> Option<&T> {
        if i == 0 {
            self.push(val);
            self.front()
        } else {
            let node = Self::index_node_mut(self.data.as_mut(), i-1)?;
            let n = Node {value: val, next: node.next.take()};
            node.next = Some(Box::new(n));
            self.len += 1;
            Some(&node.next.as_ref()?.value)
        }
    }
    /// 在指定位置删除元素，返回被删元素，当插入失败时返回None
    /// # 例子
    /// ```
    /// use link::*;
    /// let mut l: Link<isize> = link![0, 1, 2];
    /// let v = l.delete(1);
    /// assert_eq!(v, Some(1));
    /// assert_eq!(format!("{:?}", l), "[0, 2]");
    /// ```
    pub fn delete(&mut self, i: usize) -> Option<T> {
        if i == 0 {
            self.pop()
        } else {
            let node = Self::index_node_mut(self.data.as_mut(), i-1)?;
            let n = node.next.take()?;
            node.next = n.next;
            self.len -= 1;
            Some(n.value)
        }
    }
    /// 生成不可变迭代器
    /// # 例子
    /// ```
    /// use link::*;
    /// let l: Link<isize> = link![0, 1, 2];
    /// let mut iter = l.iter();
    /// assert_eq!(iter.next(), Some(&0));
    /// assert_eq!(iter.next(), Some(&1));
    /// assert_eq!(iter.next(), Some(&2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter(&self) -> Iter<'_, T> {
        Iter {data: self.data.as_ref()}
    }
    /// 生成可变迭代器
    /// # 例子
    /// ```
    /// use link::*;
    /// let mut l: Link<isize> = link![0, 1, 2];
    /// let mut iter = l.iter_mut();
    /// assert_eq!(iter.next(), Some(&mut 0));
    /// assert_eq!(iter.next(), Some(&mut 1));
    /// assert_eq!(iter.next(), Some(&mut 2));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut {data: self.data.as_mut()}
    }
    /// 用重复的元素创建链表
    /// # 例子
    /// ```
    /// use link::*;
    /// let a: Link<isize> = Link::from_elem(-1, 3);
    /// assert_eq!(format!("{:?}", a), "[-1, -1, -1]");
    /// ```
    pub fn from_elem(val: T, n: usize) -> Self 
    where
        T: Clone {
        let mut link: Link<T> = Self::new();
        for _ in 0..n {
            link.push(val.clone());
        }
        link
    }
}
use std::iter;
/// 不可变引用的迭代器
pub struct Iter<'a, T> {
    data: Option<&'a Box<Node<T>>>
}
impl<'a, T> iter::Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(& mut self) -> Option<Self::Item> {
        let node = self.data.take()?;
        self.data = node.next.as_ref();
        Some(&node.value)
    }
}
/// 不可变引用的迭代适配器
/// # 例子
/// ```
/// use link::*;
/// let l: Link<isize> = link![1, 2, 3];
/// let mut s = 0;
/// for i in &l {
///     s += i;
/// }
/// assert_eq!(s, 6);
/// ```
impl<'a, T> iter::IntoIterator for &'a Link<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}
/// 可变引用的迭代器
pub struct IterMut<'a, T> {
    data: Option<&'a mut Box<Node<T>>>
}
impl<'a, T> iter::Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(& mut self) -> Option<Self::Item> {
        let node = self.data.take()?;
        self.data = node.next.as_mut();
        Some(&mut node.value)
    }
}
/// 可变引用的迭代适配器
/// # 例子
/// ```
/// use link::*;
/// let mut l: Link<isize> = link![1, 2, 3];
/// for i in &mut l {
///     *i += 1;
/// }
/// assert_eq!(format!("{:?}", l), "[2, 3, 4]");
/// ```
impl<'a, T> iter::IntoIterator for &'a mut Link<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}
/// 元素迭代器
pub struct IntoIter<T> {
    data: Option<Box<Node<T>>>
}
impl<T> iter::Iterator for IntoIter<T> {
    type Item = T;
    fn next(& mut self) -> Option<Self::Item> {
        let node = self.data.take()?;
        self.data = node.next;
        Some(node.value)
    }
}
/// 元素迭代适配器
/// # 例子
/// ```
/// use link::*;
/// let l: Link<isize> = link![0, 1, 2];
/// let mut iter = l.into_iter();
/// assert_eq!(iter.next(), Some(0));
/// assert_eq!(iter.next(), Some(1));
/// assert_eq!(iter.next(), Some(2));
/// assert_eq!(iter.next(), None);
/// ```
impl<T> iter::IntoIterator for Link<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;
    
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {data: self.data}
    }
}
/// 迭代转化器
/// # 例子
/// ```
/// use link::*;
/// let a: Link<isize> = std::iter::repeat(-1).take(3).collect();
/// assert_eq!(format!("{:?}", a), "[-1, -1, -1]");
/// ```
impl<T> iter::FromIterator<T> for Link<T> {
    fn from_iter<I>(iter: I) -> Self 
    where
        I: iter::IntoIterator<Item = T> {
        let mut link: Link<T> = Self::new();
        let mut node = &mut link.data;
        for i in iter {
            *node = Some(Box::new(Node{value: i, next: None}));
            node = &mut node.as_mut().unwrap().next;
            link.len += 1;
        }
        link
    }
}
/// 迭代复制转化器
/// # 例子
/// ```
/// use link::*;
/// let v = vec![-1; 3];
/// let a: Link<isize> = v.iter().collect();
/// assert_eq!(format!("{:?}", a), "[-1, -1, -1]");
/// ```
impl<'a, T: Clone + 'a> iter::FromIterator<&'a T> for Link<T> {
    fn from_iter<I>(iter: I) -> Self 
    where
        I: iter::IntoIterator<Item = &'a T> {
        let mut link: Link<T> = Self::new();
        let mut node = &mut link.data;
        for i in iter {
            *node = Some(Box::new(Node{value: i.clone(), next: None}));
            node = &mut node.as_mut().unwrap().next;
            link.len += 1;
        }
        link
    }
}
/// 创建链表的宏
/// # 例子
/// ```
/// use link::*;
/// let a: Link<isize> = link![0, 1, 2];
/// let b: Link<isize> = link![-1; 3];
/// let c: Link<isize> = link![];
/// assert_eq!(format!("{:?}", a), "[0, 1, 2]");
/// assert_eq!(format!("{:?}", b), "[-1, -1, -1]");
/// assert_eq!(format!("{:?}", c), "[]");
/// ```
#[macro_export]
macro_rules! link {
    ($($x:expr),+) => (vec![$($x),*].into_iter().collect());
    ($x:expr; $n:expr) => ($crate::Link::from_elem($x, $n));
    () => ($crate::Link::new())
}
use std::fmt;
/// 格式化表示
impl<T: fmt::Debug> fmt::Debug for Link<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()      
    }
}
use std::ops;
/// 索引（查看）操作
/// # 例子
/// ```
/// use link::*;
/// let a: Link<isize> = link![1, 2, 3];
/// assert_eq!(a[0], 1);
/// assert_eq!(a[1], 2);
/// assert_eq!(a[2], 3);
/// ```
impl<T> ops::Index<usize> for Link<T> {
    type Output = T;

    fn index(&self, i: usize) -> &Self::Output {
        match Self::index_node(self.data.as_ref(),i) {
            Some(n) => &n.value,
            None => Self::out_of_range(i, self.len)
        }
    }
}
/// 索引（修改）操作
/// # 例子
/// ```
/// use link::*;
/// let mut a: Link<isize> = link![1, 2, 3];
/// a[1] = -1;
/// assert_eq!(format!("{:?}", a), "[1, -1, 3]");
/// ```
impl<T> ops::IndexMut<usize> for Link<T> {
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        match Self::index_node_mut(self.data.as_mut(),i) {
            Some(n) => &mut n.value,
            None => Self::out_of_range(i, self.len)
        }
    }
}
/// 加法（追加）操作
/// # 例子
/// ```
/// use link::*;
/// let mut a: Link<isize> = link![1, 2, 3];
/// a = a + 4;
/// assert_eq!(format!("{:?}", a), "[1, 2, 3, 4]");
/// ```
impl<T> ops::Add<T> for Link<T> {
    type Output = Self;
    fn add(mut self, rhs: T) -> Self::Output {
        self.push_back(rhs);
        self
    }
}
/// 加法（拼接）操作
/// # 例子
/// ```
/// use link::*;
/// let mut a: Link<isize> = link![1, 2, 3];
/// let b: Link<isize> = link![4, 5];
/// a = a + b;
/// assert_eq!(format!("{:?}", a), "[1, 2, 3, 4, 5]");
/// ```
impl<T> ops::Add for Link<T> {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.concat(rhs);
        self
    }
}
/// 自加（追加）操作
/// # 例子
/// ```
/// use link::*;
/// let mut a: Link<isize> = link![1, 2, 3];
/// a += 4;
/// assert_eq!(format!("{:?}", a), "[1, 2, 3, 4]");
/// ```
impl<T> ops::AddAssign<T> for Link<T> {
    fn add_assign(&mut self, other: T) {
        self.push_back(other);
    }
}
/// 自加（拼接）操作
/// # 例子
/// ```
/// use link::*;
/// let mut a: Link<isize> = link![1, 2, 3];
/// let b: Link<isize> = link![4, 5];
/// a += b;
/// assert_eq!(format!("{:?}", a), "[1, 2, 3, 4, 5]");
/// ```
impl<T> ops::AddAssign for Link<T> {
    fn add_assign(&mut self, other: Self) {
        self.concat(other);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test1() {  
    }
}
