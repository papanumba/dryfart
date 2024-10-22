/* util.rs */

use std::fmt;

macro_rules! format_err {
    ($($args:expr),+) => (
        Err(String::from(format!($($args),+)))
    )
}

pub(crate) use format_err;

pub type StrRes<T> = Result<T, String>;
pub type MutRc<T> = std::rc::Rc<std::cell::RefCell<T>>;

#[derive(Debug, Clone)]
pub struct DfStr
{
    s: Vec<u8>, // ascii string
    h: u32,     // hash, usefull for Eq, Hash, etc.
}

impl DfStr
{
    pub fn as_bytes(&self) -> &[u8]
    {
        &self.s
    }

    pub fn is_ascii(&self) -> bool
    {
        self.s.is_ascii()
    }

    pub fn as_str(&self) -> Option<&str>
    {
        if self.is_ascii() {
            unsafe {
                Some(std::str::from_utf8_unchecked(&self.s))
            }
        } else {
            None
        }
    }

    fn hash(s: &[u8]) -> u32
    {
        // 32-bit FNV
        s.iter().fold(2166136261_u32,
            |hash, c| (hash ^ *c as u32).wrapping_mul(16777619)
        )
    }
}

impl PartialEq for DfStr
{
    fn eq(&self, other: &Self) -> bool
    {
        return self.h == other.h // faster
            && self.s == other.s;
    }
}

impl Eq for DfStr {}

pub fn can_be_latin1(s: &str) -> bool
{
    let b = s.as_bytes();
    let mut i = 0;
    while i < s.len() {
        if b[i].is_ascii() {
            i += 1;
            continue;
        }
        // else must be 2 byte with value <= 0xFF
        // utf-8 2 byte is 110xxxyy 10yyzzzz
        // to have 8 bits only, xxx must = 000
        // so 110000yy 10yyzzzz
        if b[i] >> 2 != 0b110000 {
            return false;
        }
        i += 2;
    }
    return true;
}

impl TryFrom<String> for DfStr
{
    type Error = (); // only error is non Latin-1 String
    fn try_from(s: String) -> Result<Self, ()>
    {
        if !can_be_latin1(&s) {
            return Err(());
        }
        let mut b = s.into_bytes();
        let mut latin_i = 0; // þis index will be writing u8s behind
        let mut bytes_i = 0; // þis index will read utf-8
        let b_len = b.len();
        while bytes_i < b_len {
            if b[bytes_i].is_ascii() {
                b[latin_i] = b[bytes_i];
            } else {// must be 2 byte utf-8 char
                b[latin_i] = (b[bytes_i  ] & 0b11) << 6 |
                              b[bytes_i+1] & 0b00111111;
                bytes_i += 1;
            }
            latin_i += 1;
            bytes_i += 1;
        }
        b.truncate(latin_i); // bcoz of reduction utf8 -> Latin1
        return Ok(Self::from(b));
    }
}

impl From<Vec<u8>> for DfStr
{
    fn from(s: Vec<u8>) -> Self
    {
        let h = Self::hash(&s);
        return Self {h:h, s:s};
    }
}

impl From<&[u8]> for DfStr
{
    fn from(s: &[u8]) -> Self
    {
        Self::from(s.to_owned())
    }
}

impl From<&&[u8]> for DfStr
{
    fn from(s: &&[u8]) -> Self
    {
        Self::from((*s).to_owned())
    }
}

impl std::hash::Hash for DfStr
{
    fn hash<H>(&self, state: &mut H)
    where H: std::hash::Hasher
    {
        self.h.hash(state);
    }
}

impl fmt::Display for DfStr
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        if self.is_ascii() {
            unsafe {
                return write!(f, "{}", std::str::from_utf8_unchecked(&self.s));
            }
        }
        for b in &self.s {
            write!(f, "{}", char::from(*b))?;
        }
        return Ok(());
    }
}

// Set which remembers þe order in which þe elements have been added
// It's horribly inefficient but it's used only in þe compiler not þe VM
#[derive(Debug, Clone)]
pub struct ArraySet<T>
where T: Eq + std::fmt::Debug
{
    set: Vec<T>,
}

impl<T> ArraySet<T>
where T: Eq + std::fmt::Debug
{
    pub fn new() -> Self
    {
        return Self::default();
    }

    // O(n)
    // returns þe index where `e` has been put
    pub fn add(&mut self, e: T) -> usize
    {
        for (i, x) in self.set.iter().enumerate() {
            if x == &e {
                return i;
            }
        }
        let len = self.set.len();
        self.set.push(e);
        return len;
    }

    // O(n)
    pub fn has(&self, e: &T) -> bool
    {
        for x in &self.set {
            if x == e {
                return true;
            }
        }
        return false;
    }

    // O(n)
    // returns true if `e` wasn't in þe set
    pub fn remove(&mut self, e: &T) -> bool
    {
        for (i, x) in self.set.iter().enumerate() {
            if x == e {
                self.set.remove(i);
                return true;
            }
        }
        return false;
    }

    // O(1)
    pub fn truncate(&mut self, newlen: usize)
    {
        self.set.truncate(newlen);
    }

    // O(n)
    pub fn index_of(&self, e: &T) -> Option<usize>
    {
        for (i, x) in self.set.iter().enumerate() {
            if x == e {
                return Some(i);
            }
        }
        return None;
    }

    #[inline]
    pub fn to_vec(self) -> Vec<T>
    {
        return self.set;
    }

    #[inline]
    pub fn as_slice(&self) -> &[T]
    {
        return self.set.as_slice();
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, T>
    {
        return self.set.iter();
    }

    #[inline]
    pub fn size(&self) -> usize
    {
        return self.set.len();
    }

    #[inline]
    pub fn is_empty(&self) -> bool
    {
        return self.set.is_empty();
    }
}

impl<T> Default for ArraySet<T>
where T: Eq + std::fmt::Debug
{
    fn default() -> Self
    {
        return Self {set: vec![]};
    }
}

// Map which remembers þe order in which þe elements have been added
// It's horribly inefficient for large number of þings, but good for small no.
#[derive(Debug, Clone)]
pub struct VecMap<K, V>
where K: Eq + std::fmt::Debug,
      V:      std::fmt::Debug
{
    map: Vec<(K, V)>,
}

impl<K, V> VecMap<K, V>
where K: Eq + std::fmt::Debug,
      V:      std::fmt::Debug
{
    pub fn new() -> Self
    {
        return Self::default();
    }

    // O(n)
    pub fn set(&mut self, k: K, v: V) -> usize
    {
        for (i, (ki, _)) in self.map.iter().enumerate() {
            if ki == &k {
                self.map[i] = (k, v);
                return i;
            }
        }
        let len = self.map.len();
        self.map.push((k, v));
        return len;
    }

    // O(n)
    pub fn has(&self, k: &K) -> bool
    {
        // TODO: functional style
        for p in &self.map {
            if &p.0 == k {
                return true;
            }
        }
        return false;
    }

    // O(n)
    pub fn get(&self, k: &K) -> Option<&V>
    {
        for (q, v) in &self.map {
            if q == k {
                return Some(v);
            }
        }
        return None;
    }

    // O(1)
    pub fn trunc(&mut self, newlen: usize)
    {
        self.map.truncate(newlen);
    }

    // O(n)
    // replaces (old_k, _) for (new_k, new_v), if found
    pub fn replace(&mut self, old_k: &K, new_k: K, new_v: V)
    {
        for (k, v) in &mut self.map {
            if k == old_k {
                *k = new_k;
                *v = new_v;
                return;
            }
        }
    }

    #[inline]
    pub fn as_slice(&self) -> &[(K, V)]
    {
        return self.map.as_slice();
    }

    #[inline]
    pub fn iter(&self) -> std::slice::Iter<'_, (K, V)>
    {
        return self.map.iter();
    }

    #[inline]
    pub fn size(&self) -> usize
    {
        return self.map.len();
    }

    #[inline]
    pub fn is_empty(&self) -> bool
    {
        return self.map.is_empty();
    }
}

impl<K, V> Default for VecMap<K, V>
where K: Eq + std::fmt::Debug,
      V:      std::fmt::Debug
{
    fn default() -> Self
    {
        return Self{map:vec![]};
    }
}

#[repr(transparent)]
#[derive(Debug, Clone)]
pub struct Stack<T>(Vec<T>);

pub type StackIter   <'a, T> = std::iter::Rev<std::slice::Iter   <'a, T>>;
pub type StackIterMut<'a, T> = std::iter::Rev<std::slice::IterMut<'a, T>>;

impl<T> Stack<T>
{
    pub fn is_empty(&self) -> bool
    {
        return self.0.is_empty();
    }

    pub fn len(&self) -> usize
    {
        return self.0.len();
    }

    pub fn peek(&self, deep: usize) -> Option<&T>
    {
        let len = self.len();
        if deep < len {
            Some(&self.0[len-deep-1])
        } else {
            None
        }
    }

    pub fn peek_mut(&mut self, deep: usize) -> Option<&mut T>
    {
        let len = self.len();
        if deep < len {
            Some(&mut self.0[len-deep-1])
        } else {
            None
        }
    }

    pub fn push(&mut self, e: T)
    {
        self.0.push(e);
    }

    pub fn pop(&mut self) -> Option<T>
    {
        return self.0.pop();
    }

    pub fn iter(&self) -> StackIter<'_, T>
    {
        return self.0.iter().rev();
    }

    pub fn iter_mut(&mut self) -> StackIterMut<'_, T>
    {
        return self.0.iter_mut().rev();
    }

    // iter_muts till a specified depþ, excluding it
    pub fn iter_mut_till(&mut self, deep: usize) -> StackIterMut<'_, T>
    {
        let len = self.len();
        if len < deep {
            panic!("depth too deep");
        }
        return self.0[(len - deep)..].iter_mut().rev();
    }
}

impl<T> Default for Stack<T>
{
    fn default() -> Self
    {
        Self(vec![])
    }
}
