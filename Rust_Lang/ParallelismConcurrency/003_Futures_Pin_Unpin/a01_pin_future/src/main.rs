use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll, RawWaker, RawWakerVTable, Waker},
};

struct Telephone {
    number: i32,
}

struct CallerFuture {
    tel: Telephone,
    tel_ptr: *const Telephone, // raw self-reference
    state: u8,
}

impl CallerFuture {
    fn new() -> Pin<Box<Self>> {
        let mut fut = Box::pin(CallerFuture {
            tel: Telephone { number: 777 },
            tel_ptr: std::ptr::null(),
            state: 0,
        });

        // initialize self-reference AFTER pinning
        let tel_ptr = &fut.tel as *const Telephone;

        unsafe {
            let fut_mut = Pin::get_unchecked_mut(fut.as_mut());
            fut_mut.tel_ptr = tel_ptr;
        }

        fut
    }
}

impl Future for CallerFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<()> {
        let this = unsafe { self.get_unchecked_mut() };

        if this.state == 0 {
            this.state = 1;
            return Poll::Pending;
        }

        unsafe {
            println!("tel address inside future = {:p}", this.tel_ptr);
            println!("number = {}", (*this.tel_ptr).number);
        }

        Poll::Ready(())
    }
}

// minimal dummy waker
fn dummy_waker() -> Waker {
    unsafe fn clone(_: *const ()) -> RawWaker {
        RawWaker::new(std::ptr::null(), &VTABLE)
    }
    unsafe fn wake(_: *const ()) {}
    unsafe fn wake_by_ref(_: *const ()) {}
    unsafe fn drop(_: *const ()) {}

    static VTABLE: RawWakerVTable = RawWakerVTable::new(clone, wake, wake_by_ref, drop);

    unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
}

fn main() {
    let mut fut = CallerFuture::new();

    println!("future address (pinned) = {:p}", &*fut);
    println!("tel address (direct)    = {:p}", &fut.tel);

    let waker = dummy_waker();
    let mut cx = Context::from_waker(&waker);

    // first poll
    assert!(matches!(fut.as_mut().poll(&mut cx), Poll::Pending));

    // second poll
    assert!(matches!(fut.as_mut().poll(&mut cx), Poll::Ready(())));
}
