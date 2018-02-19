use std::any::TypeId;
use std::cell::RefCell;
use std::hash::{BuildHasherDefault, Hasher};
use std::collections::HashMap;

pub struct Spawn<T: ?Sized> {
    id: usize,
    data: LocalMap,
    obj: T,
}

pub fn spawn<T>(obj: T) -> Spawn<T> {
    Spawn {
        id: fresh_task_id(),
        obj: obj,
        data: local_map(),
    }
}

impl<T: ?Sized> Spawn<T> {
    pub fn poll_future_notify<N>(&mut self,
                                 notify: &N,
                                 id: usize) -> Poll<T::Item, T::Error>
        where N: Clone + Into<NotifyHandle>,
              T: Future,
    {
        let mk = || notify.clone().into();
        self.enter(BorrowedUnpark::new(&mk, id), |f| f.poll())
    }

    fn enter<F, R>(&mut self, unpark: BorrowedUnpark, f: F) -> R
        where F: FnOnce(&mut T) -> R
    {
        let borrowed = BorrowedTask {
            id: self.id,
            unpark: unpark,
            events: BorrowedEvents::new(),
            map: &self.data,
        };
        let obj = &mut self.obj;
        set(&borrowed, || f(obj))
    }
}

fn fresh_task_id() -> usize {
    use core::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

    static NEXT_ID: AtomicUsize = ATOMIC_USIZE_INIT;
    let id = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    assert!(id < usize::max_value() / 2,
            "too many previous tasks have been allocated");
    id
}

pub fn local_map() -> LocalMap {
    RefCell::new(HashMap::default())
}

pub type LocalMap = RefCell<HashMap<TypeId,
                                    Box<Opaque>,
                                    BuildHasherDefault<IdHasher>>>;

pub trait Opaque: Send {}
impl<T: Send> Opaque for T {}

pub fn local_map() -> LocalMap {
    RefCell::new(HashMap::default())
}

pub struct IdHasher {
    id: u64,
}

impl Default for IdHasher {
    fn default() -> IdHasher {
        IdHasher { id: 0 }
    }
}

impl Hasher for IdHasher {
    fn write(&mut self, _bytes: &[u8]) {
        // TODO: need to do something sensible
        panic!("can only hash u64");
    }

    fn write_u64(&mut self, u: u64) {
        self.id = u;
    }

    fn finish(&self) -> u64 {
        self.id
    }
}

pub trait Future {
    type Item;
    type Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error>;

    #[cfg(feature = "use_std")]
    fn wait(self) -> result::Result<Self::Item, Self::Error>
        where Self: Sized
    {
        ::executor::spawn(self).wait_future()
    }
}


pub struct NotifyHandle {
    inner: *mut UnsafeNotify,
}

unsafe impl Send for NotifyHandle {}
unsafe impl Sync for NotifyHandle {}

impl NotifyHandle {
    #[inline]
    pub unsafe fn new(inner: *mut UnsafeNotify) -> NotifyHandle {
        NotifyHandle { inner: inner }
    }

    pub fn notify(&self, id: usize) {
        unsafe { (*self.inner).notify(id) }
    }

    fn clone_id(&self, id: usize) -> usize {
        unsafe { (*self.inner).clone_id(id) }
    }

    fn drop_id(&self, id: usize) {
        unsafe { (*self.inner).drop_id(id) }
    }
}

impl Clone for NotifyHandle {
    #[inline]
    fn clone(&self) -> Self {
        unsafe {
            (*self.inner).clone_raw()
        }
    }
}

impl fmt::Debug for NotifyHandle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("NotifyHandle")
         .finish()
    }
}

impl Drop for NotifyHandle {
    fn drop(&mut self) {
        unsafe {
            (*self.inner).drop_raw()
        }
    }
}

pub unsafe trait UnsafeNotify: Notify {
    unsafe fn clone_raw(&self) -> NotifyHandle;
    unsafe fn drop_raw(&self);
}