use futures::{
        future::{BoxFuture, FutureExt},
        task::{waker_ref, ArcWake},
};

use std::{
        cell::RefCell, 
        future::Future, 
        sync::{Arc, Mutex}, 
        task::Context,
};

// region:      -- Task<T>

#[allow(dead_code)]
struct Task<T> {
        future: Mutex<Option<BoxFuture<'static, T>>>,
}

#[allow(dead_code)]
impl<T> Task<T> {
        fn new(future: impl Future<Output = T> + 'static + Send) -> Self {
                let future = future.boxed();
                Self {
                        future: Mutex::new(Some(future)),
                }
        }
}

impl<T> ArcWake for Task<T> {
        fn wake_by_ref(_arc_self: &Arc<Self>) {
                // do nothing
        }
}

// endregion:   -- Task<T>


// region:      -- SinleExecutor

#[allow(dead_code)]
pub trait SingleExecutor {
        type Output;

        fn spawn(future: impl Future<Output = Self::Output> + 'static + Send) -> Self;
        fn run(&self) -> Option<Self::Output>;
}

#[allow(dead_code)]
pub struct SingleExecutorType<T> {
        task: RefCell<Arc<Task<T>>>,
}

impl<T> SingleExecutor for SingleExecutorType<T>
where T: 'static 
{
        type Output = T;
        
        // executor visit task's {Option<BoxFuture<>>}, then take BoxFuture out from option and poll that
        // if pending => put future back to option
        // if ready => return Ready(Output)

        /// create an Executor
        fn spawn(future: impl Future<Output = Self::Output> + 'static + Send) -> Self {
                let future = future.boxed();
                let task = RefCell::new(Arc::new(Task::<Self::Output>::new(future)));
                
                Self { task }
        }
        
        /// run and get result
        fn run(&self) -> Option<Self::Output> {

                let binding = self.task.borrow_mut();
                let mut future_slot = binding.future.lock().unwrap();

                // loop while future is not none
                //      => this task is not finished
                while let Some(mut future) = future_slot.take() {
                        let waker = waker_ref(&binding);
                        let context = &mut Context::from_waker(&waker);

                        match future.as_mut().poll(context) {
                            std::task::Poll::Ready(o) => return Some(o),
                            std::task::Poll::Pending => *future_slot = Some(future),
                        }

                }

                None
        }
        
}

// endregion:      -- SinleExecutor