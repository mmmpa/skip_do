use std::iter::Peekable;

#[cfg(test)]
mod tests {
    use crate::SkipDo;
    #[test]
    fn it_works() {
        let mut v = vec![1, 2, 3, 4];
        let mut v2 = vec![];

        v = v
            .into_iter()
            .skip_do(|x| x <= &2, |x| v2.push(x))
            .map(|x| x)
            .collect();

        assert_eq!(v, [3, 4]);
        assert_eq!(v2, [1, 2]);
    }
}

/// An iterator that skip items and use skipped items.
///
/// ```rust
/// use skip_do::SkipDo;
///
/// let mut v = vec![1, 2, 3, 4];
/// let mut v2 = vec![];
///
/// v = v
///     .into_iter()
///     .skip_do(|x| x <= &2, |x| v2.push(x))
///     .map(|x| x)
///     .collect();
///
/// assert_eq!(v, [3, 4]);
/// assert_eq!(v2, [1, 2]);
/// ```
pub struct SkipDoIterator<I: Iterator, S: Fn(&I::Item) -> bool, W: FnMut(I::Item)> {
    iter: Peekable<I>,
    skip_while: S,
    work: W,
    skipped: bool,
}

impl<IT: Iterator, F: Fn(&IT::Item) -> bool, DO: FnMut(IT::Item)> Iterator
    for SkipDoIterator<IT, F, DO>
{
    type Item = IT::Item;

    fn next(&mut self) -> Option<Self::Item> {
        if self.skipped {
            return self.iter.next();
        }

        self.skipped = true;

        loop {
            match self.iter.peek() {
                None => break,
                Some(x) => {
                    if (self.skip_while)(x) {
                        (self.work)(self.iter.next().unwrap());
                    } else {
                        break;
                    }
                }
            }
        }

        self.iter.next()
    }
}

impl<IT: Iterator> SkipDo for IT {}
pub trait SkipDo: Iterator {
    fn skip_do<S, W>(self, skip_while: S, work: W) -> SkipDoIterator<Self, S, W>
    where
        Self: std::marker::Sized + Iterator,
        S: Fn(&Self::Item) -> bool,
        W: FnMut(Self::Item),
    {
        SkipDoIterator {
            iter: self.peekable(),
            skip_while,
            work,
            skipped: false,
        }
    }
}
