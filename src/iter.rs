use std::marker::PhantomData;
use std::mem;

pub struct DefaultIfEmpty<T, I, F> {
    source: I,
    default: Option<F>,
    data: PhantomData<T>,
}

impl<T, I, F> Iterator for DefaultIfEmpty<T, I, F>
    where I: Iterator<Item = T>,
          F: FnOnce() -> T
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        match self.source.next() {
            Some(item) => {
                if self.default.is_some() {
                    self.default = None;
                }

                Some(item)
            }

            None => {
                if self.default.is_some() {
                    let mut default_fn = None;
                    mem::swap(&mut default_fn, &mut self.default);
                    default_fn.map(|f| f())
                } else {
                    None
                }
            }
        }
    }
}

pub trait DefaultIter<T>
    where Self: Sized
{
    fn default<F: FnOnce() -> T>(self, f: F) -> DefaultIfEmpty<T, Self, F>;
}

impl<I: Iterator<Item = T>, T> DefaultIter<T> for I {
    fn default<F: FnOnce() -> T>(self, f: F) -> DefaultIfEmpty<T, Self, F> {
        DefaultIfEmpty {
            source: self,
            default: Some(f),
            data: PhantomData,
        }
    }
}
