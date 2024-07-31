use num::Num;

pub struct DigitIterator<T>
where
    T: Num,
{
    current: T,
    base: T,
}

impl<T> DigitIterator<T>
where
    T: Num,
{
    fn new(n: T, base: T) -> Self {
        DigitIterator { current: n, base }
    }
}

impl<T> Iterator for DigitIterator<T>
where
    T: Num + Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == T::zero() {
            None
        } else {
            let digit = self.current % self.base;
            self.current = self.current / self.base;
            Some(digit)
        }
    }
}

pub fn digits<T>(n: T, base: T) -> DigitIterator<T>
where
    T: Num,
{
    DigitIterator::new(n, base)
}
