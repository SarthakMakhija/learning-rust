use std::thread;
use std::thread::JoinHandle;
use crate::concurrency::sum_of;

const CONCURRENCY_FACTOR: usize = 10;

fn sum(input: Vec<u64>) -> u64 {
    let thread_handles = concurrent_sum(input);
    let mut total: u64 = 0;
    for handle in thread_handles {
        total = total + handle.join().unwrap();
    }
    return total;
}

fn concurrent_sum(input: Vec<u64>) -> Vec<JoinHandle<u64>> {
    let chunks = divide(&input, CONCURRENCY_FACTOR).clone();
    let mut thread_handles = Vec::with_capacity(CONCURRENCY_FACTOR);

    for chunk in chunks {
        thread_handles.push(thread::spawn(move || {
            return sum_of(chunk);
        }));
    }
    return thread_handles;
}

fn divide(input: &Vec<u64>, chunk_size: usize) -> Vec<Vec<u64>> {
    return input.chunks(chunk_size).map(|s| s.into()).collect();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concurrent_sum() {
        let elements = prepare(500000);
        let result = sum(elements);
        assert_eq!(125_000_250_000, result);
    }

    fn prepare(count: u32) -> Vec<u64> {
        let mut elements: Vec<u64> = Vec::with_capacity(500000);
        (1..count + 1).
            for_each(|e| {
                elements.push(u64::from(e));
            });
        return elements;
    }
}