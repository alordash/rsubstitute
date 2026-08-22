use rsubstitute::{Arg, Mockable, mock};
use rsubstitute_core::Times;
use std::pin::Pin;
use std::sync::atomic::{AtomicI32, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll, Waker};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

#[mock]
struct MyFuture {
    atomic_result: Arc<AtomicI32>,
    atomic_waker: Arc<Mutex<Option<Waker>>>,
    thread: JoinHandle<()>,
}

#[mock(base)]
impl MyFuture {
    pub fn new(value: i32) -> MyFuture {
        let atomic_result = Arc::new(AtomicI32::new(0));
        let atomic_waker = Arc::new(Mutex::new(None));
        let atomic_result_clone = atomic_result.clone();
        let atomic_waker_clone = atomic_waker.clone();
        MyFuture {
            atomic_waker,
            atomic_result,
            thread: thread::spawn(move || {
                thread::sleep(Duration::from_millis(100));

                atomic_result_clone.store(value, Ordering::SeqCst);

                let mut waker_lock = atomic_waker_clone.lock().unwrap();
                if let Some(waker) = waker_lock.take() {
                    println!("Waking!");
                    waker.wake();
                } else {
                    println!("No waker :(");
                }
            }),
        }
    }
}

async fn work() -> i32 {
    use work::*;
    let call = work_Call::<'_> {
        __rs_generics: ::core::marker::PhantomData,
    };
    let fn_data: &::rsubstitute::for_generated::FnData<'_, workMock<'_>, true, true, false> =
        ::rsubstitute::for_generated::get_static_fn_data("work");
    fn_data.handle_async((), call, __rs_base_work).await
}
#[allow(unused)]
#[allow(unreachable_pub)]
#[allow(nonstandard_style)]
mod work {
    use super::*;
    #[doc(hidden)]
    pub async fn __rs_base_work(_: (), call: work_Call<'_>) -> i32 {
        let work_Call::<'_> { .. } = call;
        tokio::time::sleep(Duration::from_secs(1));
        return 12;
    }
    pub fn setup<'__rsa>() -> ::rsubstitute::for_generated::FnConfigurator<
        '__rsa,
        workMock<'__rsa>,
        workStaticSetup<'__rsa>,
        (),
        i32,
        workMock<'__rsa>,
        true,
        true,
        false,
    > {
        ::rsubstitute::for_generated::clear_static_fn_data::<workMock<'__rsa>>();
        workStaticSetup::<'__rsa> {
            __rs_generics: ::core::marker::PhantomData,
        }
        .setup()
    }
    pub fn received<'__rsa>(
        times: ::rsubstitute::for_generated::Times,
    ) -> ::rsubstitute::for_generated::ArgRefsBinder<workStaticReceived<'__rsa>, ()> {
        workStaticReceived::<'__rsa> {
            __rs_generics: ::core::marker::PhantomData,
        }
        .received(times)
    }
    pub struct work_Call<'__rsa> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__rsa (),)>,
    }
    impl<'__rsa> ::rsubstitute::for_generated::IGenericsInfoProvider for work_Call<'__rsa> {}
    impl<'__rsa> ::rsubstitute::for_generated::ICall for work_Call<'__rsa> {}
    impl<'__rsa> ::core::clone::Clone for work_Call<'__rsa> {
        #[inline]
        fn clone(&self) -> work_Call<'__rsa> {
            work_Call::<'__rsa> {
                __rs_generics: ::core::clone::Clone::clone(&self.__rs_generics),
            }
        }
    }
    struct work_ArgsChecker<'__rsa> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__rsa (),)>,
    }
    impl<'__rsa> ::rsubstitute::for_generated::IGenericsInfoProvider for work_ArgsChecker<'__rsa> {}
    impl<'__rsa> ::rsubstitute::for_generated::IArgsChecker for work_ArgsChecker<'__rsa> {}
    pub struct workMock<'__rsa> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__rsa (),)>,
    }
    pub struct workStaticSetup<'__rsa> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__rsa (),)>,
    }
    impl<'__rsa> workStaticSetup<'__rsa> {
        pub fn setup(
            &self,
        ) -> ::rsubstitute::for_generated::FnConfigurator<
            '_,
            workMock<'__rsa>,
            Self,
            (),
            i32,
            workMock<'__rsa>,
            true,
            true,
            false,
        > {
            let args_checker = work_ArgsChecker::<'__rsa> {
                __rs_generics: ::core::marker::PhantomData,
            };
            let fn_data: &::rsubstitute::for_generated::FnData<
                '_,
                workMock<'__rsa>,
                true,
                true,
                false,
            > = ::rsubstitute::for_generated::get_static_fn_data("work");
            let fn_configurator: ::rsubstitute::for_generated::FnConfigurator<
                '_,
                workMock<'__rsa>,
                Self,
                (),
                i32,
                workMock<'__rsa>,
                true,
                true,
                false,
            > = fn_data.add_config(args_checker, self);
            ::rsubstitute::transmute_lifetime!(fn_configurator)
        }
    }
    pub struct workStaticReceived<'__rsa> {
        pub __rs_generics: ::core::marker::PhantomData<(&'__rsa (),)>,
    }
    impl<'__rsa> ::core::clone::Clone for workStaticReceived<'__rsa> {
        #[inline]
        fn clone(&self) -> workStaticReceived<'__rsa> {
            workStaticReceived::<'__rsa> {
                __rs_generics: ::core::clone::Clone::clone(&self.__rs_generics),
            }
        }
    }
    impl<'__rsa> workStaticReceived<'__rsa> {
        pub fn received(
            &self,
            times: ::rsubstitute::for_generated::Times,
        ) -> ::rsubstitute::for_generated::ArgRefsBinder<Self, ()> {
            let args_checker = work_ArgsChecker::<'__rsa> {
                __rs_generics: ::core::marker::PhantomData,
            };
            let fn_data: &::rsubstitute::for_generated::FnData<
                '_,
                workMock<'_>,
                true,
                true,
                false,
            > = ::rsubstitute::for_generated::get_static_fn_data("work");
            fn_data.verify_received(args_checker, times);
            rsubstitute::for_generated::ArgRefsBinder::new(self.clone())
        }
        pub fn no_other_calls(&self) {
            ::rsubstitute::for_generated::verify_static_fn_received_nothing_else::<workMock<'_>>()
        }
    }
}

#[mock(base)]
impl Future for MyFuture {
    type Output = i32;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<<Self as Future>::Output> {
        println!("Polling!");
        if self.thread.is_finished() {
            Poll::Ready(self.atomic_result.load(Ordering::SeqCst))
        } else {
            *self.atomic_waker.lock().unwrap() = Some(cx.waker().clone());
            Poll::Pending
        }
    }
}

#[tokio::main]
async fn main() {
    let mut future = MyFuture::new(166);
    future.setup().as_Future().poll(Arg::Any).call_base();
    let result = future.await;
    assert_eq!(166, result);

    work::setup().returns(515);
    let work_result = work().await;
    assert_eq!(515, work_result);
    work::received(Times::Once).no_other_calls();
    dbg!(work_result);

    work::setup().call_base();
    let work_result = work().await;
    assert_eq!(12, work_result);
    work::received(Times::Once).no_other_calls();
    dbg!(work_result);
}
