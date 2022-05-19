use crossbeam::channel::unbounded;
use rayon::ThreadPoolBuilder;

const THREAD_NUMBER: usize = 10;
const THRESHOLD: usize = 10;

struct OrderedItem<I> {
    item: I,
    index: usize,
}

impl<I> OrderedItem<I> {
    fn new(item: I, index: usize) -> Self {
        OrderedItem {
            item,
            index,
        }
    }
}

pub fn smart_execute<T, R>(params: Vec<T>, function: fn(t: T) -> R) -> Vec<R>
    where T: Copy + Sync + Send,
          R: Send + Copy + Sync {
    let size = params.len();
    if size <= THRESHOLD {
        return params.iter().map(|i| {
            function(*i)
        }).collect();
    }

    let (tx, rx) = unbounded();
    let pool = ThreadPoolBuilder::new()
        .num_threads(THREAD_NUMBER)
        .build()
        .unwrap();

    pool.scope(move |s| {
        params.iter().enumerate().for_each(|(i, x)| {
            let sender = tx.clone();
            let param = x.clone();
            s.spawn(move |_| {
                let result = function(param);
                sender.send(OrderedItem::new(result.clone(), i)).unwrap();
            });
        });
    });

    let mut results: Vec<OrderedItem<R>> = vec![];

    for _ in 0..size {
        results.push(rx.recv().unwrap());
    }

    results.sort_by(|i, j| {
        i.index.cmp(&j.index)
    });
    return results.iter().map(|i| {
        i.item
    }).collect();
}

#[cfg(test)]
mod tests {
    use std::thread;
    use std::time::{Duration, SystemTime};
    use crate::executor::{smart_execute};

    #[test]
    fn return_ok_with_one_item() {
        let params = vec![1];
        let inc_function = |i| {
            i + 1
        };
        let result = smart_execute(params, inc_function);
        assert_eq!(1, result.len());
        let first_item = result.first().expect("Can't fetch first item");
        assert_eq!(2, *first_item)
    }

    #[test]
    fn return_ok_with_multiple_items() {
        let params: Vec<i32> = (0..100).collect();
        let inc_function = |i| {
            i + 1
        };
        let result = smart_execute(params.clone(), inc_function);
        assert_eq!(params.len(), result.len());
        let expected: Vec<i32> = params.iter()
            .map(|i| inc_function(*i))
            .collect();
        assert_eq!(expected, result)
    }

    #[test]
    fn return_result_faster_than_single_thread() {
        let params: Vec<i32> = (0..40).collect();
        let inc_function = |i| {
            thread::sleep(Duration::from_millis(200));
            i + 1
        };
        let duration = get_duration(|| { smart_execute(params.clone(), inc_function) });
        assert!(duration.lt(&Duration::from_millis(8000)))
    }

    fn get_duration<F: Fn() -> T, T>(f: F) -> Duration {
        let start = SystemTime::now();
        let _result = f();
        let end = SystemTime::now();
        return end.duration_since(start).unwrap();
    }
}
