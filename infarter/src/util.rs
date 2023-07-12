/* src/util.rs */

#[derive(Debug, Copy, Clone)]
pub struct Vec16<T>
where T: Sized /*+ Copy*/ + Clone + Default
{
    vec: [T; 16],
    len: usize,
}

impl<T> Vec16<T>
where T: Sized /*+ Copy*/ + Clone + Default
{
    pub fn new() -> Self
    {
        let d: T = Default::default();
        return Self {
            vec: [d; 16],
            len: 0,
        };
    }

    pub fn len(&self) -> usize
    {
        return self.len;
    }

    pub fn as_slice(&self) -> &[T]
    {
        return &self.vec[0..self.len];
    }

    pub fn get(&self, i: usize) -> &T
    {
        if i >= self.len {
            panic!("ERROR: Vec16<T>::set: out of bounds");
        }
        return &self.vec[i];
    }

    pub fn set(&mut self, i: usize, e: &T)
    {
        if i >= self.len {
            panic!("ERROR: Vec16<T>::set: out of bounds");
        }
        self.vec[i] = *e;
    }

    pub fn push(&mut self, e: &T)
    {
        if self.len == 15 {
            panic!("ERROR: Vec16<T>::push: stack overflow")
        } else {
            self.vec[self.len] = *e;
            self.len += 1;
        }
    }

    pub fn pop(&mut self) -> &T
    {
        if self.len == 0 {
            panic!("ERROR: Vec16<T>::pop: empty stack");
        } else {
            self.len -= 1;
            return &self.vec[self.len];
        }
    }
}

impl<T> std::ops::Index<usize> for Vec16<T>
where T: Sized /*+ Copy*/ + Clone + Default
{
    type Output = T;
    fn index(&self, i: usize) -> &T
    {
        return self.get(i);
    }
}
/*
impl<T> Iterator for Vec16<T>
where T: Sized + Copy + Clone + Default
{
    type Item = usize;
    fn next(&mut self) -> Option<Self::Item>
}
*/
