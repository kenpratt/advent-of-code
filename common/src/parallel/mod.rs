use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_find<I, T, R>(iter: I, batch_size: usize, do_work: fn(&T) -> Option<R>) -> Option<R>
where
    I: Iterator<Item = T> + Send,
    R: Send + Debug,
{
    let num_workers = thread::available_parallelism().unwrap().get();
    let work_queue = Arc::new(Mutex::new(iter));
    let complete = Arc::new(AtomicBool::new(false));
    let result = Arc::new(Mutex::new(None));

    thread::scope(|s| {
        for _ in 0..num_workers {
            let this_work_queue = Arc::clone(&work_queue);
            let this_complete = Arc::clone(&complete);
            let this_result = Arc::clone(&result);
            s.spawn(move || {
                work(
                    this_work_queue,
                    this_complete,
                    this_result,
                    batch_size,
                    do_work,
                )
            });
        }
    });

    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

fn work<I, T, R>(
    work_queue: Arc<Mutex<I>>,
    complete: Arc<AtomicBool>,
    global_result: Arc<Mutex<Option<R>>>,
    batch_size: usize,
    do_work: fn(&T) -> Option<R>,
) where
    I: Iterator<Item = T>,
{
    let mut jobs: Vec<Option<T>> = std::iter::repeat_with(|| None).take(batch_size).collect();

    while !complete.load(Ordering::Relaxed) {
        let num_jobs = take_jobs(&work_queue, &mut jobs);
        if num_jobs == 0 {
            return;
        }

        for job_index in 0..num_jobs {
            let job = jobs[job_index].as_ref().unwrap();
            if complete.load(Ordering::Relaxed) {
                return;
            }

            let result = do_work(job);
            if result.is_some() {
                complete.store(true, Ordering::Relaxed);
                *global_result.lock().unwrap() = result;
            }
        }
    }
}

fn take_jobs<I, T>(work_queue: &Arc<Mutex<I>>, jobs: &mut Vec<Option<T>>) -> usize
where
    I: Iterator<Item = T>,
{
    let mut queue = work_queue.lock().unwrap();
    let mut i = 0;
    while let Some(job) = queue.next() {
        jobs[i] = Some(job);
        i += 1;
        if i == jobs.len() {
            break;
        }
    }
    i
}
