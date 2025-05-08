/* util.rs */

use std::{fmt, ops::Deref};

// IDEA TODO crate derive_more has #[derive(From)] for newtypes

/*macro_rules! format_err {
    ($($args:expr),+) => (
        Err(String::from(format!($($args),+)))
    )
}

pub(crate) use format_err;*/

//pub type StrRes<T> = Result<T, String>;

#[derive(Clone, Eq, PartialEq)]
pub struct DfString(Vec<u8>);

impl DfString
{
    pub fn fast_from_string(s: String) -> Self
    {
        // warning: panics if error, because it will assume `s` is only Latin-1
        // (still in UTF-8) and will consume and rewrite it in u8 Latin-1.
        // It's "fast" because it only passes the string once, not twice as
        // when checking 1st þen convert it.
        let mut b = s.into_bytes();
        let mut latin_i = 0; // þis index will be writing u8s behind
        let mut bytes_i = 0; // þis index will read utf-8
        let b_len = b.len();
        // start rewriting!
        while bytes_i < b_len {
            if b[bytes_i].is_ascii() {
                b[latin_i] = b[bytes_i];
            } else if b[bytes_i] >> 2 == 0b110000 {
                // 2-byte utf-8 char with value <= 0xFF (Latin-1)
                // form: 110xxxyy 10yyzzzz
                // (xxxyyyyzzzz <= 0xFF) => (xxx = 000)
                // so 110000yy 10yyzzzz -> yyyyzzzz
                b[latin_i] = (b[bytes_i  ] & 0b11) << 6 |
                              b[bytes_i+1] & 0b00111111;
                bytes_i += 1;
            } else {
                panic!("not a Latin1 String");
            }
            latin_i += 1;
            bytes_i += 1;
        }
        b.truncate(latin_i); // bcoz of reduction utf8 -> Latin1
        return b.into(); // Vec<u8> -> Self
    }
}

impl fmt::Display for DfString
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", DfStr::from(self))
    }
}

impl fmt::Debug for DfString
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "DfString(\"{}\")", self)
    }
}

impl AsRef<[u8]> for DfString
{
    fn as_ref(&self) -> &[u8]
    {
        return &self.0;
    }
}

impl From<Vec<u8>> for DfString
{
    fn from(v: Vec<u8>) -> Self
    {
        return Self(v);
    }
}

impl From<DfStr<'_>> for DfString
{
    fn from(s: DfStr<'_>) -> Self
    {
        return Self(s.as_ref().to_vec());
    }
}

// ---------------------------------

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct DfStr<'a>(pub &'a [u8]);

impl<'a> From<&'a [u8]> for DfStr<'a>
{
    fn from(b: &'a [u8]) -> Self
    {
        return Self(b);
    }
}

impl<'a> From<&'a DfString> for DfStr<'a>
{
    fn from(s: &'a DfString) -> Self
    {
        Self(s.as_ref())
    }
}

impl<'a> From<&'a Symbol> for DfStr<'a>
{
    fn from(s: &'a Symbol) -> Self
    {
        Self(s.s.as_ref())
    }
}

impl<'a> From<Sym<'a>> for DfStr<'a>
{
    fn from(s: Sym<'a>) -> Self
    {
        return s.s;
    }
}

// print bytes as Latin-1
impl fmt::Display for DfStr<'_>
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        // TODO optimiz'it with ascii chunks
        for c in self.0 {
            write!(f, "{}", *c as char)?;
        }
        Ok(())
    }
}

impl AsRef<[u8]> for DfStr<'_>
{
    fn as_ref(&self) -> &[u8]
    {
        return self.0;
    }
}

// Symbol stuff -------------------------

#[inline]
fn hash_bytes(b: &[u8]) -> u32
{
    // 32-bit FNV
    b   .iter()
        .fold(2166136261_u32,
            |hash, c| (hash ^ *c as u32).wrapping_mul(16777619)
        )
}

pub type SymId = u16;

// a hashed DfString
#[derive(Clone)]
pub struct Symbol
{
    s: DfString, // string in Latin-1
    h: u32,      // hash, usefull for Eq, Hash, etc.
}

impl Symbol
{
    pub fn hash(&self) -> u32
    {
        return self.h;
    }
}

impl PartialEq for Symbol
{
    fn eq(&self, other: &Symbol) -> bool
    {
        return self == other;
    }
}

// TODO impl PartialEq<rhs = Sym<'_>> for Symbol

impl Eq for Symbol {}

impl From<Vec<u8>> for Symbol
{
    fn from(s: Vec<u8>) -> Self
    {
        let h = hash_bytes(&s);
        let s = s.into();
        return Self {s, h};
    }
}

impl From<&[u8]> for Symbol
{
    fn from(s: &[u8]) -> Self
    {
        return s.to_vec().into();
    }
}

impl From<Sym<'_>> for Symbol
{
    fn from(s: Sym<'_>) -> Self
    {
        return Self { s: DfStr::from(s).into(), h: s.hash() };
    }
}

impl PartialEq<Sym<'_>> for Symbol
{
    fn eq(&self, other: &Sym<'_>) -> bool
    {
        return Sym::from(self) == *other;
    }
}

impl PartialEq<Symbol> for Sym<'_>
{
    fn eq(&self, other: &Symbol) -> bool
    {
        return *self == Sym::from(other);
    }
}

impl std::hash::Hash for Symbol
{
    fn hash<H>(&self, state: &mut H)
    where H: std::hash::Hasher
    {
        self.h.hash(state);
    }
}

/*// so it can use Display for DfString
impl Deref for Symbol
{
    type Target = DfString;
    fn deref(&self) -> &Self::Target
    {
        return &self.s;
    }
}*/

impl fmt::Debug for Symbol
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "Symbol({})", self.s)
    }
}

// borrowed Symbol
#[derive(Copy, Clone)]
pub struct Sym<'a>
{
    pub s: DfStr<'a>,
    h: u32,
}

impl<'a> Sym<'a>
{
    pub fn hash(&self) -> u32
    {
        return self.h;
    }
}

impl<'a> From<DfStr<'a>> for Sym<'a>
{
    fn from(s: DfStr<'a>) -> Self
    {
        let h = hash_bytes(s.as_ref());
        return Self {s, h};
    }
}

impl<'a> From<&'a Symbol> for Sym<'a>
{
    fn from(symbol: &'a Symbol) -> Self
    {
        return Sym {s:symbol.into(), h:symbol.hash()};
    }
}

impl<'a> PartialEq for Sym<'a>
{
    fn eq(&self, other: &Sym<'a>) -> bool
    {
        // here's þe trick: 1st compare þe pre-computed hashes
        return self.h == other.h && self.s == other.s;
    }
}

// so it can use Display for DfString
impl<'a> Deref for Sym<'a>
{
    type Target = DfStr<'a>;
    fn deref(&self) -> &Self::Target
    {
        return &self.s;
    }
}

impl<'a> From<&'a [u8]> for Sym<'a>
{
    fn from(b: &'a [u8]) -> Sym<'a>
    {
        return Sym {s:b.into(), h:hash_bytes(b)};
    }
}

// ------------------------------------------

// Set which remembers þe order in which þe elements have been added
// It's horribly inefficient but it's used only in þe compiler not þe VM
#[derive(Clone)]
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
        if let Some(i) = self.index_of(&e) {
            return i;
        } else {
            let len = self.set.len();
            self.set.push(e);
            return len;
        }
    }

    // Similar but optimized for not needing to construct a new <T>
    // if self already contains an element Eq to e
    pub fn add_clone<S>(&mut self, e: S) -> usize
    where S: PartialEq<T> + Into<T>, T: PartialEq<S>
    {
        if let Some(i) = self.index_of(&e) {
            return i;
        } else {
            let len = self.set.len();
            self.set.push(e.into());
            return len;
        }
    }

/*    // O(n)
    pub fn has(&self, e: &T) -> bool
    {
        return self.set.iter().find(|&x| x == e).is_some();
    }*/

/*    // O(n)
    // returns true if `e` wasn't in þe set
    pub fn remove(&mut self, e: &T) -> bool
    {
        let pos = self.set.iter().position(|x| x == e);
        if let Some(i) = pos {
            self.set.remove(i);
        }
        return pos.is_some();
    }*/

/*    // O(1)
    pub fn truncate(&mut self, newlen: usize)
    {
        self.set.truncate(newlen);
    }*/

    // O(n)
    pub fn index_of<S>(&self, e: &S) -> Option<usize>
    where S: PartialEq<T>, T: PartialEq<S>
    {
        return self.set.iter().position(|x| x == e);
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
        if let Some(i) = self.map.iter().position(|pair| pair.0 == k) {
            self.map[i] = (k, v);
            return i;
        }
        let len = self.map.len();
        self.map.push((k, v));
        return len;
    }

    // O(n)
    pub fn has(&self, k: &K) -> bool
    {
        return self.get(k).is_some();
    }

    // O(n)
    pub fn get(&self, k: &K) -> Option<&V>
    {
        return self.map
            .iter()
            .find(|pair| &pair.0 == k)
            .map(|pair| &pair.1);
    }

    // O(1)
    pub fn trunc(&mut self, newlen: usize)
    {
        self.map.truncate(newlen);
    }

    // O(n)
    // replaces (old_k, _) for (new_k, new_v), if found
/*    pub fn replace(&mut self, old_k: &K, new_k: K, new_v: V)
    {
//        if let Some(old_pair) = self TODO
        for (k, v) in &mut self.map {
            if k == old_k {
                *k = new_k;
                *v = new_v;
                return;
            }
        }
    }*/

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
