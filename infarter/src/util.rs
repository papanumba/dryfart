/* src/util.rs */

#[derive(Debug, Copy, Clone)]
pub struct StaVec<const N: usize, T>
where T: Sized + Copy + Clone + Default
{
    vec: [T; N],
    len: usize,
}

impl<const N: usize, T> StaVec<N, T>
where T: Sized + Copy + Clone + Default
{
    #[inline]
    pub fn new() -> Self
    {
        let d: T = Default::default();
        return Self {
            vec: [d; N],
            len: 0,
        };
    }

    #[inline]
    pub fn len(&self) -> usize
    {
        return self.len;
    }

    #[inline]
    pub fn as_slice(&self) -> &[T]
    {
        return &self.vec[0..self.len];
    }

    #[inline]
    pub fn get(&self, i: usize) -> &T
    {
        if i >= self.len {
            panic!("ERROR: StaVec::set: out of bounds");
        }
        return &self.vec[i];
    }

    #[inline]
    pub fn set(&mut self, i: usize, e: &T)
    {
        if i >= self.len {
            panic!("ERROR: StaVec::set: out of bounds");
        }
        self.vec[i] = *e;
    }

    #[inline]
    pub fn push(&mut self, e: &T)
    {
        if self.len + 1 == N {
            panic!("ERROR: StaVec::push: stack overflow")
        } else {
            self.vec[self.len] = *e;
            self.len += 1;
        }
    }

    #[inline]
    pub fn pop(&mut self) -> &T
    {
        if self.len == 0 {
            panic!("ERROR: StaVec::pop: empty stack");
        } else {
            self.len -= 1;
            return &self.vec[self.len];
        }
    }
}

impl<const N: usize, T> std::ops::Index<usize> for StaVec<N, T>
where T: Sized + Copy + Clone + Default
{
    type Output = T;
    fn index(&self, i: usize) -> &T
    {
        return self.get(i);
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
        return Self {set: vec![]};
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
/*        if newlen > self.size() {
            panic!("cannot truncate ArraySet to len bigger");
        }*/
        self.set.truncate(newlen);
    }

    // O(n)
    pub fn index_of(&mut self, e: &T) -> Option<usize>
    {
        for (i, x) in self.set.iter().enumerate() {
            if x == e {
                return Some(i);
            }
        }
        return None;
    }

    #[inline]
    pub fn as_slice(&self) -> &[T]
    {
        return self.set.as_slice();
    }

    #[inline]
    pub fn size(&self) -> usize
    {
        return self.set.len();
    }
}
