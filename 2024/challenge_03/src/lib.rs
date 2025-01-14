use std::cell::RefCell;
use std::future::Future;
use std::ops::{Deref, DerefMut};
use std::pin::Pin;
use std::task::{Context, Poll};

/// Неблокиращ мутекс, предназначен да се използва от асинхронен код.
pub struct MyMutex<T> {
    value: RefCell<T>,
    /* todo: other fields */
}

impl<T> MyMutex<T> {
    pub fn new(value: T) -> Self {
        todo!()
    }

    // Забележете, че `lock` не е маркирана като `async fn`, защото си имплементираме future-а
    // на ръка (тук компилатора няма как да ни помогне).
    //
    // Бихме могли да я декларираме `fn lock() -> impl Future<Output = MyMytexGuard<'_, T>>`,
    // ако искаме да не правим структурата публична, но пак ще трябва да си напишем и върнем
    // наша структура.
    pub fn lock(&self) -> MyMutexLockFuture<'_, T> {
        todo!()
    }

    fn unlock(&self) {
        todo!()
    }
}

pub struct MyMutexLockFuture<'a, T> {
    /* todo: fields */
}

impl<'a, T> Future for MyMutexLockFuture<'a, T> {
    type Output = MyMutexGuard<'a, T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        todo!()
    }
}

pub struct MyMutexGuard<'a, T> {
    /* todo: fields */
}

impl<'a, T> Deref for MyMutexGuard<'a, T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        todo!()
    }
}

impl<'a, T> DerefMut for MyMutexGuard<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        todo!()
    }
}

impl<'a, T> Drop for MyMutexGuard<'a, T> {
    fn drop(&mut self) {
        // hint: извикайте MyMytex::unlock
        todo!()
    }
}

