Имплементирайте async-aware мутекс.

Това изисква скок зад кулисите на async/await машинарията, защото ще трябва да се имплементира един `Future` на ръка. Но от друга страна е добро упражнение, ако човек иска да добие интуиция как работят future-ите в езика.

За по-лесно, ще искаме въпросния мутекс да не е thread safe, т.е. да може да се използва само от single threaded runtime.

```rust
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
```

Зашо използваме `RefCell` за `value`? Защото ни дава всичката необходима функционалност. За да имплементираме мутекса е нужно да можем от `&MyMutex<T>` да вземем `&mut T`, за което е нужен някакъв вид internal mutability.  
Бихме могли да използваме `std::sync::Mutex`, но няма да го използваме пълноценно. Или дори `UnsafeCell`, но това изисква unsafe код и просто ще преимплементираме RefCell (но би имало смисъл, ако правим thread safe вариант).

Изискването за мутекса е когато две задачи (task-а) се опитат да го заключат едновременно, т.е. да извикат `my_mutex.lock().await`, едната задача ще получи `MyMutexGuard` и ще продължи изпълнението си, докато другата ще бъде "блокирана" и ще трябва да изчака, докато мутекса не се освободи.  
За да се получи това, при poll-ването на future-а, върнат от `my_mutex.lock()`, във втората задача трябва да се върне `Poll::Pending`, което означава, че задачата за момента не може да продължи работата си. Съответно async runtime-а повече няма да schedule-ва тази задача, но е свободен да изпълнява други задачи. Когато обаче мутекса се освободи, runtime-а трябва да бъде уведомен, че втората задача вече може да направи прогрес. За целта трябва да се вземе `Waker` обекта за съответната задача, който може да бъде получен от `Context` параметъра на `poll`, и да се запази до момента, в който задачата трябва да бъде събудена.

Целия тест можете да намерите на <https://github.com/fmi/rust-homework>.  
Внимание - теста използва `futures` библиотеката, но тя е достъпна само за тестовете, но не и за решението (добавена е като dev-dependency). В решението не можете да използвате външни библиотеки - но не са ви и нужни.
