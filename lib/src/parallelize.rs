use std::{
    any::Any,
    iter,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
        mpsc,
    },
    thread,
    time::{Duration, Instant},
};

pub fn parallelize_with_time_limit<I, T, O>(
    iter: I,
    func: fn(T) -> O,
    limit: Duration,
) -> Box<[Option<O>]>
where
    I: IntoIterator<Item = T>,
    T: Send + Clone + 'static,
    O: Send + 'static,
{
    let (tx, rx) = mpsc::channel::<(usize, O)>();
    let received_counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    let main_thread = thread::current();

    let mut length = 0;
    for (i, input) in iter.into_iter().enumerate() {
        let tx = tx.clone();
        let counter = Arc::clone(&received_counter);
        let main_thread = main_thread.clone();
        let input = input.clone();

        thread::spawn(move || {
            let result = func(input);
            let _ = tx.send((i, result));
            let old_count = counter.fetch_add(1, Ordering::AcqRel);
            if old_count + 1 == length {
                main_thread.unpark();
            }
        });

        length += 1;
    }

    let time_remaining = || limit.saturating_sub(start.elapsed());
    loop {
        let remaining = time_remaining();
        if remaining.is_zero() || received_counter.load(Ordering::Acquire) >= length {
            break;
        }
        thread::park_timeout(remaining);
    }
    let mut results: Box<[Option<O>]> = iter::repeat_with(|| None).take(length).collect();
    for (i, data) in rx.try_iter() {
        results[i] = Some(data);
    }
    results
}

pub fn parallelize<I, T, O>(
    iter: I,
    func: fn(T) -> O,
) -> Box<[Result<O, Box<dyn Any + Send + 'static>>]>
where
    I: IntoIterator<Item = T>,
    T: Send,
    O: Send,
{
    thread::scope(|s| {
        let handles: Vec<_> = iter
            .into_iter()
            .map(|item| s.spawn(|| func(item)))
            .collect();
        handles.into_iter().map(|handle| handle.join()).collect()
    })
}
