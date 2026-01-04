//! Advanced concurrency utilities using crossbeam.
//!
//! Provides work-stealing, scoped threads, and channel-based progress reporting.

use rustmax::prelude::*;
use rustmax::crossbeam::channel::{Receiver, Sender, bounded, unbounded};
use rustmax::crossbeam::deque::{Injector, Stealer, Worker};
use rustmax::crossbeam::scope;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Progress event sent during builds.
#[derive(Debug, Clone)]
pub enum ProgressEvent {
    /// A task has started.
    Started { name: String },
    /// A task has completed.
    Completed { name: String, duration_ms: u64 },
    /// A task failed.
    Failed { name: String, error: String },
    /// Overall progress update.
    Progress { completed: usize, total: usize },
    /// Build finished.
    Finished { total_duration_ms: u64 },
}

/// Progress reporter that sends events over a channel.
#[derive(Clone)]
pub struct ProgressReporter {
    sender: Sender<ProgressEvent>,
    completed: Arc<AtomicUsize>,
    total: Arc<AtomicUsize>,
}

impl ProgressReporter {
    /// Create a new progress reporter and receiver.
    pub fn new() -> (Self, Receiver<ProgressEvent>) {
        let (sender, receiver) = unbounded();
        (
            Self {
                sender,
                completed: Arc::new(AtomicUsize::new(0)),
                total: Arc::new(AtomicUsize::new(0)),
            },
            receiver,
        )
    }

    /// Set the total number of tasks.
    pub fn set_total(&self, total: usize) {
        self.total.store(total, Ordering::SeqCst);
    }

    /// Report that a task started.
    pub fn start(&self, name: impl Into<String>) {
        let _ = self.sender.send(ProgressEvent::Started { name: name.into() });
    }

    /// Report that a task completed.
    pub fn complete(&self, name: impl Into<String>, duration_ms: u64) {
        let completed = self.completed.fetch_add(1, Ordering::SeqCst) + 1;
        let total = self.total.load(Ordering::SeqCst);
        let _ = self.sender.send(ProgressEvent::Completed {
            name: name.into(),
            duration_ms,
        });
        let _ = self.sender.send(ProgressEvent::Progress { completed, total });
    }

    /// Report that a task failed.
    pub fn fail(&self, name: impl Into<String>, error: impl Into<String>) {
        let _ = self.sender.send(ProgressEvent::Failed {
            name: name.into(),
            error: error.into(),
        });
    }

    /// Report that all tasks are finished.
    pub fn finish(&self, total_duration_ms: u64) {
        let _ = self.sender.send(ProgressEvent::Finished { total_duration_ms });
    }
}

/// Work-stealing task pool for dynamic load balancing.
pub struct TaskPool<T> {
    injector: Injector<T>,
    stealers: Vec<Stealer<T>>,
    workers: Vec<Worker<T>>,
}

impl<T> TaskPool<T> {
    /// Create a new task pool with the specified number of workers.
    pub fn new(num_workers: usize) -> Self {
        let mut workers = Vec::with_capacity(num_workers);
        let mut stealers = Vec::with_capacity(num_workers);

        for _ in 0..num_workers {
            let worker = Worker::new_fifo();
            stealers.push(worker.stealer());
            workers.push(worker);
        }

        Self {
            injector: Injector::new(),
            stealers,
            workers,
        }
    }

    /// Push a task to the global queue.
    pub fn push(&self, task: T) {
        self.injector.push(task);
    }

    /// Try to steal a task for a worker.
    pub fn steal(&self, worker_id: usize) -> Option<T> {
        // First try the local worker queue.
        if let Some(task) = self.workers[worker_id].pop() {
            return Some(task);
        }

        // Then try the global queue.
        loop {
            match self.injector.steal_batch_and_pop(&self.workers[worker_id]) {
                rustmax::crossbeam::deque::Steal::Success(task) => return Some(task),
                rustmax::crossbeam::deque::Steal::Empty => break,
                rustmax::crossbeam::deque::Steal::Retry => continue,
            }
        }

        // Finally try stealing from other workers.
        for (i, stealer) in self.stealers.iter().enumerate() {
            if i == worker_id {
                continue;
            }
            loop {
                match stealer.steal() {
                    rustmax::crossbeam::deque::Steal::Success(task) => return Some(task),
                    rustmax::crossbeam::deque::Steal::Empty => break,
                    rustmax::crossbeam::deque::Steal::Retry => continue,
                }
            }
        }

        None
    }

    /// Check if the pool is empty.
    pub fn is_empty(&self) -> bool {
        self.injector.is_empty() && self.workers.iter().all(|w| w.is_empty())
    }
}

/// Run tasks in parallel using scoped threads.
pub fn parallel_for<T, F>(items: &[T], f: F)
where
    T: Sync,
    F: Fn(&T) + Sync,
{
    let num_threads = thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4);

    scope(|s| {
        let chunk_size = (items.len() + num_threads - 1) / num_threads;
        let f = &f;

        for chunk in items.chunks(chunk_size) {
            s.spawn(move |_| {
                for item in chunk {
                    f(item);
                }
            });
        }
    })
    .expect("scoped threads panicked");
}

/// Run tasks in parallel and collect results.
pub fn parallel_map<T, R, F>(items: &[T], f: F) -> Vec<R>
where
    T: Sync,
    R: Send,
    F: Fn(&T) -> R + Sync,
{
    let num_threads = thread::available_parallelism()
        .map(|p| p.get())
        .unwrap_or(4);

    let results = std::sync::Mutex::new(Vec::with_capacity(items.len()));

    scope(|s| {
        let chunk_size = (items.len() + num_threads - 1) / num_threads;
        let f = &f;
        let results = &results;

        for (chunk_idx, chunk) in items.chunks(chunk_size).enumerate() {
            s.spawn(move |_| {
                let chunk_results: Vec<(usize, R)> = chunk
                    .iter()
                    .enumerate()
                    .map(|(i, item)| (chunk_idx * chunk_size + i, f(item)))
                    .collect();

                let mut guard = results.lock().unwrap();
                guard.extend(chunk_results);
            });
        }
    })
    .expect("scoped threads panicked");

    let mut results = results.into_inner().expect("mutex should not be poisoned");
    results.sort_by_key(|(i, _)| *i);
    results.into_iter().map(|(_, r)| r).collect()
}

/// Create a bounded channel for backpressure.
pub fn bounded_channel<T>(capacity: usize) -> (Sender<T>, Receiver<T>) {
    bounded(capacity)
}

/// Create an unbounded channel.
pub fn unbounded_channel<T>() -> (Sender<T>, Receiver<T>) {
    unbounded()
}

/// Receive with timeout.
pub fn recv_timeout<T>(receiver: &Receiver<T>, timeout: Duration) -> Option<T> {
    receiver.recv_timeout(timeout).ok()
}

/// Try to receive without blocking.
pub fn try_recv<T>(receiver: &Receiver<T>) -> Option<T> {
    receiver.try_recv().ok()
}

/// Fan-out pattern: distribute work to multiple workers.
pub fn fan_out<T, R, F>(items: Vec<T>, num_workers: usize, f: F) -> Vec<R>
where
    T: Send,
    R: Send,
    F: Fn(T) -> R + Send + Clone,
{
    let (work_tx, work_rx) = bounded::<T>(num_workers * 2);
    let (result_tx, result_rx) = unbounded::<(usize, R)>();

    scope(|s| {
        // Spawn workers.
        for _ in 0..num_workers {
            let work_rx = work_rx.clone();
            let result_tx = result_tx.clone();
            let f = f.clone();
            s.spawn(move |_| {
                while let Ok(item) = work_rx.recv() {
                    // Work items don't track index in this pattern.
                    let result = f(item);
                    let _ = result_tx.send((0, result));
                }
            });
        }

        // Drop our copies so workers will exit.
        drop(work_rx);
        drop(result_tx);

        // Send work items.
        for item in items {
            let _ = work_tx.send(item);
        }
        drop(work_tx);
    })
    .expect("scoped threads panicked");

    result_rx.iter().map(|(_, r)| r).collect()
}

/// Pipeline pattern: chain of processing stages.
pub struct Pipeline<T> {
    receiver: Receiver<T>,
}

impl<T: Send + 'static> Pipeline<T> {
    /// Create a new pipeline from an iterator.
    pub fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T> + Send + 'static,
    {
        let (tx, rx) = unbounded();
        thread::spawn(move || {
            for item in iter {
                if tx.send(item).is_err() {
                    break;
                }
            }
        });
        Self { receiver: rx }
    }

    /// Add a transformation stage.
    pub fn map<R, F>(self, f: F) -> Pipeline<R>
    where
        R: Send + 'static,
        F: Fn(T) -> R + Send + 'static,
    {
        let (tx, rx) = unbounded();
        thread::spawn(move || {
            for item in self.receiver {
                if tx.send(f(item)).is_err() {
                    break;
                }
            }
        });
        Pipeline { receiver: rx }
    }

    /// Add a filtering stage.
    pub fn filter<F>(self, predicate: F) -> Pipeline<T>
    where
        F: Fn(&T) -> bool + Send + 'static,
    {
        let (tx, rx) = unbounded();
        thread::spawn(move || {
            for item in self.receiver {
                if predicate(&item) {
                    if tx.send(item).is_err() {
                        break;
                    }
                }
            }
        });
        Pipeline { receiver: rx }
    }

    /// Collect results.
    pub fn collect(self) -> Vec<T> {
        self.receiver.iter().collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_reporter() {
        let (reporter, receiver) = ProgressReporter::new();
        reporter.set_total(3);

        reporter.start("task1");
        reporter.complete("task1", 100);
        reporter.start("task2");
        reporter.fail("task2", "oops");
        reporter.finish(200);

        let events: Vec<_> = receiver.try_iter().collect();
        assert!(events.len() >= 4);
    }

    #[test]
    fn test_task_pool() {
        let pool: TaskPool<i32> = TaskPool::new(4);

        for i in 0..100 {
            pool.push(i);
        }

        let mut collected = Vec::new();
        for worker_id in 0..4 {
            while let Some(task) = pool.steal(worker_id) {
                collected.push(task);
            }
        }

        collected.sort();
        assert_eq!(collected, (0..100).collect::<Vec<_>>());
    }

    #[test]
    fn test_parallel_for() {
        let counter = Arc::new(AtomicUsize::new(0));
        let items: Vec<i32> = (0..100).collect();

        let counter_clone = Arc::clone(&counter);
        parallel_for(&items, move |_| {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(counter.load(Ordering::SeqCst), 100);
    }

    #[test]
    fn test_parallel_map() {
        let items: Vec<i32> = (0..100).collect();
        let results = parallel_map(&items, |x| x * 2);

        assert_eq!(results.len(), 100);
        for (i, r) in results.iter().enumerate() {
            assert_eq!(*r, (i as i32) * 2);
        }
    }

    #[test]
    fn test_bounded_channel() {
        let (tx, rx) = bounded_channel::<i32>(2);

        tx.send(1).unwrap();
        tx.send(2).unwrap();

        assert_eq!(rx.recv().unwrap(), 1);
        assert_eq!(rx.recv().unwrap(), 2);
    }

    #[test]
    fn test_fan_out() {
        let items: Vec<i32> = (0..10).collect();
        let results = fan_out(items.clone(), 4, |x| x * 2);

        assert_eq!(results.len(), 10);
        let mut sorted: Vec<i32> = results;
        sorted.sort();
        assert_eq!(sorted, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
    }

    #[test]
    fn test_pipeline() {
        let result: Vec<i32> = Pipeline::from_iter(1..=10)
            .filter(|x| x % 2 == 0)
            .map(|x| x * 3)
            .collect();

        let mut sorted = result;
        sorted.sort();
        assert_eq!(sorted, vec![6, 12, 18, 24, 30]);
    }

    #[test]
    fn test_recv_timeout() {
        let (tx, rx) = bounded_channel::<i32>(1);
        tx.send(42).unwrap();

        let result = recv_timeout(&rx, Duration::from_millis(100));
        assert_eq!(result, Some(42));

        let result = recv_timeout(&rx, Duration::from_millis(10));
        assert_eq!(result, None);
    }

    #[test]
    fn test_try_recv() {
        let (tx, rx) = bounded_channel::<i32>(1);

        assert_eq!(try_recv(&rx), None);
        tx.send(42).unwrap();
        assert_eq!(try_recv(&rx), Some(42));
    }
}
