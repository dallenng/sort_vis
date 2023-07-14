use std::cell::{Cell, Ref, RefCell, RefMut};
use std::cmp::Ordering;
use std::collections::Bound;
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::ops::{Deref, RangeBounds};
use std::rc::Rc;
use std::time::{Duration, Instant};

use futures_lite::future::yield_now;

pub trait Slice<T> {
    type SubSlice<'a>: Slice<T>
    where
        Self: 'a;

    type Output<'a>: Deref<Target = T>;

    fn len(&self) -> usize;

    fn is_empty(&self) -> bool;

    async fn get(&self, index: usize) -> Self::Output<'_>;

    async fn swap(&mut self, a: usize, b: usize);

    fn slice(&mut self, range: impl RangeBounds<usize>) -> Self::SubSlice<'_>;

    fn split_at(&mut self, mid: usize) -> (Self::SubSlice<'_>, Self::SubSlice<'_>);

    async fn binary_search(&self, x: &T) -> Result<usize, usize>
    where
        T: Ord;

    async fn is_sorted(&self) -> bool
    where
        T: PartialOrd<T>;
}

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Default)]
pub struct SharedSlice {
    inner: Rc<RefCell<Box<[Item]>>>,
}

impl FromIterator<u16> for SharedSlice {
    fn from_iter<T: IntoIterator<Item = u16>>(iter: T) -> Self {
        Self { inner: Rc::new(RefCell::new(iter.into_iter().map(Item::from).collect())) }
    }
}

impl SharedSlice {
    pub fn inner(&self) -> Ref<'_, [Item]> {
        Ref::map(self.inner.borrow(), |inner| inner.as_ref())
    }
}

#[derive(Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SharedSubSlice<'s> {
    slice: SharedSlice,
    start: usize,
    end: usize,
    _phantom: PhantomData<&'s mut SharedSlice>,
}

impl<'s> From<&'s mut SharedSlice> for SharedSubSlice<'s> {
    fn from(slice: &'s mut SharedSlice) -> Self {
        Self {
            slice: slice.clone(),
            start: 0,
            end: slice.inner.borrow().len(),
            _phantom: PhantomData,
        }
    }
}

impl<'s> Slice<u16> for SharedSubSlice<'s> {
    type SubSlice<'a> = SharedSubSlice<'a> where Self: 'a;

    type Output<'a> = Ref<'a, u16>;

    fn len(&self) -> usize {
        self.slice().len()
    }

    fn is_empty(&self) -> bool {
        self.slice().is_empty()
    }

    async fn get(&self, index: usize) -> Self::Output<'_> {
        yield_now().await;

        Ref::map(self.slice(), |slice| {
            let item = &slice[index];
            item.last_access.set(Some(Instant::now()));
            &item.value
        })
    }

    async fn swap(&mut self, a: usize, b: usize) {
        yield_now().await;

        let mut slice = self.slice_mut();
        let now = Instant::now();
        slice[a].last_access.set(Some(now));
        slice[b].last_access.set(Some(now));

        slice.swap(a, b);
    }

    fn slice(&mut self, range: impl RangeBounds<usize>) -> Self::SubSlice<'_> {
        let len = self.len();

        let start = match range.start_bound() {
            Bound::Included(start) => self.start + start,
            Bound::Excluded(start) => self.start + start + 1,
            Bound::Unbounded => self.start,
        };

        let end = match range.end_bound() {
            Bound::Included(end) => self.start + end + 1,
            Bound::Excluded(end) => self.start + end,
            Bound::Unbounded => self.end,
        };

        assert!(start <= end, "range start must not be greater than end: {start:?} <= {end:?}");
        assert!(end <= len, "range end out of bounds: {end:?} <= {len:?}");

        Self { slice: self.slice.clone(), start, end, _phantom: PhantomData }
    }

    fn split_at(&mut self, mid: usize) -> (Self::SubSlice<'_>, Self::SubSlice<'_>) {
        let len = self.len();
        let mid = self.start + mid;

        assert!(mid <= len, "mid out of bounds: {mid:?} <= {len:?}");

        (
            Self { slice: self.slice.clone(), start: self.start, end: mid, _phantom: PhantomData },
            Self { slice: self.slice.clone(), start: mid, end: self.end, _phantom: PhantomData },
        )
    }

    async fn binary_search(&self, x: &u16) -> Result<usize, usize> {
        let mut len = self.len();
        let mut left = 0;
        let mut right = len;

        while left < right {
            let mid = left + len / 2;

            match self.get(mid).await.cmp(x) {
                Ordering::Less => left = mid + 1,
                Ordering::Greater => right = mid,
                Ordering::Equal => return Ok(mid),
            }

            len = right - left;
        }

        Err(left)
    }

    async fn is_sorted(&self) -> bool {
        let mut last = if self.is_empty() { return true } else { *self.get(0).await };

        for i in 1..self.len() {
            let current = *self.get(i).await;

            if last > current {
                return false;
            }

            last = current;
        }

        true
    }
}

impl<'s> SharedSubSlice<'s> {
    fn slice(&self) -> Ref<'_, [Item]> {
        Ref::map(self.slice.inner.borrow(), |inner| &inner[self.start..self.end])
    }

    fn slice_mut(&mut self) -> RefMut<'_, [Item]> {
        RefMut::map(self.slice.inner.borrow_mut(), |inner| &mut inner[self.start..self.end])
    }
}

#[derive(Debug, Clone, Default)]
pub struct Item {
    value: u16,
    last_access: Cell<Option<Instant>>,
}

impl Eq for Item {}

impl PartialEq for Item {
    fn eq(&self, other: &Self) -> bool {
        self.value.eq(&other.value)
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Hash for Item {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl From<u16> for Item {
    fn from(value: u16) -> Self {
        Self { value, last_access: Cell::new(None) }
    }
}

impl Item {
    pub fn value(&self) -> u16 {
        self.value
    }

    pub fn last_access_elapsed(&self) -> Duration {
        self.last_access.get().map_or(Duration::MAX, |last_access| last_access.elapsed())
    }
}
