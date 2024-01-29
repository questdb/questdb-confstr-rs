use std::fmt::Debug;
use std::iter::Fuse;

/// An iterator that allows peeking at the next two items.
#[derive(Clone, Debug)]
#[must_use = "iterator adaptors are lazy and do nothing unless consumed"]
pub(crate) struct Peekable2<I>
where
    I: Iterator,
{
    iter: Fuse<I>,
    buf: [Option<I::Item>; 2],
}

pub(crate) fn peekable2<I>(iterable: I) -> Peekable2<I::IntoIter>
where
    I: IntoIterator,
{
    let mut p2 = Peekable2 {
        iter: iterable.into_iter().fuse(),
        buf: [None, None],
    };

    p2.buf[0] = p2.iter.next();
    p2.buf[1] = p2.iter.next();

    p2
}

impl<I: Iterator> Peekable2<I> {
    pub(crate) fn peek0(&self) -> Option<&I::Item> {
        self.buf[0].as_ref()
    }

    pub(crate) fn peek1(&self) -> Option<&I::Item> {
        self.buf[1].as_ref()
    }
}

impl<I> Iterator for Peekable2<I>
where
    I: Iterator,
{
    type Item = I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let popped_item = self.buf[0].take();
        self.buf[0] = self.buf[1].take();
        self.buf[1] = self.iter.next();
        popped_item
    }
}

pub(crate) trait Peekable2Ext: Iterator {
    fn peekable2(self) -> Peekable2<Self>
    where
        Self: Sized,
    {
        peekable2(self)
    }
}

impl<I: Iterator> Peekable2Ext for I {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peekable2() {
        let v = vec![1, 2, 3, 4, 5];
        let mut iter = v.into_iter().peekable2();
        assert_eq!(iter.peek0(), Some(&1));
        assert_eq!(iter.peek1(), Some(&2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.peek0(), Some(&2));
        assert_eq!(iter.peek1(), Some(&3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.peek0(), Some(&3));
        assert_eq!(iter.peek1(), Some(&4));
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.peek0(), Some(&4));
        assert_eq!(iter.peek1(), Some(&5));
        assert_eq!(iter.next(), Some(4));
        assert_eq!(iter.peek0(), Some(&5));
        assert_eq!(iter.peek1(), None);
        assert_eq!(iter.next(), Some(5));
        assert_eq!(iter.peek0(), None);
        assert_eq!(iter.peek1(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.peek0(), None);
        assert_eq!(iter.peek1(), None);
        assert_eq!(iter.next(), None);
        assert_eq!(iter.peek0(), None);
        assert_eq!(iter.peek1(), None);
        assert_eq!(iter.next(), None);
    }
}
