use std::fmt::Debug;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

pub fn parallel_find<I, T, R, F>(iter: I, batch_size: usize, work_fn: F) -> Option<R>
where
    I: Iterator<Item = T> + Send,
    R: Send + Debug,
    F: Fn(T) -> Option<R> + Send + Sync,
{
    let num_workers = thread::available_parallelism().unwrap().get();
    let work_queue = Arc::new(Mutex::new(iter));
    let complete = Arc::new(AtomicBool::new(false));
    let result = Arc::new(Mutex::new(None));
    let do_work = Arc::new(work_fn);

    thread::scope(|s| {
        for _ in 0..num_workers {
            s.spawn(|| {
                work(
                    Arc::clone(&work_queue),
                    Arc::clone(&complete),
                    Arc::clone(&result),
                    batch_size,
                    Arc::clone(&do_work),
                )
            });
        }
    });

    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}

fn work<I, T, R, F>(
    work_queue: Arc<Mutex<I>>,
    complete: Arc<AtomicBool>,
    global_result: Arc<Mutex<Option<R>>>,
    batch_size: usize,
    do_work: Arc<F>,
) where
    I: Iterator<Item = T>,
    F: Fn(T) -> Option<R> + Send + Sync,
{
    let mut jobs: Vec<Option<T>> = std::iter::repeat_with(|| None).take(batch_size).collect();

    while !complete.load(Ordering::Relaxed) {
        let num_jobs = take_jobs(&work_queue, &mut jobs);
        if num_jobs == 0 {
            return;
        }

        for job_index in 0..num_jobs {
            let job = jobs[job_index].take().unwrap();
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
