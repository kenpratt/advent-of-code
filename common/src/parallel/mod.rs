use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_find<I, T, R>(iter: I, do_work: fn(T) -> Option<R>) -> Option<R>
where
    I: Iterator<Item = T> + Send,
    R: Send + Debug,
{
    let num_workers = thread::available_parallelism().unwrap().get();
    let work_queue = Arc::new(Mutex::new(iter));
    let result: Arc<Mutex<Option<R>>> = Arc::new(Mutex::new(None));

    thread::scope(|s| {
        for _ in 0..num_workers {
            let this_work_queue = Arc::clone(&work_queue);
            let this_result = Arc::clone(&result);
            s.spawn(move || work(this_work_queue, this_result, do_work));
        }
    });

    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

fn work<I, T, R>(
    work_queue: Arc<Mutex<I>>,
    global_result: Arc<Mutex<Option<R>>>,
    do_work: fn(T) -> Option<R>,
) where
    I: Iterator<Item = T>,
{
    while global_result.lock().unwrap().is_none() {
        let maybe_job = work_queue.lock().unwrap().next();
        match maybe_job {
            Some(job) => {
                let result = do_work(job);
                if result.is_some() {
                    *global_result.lock().unwrap() = result;
                }
            }
            None => return,
        }
    }
}
