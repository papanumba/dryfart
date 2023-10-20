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
