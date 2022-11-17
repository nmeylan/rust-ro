#![allow(dead_code)]

use std::cell::{Cell, UnsafeCell};
use std::marker::PhantomData;
use std::ops::{Deref, DerefMut};
use std::ptr::NonNull;

/// Works like RefCell but can be borrow and borrow_mut at the same time.
/// Should be used with caution
pub struct MyUnsafeCell<T: ?Sized> {
    borrow: Cell<isize>,
    value: UnsafeCell<T>
}

pub struct MyRef<'b, T: ?Sized + 'b> {
    value: NonNull<T>,
    borrow: MyBorrowRef<'b>,
}

pub struct MyRefMut<'b, T: ?Sized + 'b> {
    // NB: we use a pointer instead of `&'b mut T` to avoid `noalias` violations, because a
    // `RefMut` argument doesn't hold exclusivity for its whole scope, only until it drops.
    value: NonNull<T>,
    borrow: MyBorrowRefMut<'b>,
    // `NonNull` is covariant over `T`, so we need to reintroduce invariance.
    marker: PhantomData<&'b mut T>,
}

pub struct MyBorrowRef<'b> {
    borrow: &'b Cell<isize>,
}

pub struct MyBorrowRefMut<'b> {
    borrow: &'b Cell<isize>,
}


impl <T> MyUnsafeCell<T> {
    pub const fn new(value: T) -> MyUnsafeCell<T> {
        MyUnsafeCell {
            value: UnsafeCell::new(value),
            borrow: Cell::new(0)
        }
    }
}

impl<T: Default> Default for MyUnsafeCell<T> {
    /// Creates a `RefCell<T>`, with the `Default` value for T.
    #[inline]
    fn default() -> MyUnsafeCell<T> {
        MyUnsafeCell::new(Default::default())
    }
}

impl<T: ?Sized> MyUnsafeCell<T> {
    pub fn borrow(&self) -> MyRef<'_, T> {
        let value = unsafe { NonNull::new_unchecked(self.value.get()) };
        MyRef { value, borrow: MyBorrowRef::new(&self.borrow) }
    }
    pub fn borrow_mut(&self) -> MyRefMut<'_, T> {
        let value = unsafe { NonNull::new_unchecked(self.value.get()) };
        MyRefMut { value, borrow: MyBorrowRefMut::new(&self.borrow), marker: PhantomData }
    }
}
impl<T: Clone> Clone for MyUnsafeCell<T> {
    fn clone(&self) -> MyUnsafeCell<T> {
        MyUnsafeCell::new(self.borrow().clone())
    }
}

impl<T: ?Sized> Deref for MyRef<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: the value is accessible as long as we hold our borrow.
        unsafe { self.value.as_ref() }
    }
}

impl<T: ?Sized> Deref for MyRefMut<'_, T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &T {
        // SAFETY: the value is accessible as long as we hold our borrow.
        unsafe { self.value.as_ref() }
    }
}

impl<T: ?Sized> DerefMut for MyRefMut<'_, T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut T {
        // SAFETY: the value is accessible as long as we hold our borrow.
        unsafe { self.value.as_mut() }
    }
}

impl<'b> MyBorrowRef<'b> {
    #[inline]
    fn new(borrow: &'b Cell<isize>) -> MyBorrowRef<'b> {
        let b = borrow.get().wrapping_add(1);
        borrow.set(b);
        MyBorrowRef { borrow }
    }
}

impl Drop for MyBorrowRef<'_> {
    #[inline]
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        self.borrow.set(borrow - 1);
    }
}

impl Drop for MyBorrowRefMut<'_> {
    #[inline]
    fn drop(&mut self) {
        let borrow = self.borrow.get();
        self.borrow.set(borrow - 1);
    }
}


impl<'b> MyBorrowRefMut<'b> {
    #[inline]
    fn new(borrow: &'b Cell<isize>) -> MyBorrowRefMut<'b> {
        // It behave like MyBorrowRef
        let b = borrow.get().wrapping_add(1);
        borrow.set(b);
        MyBorrowRefMut { borrow }
    }
}


impl<'b, T: ?Sized> MyRef<'b, T> {
    pub fn map<U: ?Sized, F>(orig: MyRef<'b, T>, f: F) -> MyRef<'b, U>
        where
            F: FnOnce(&T) -> &U,
    {
        MyRef { value: NonNull::from(f(&*orig)), borrow: orig.borrow }
    }
}