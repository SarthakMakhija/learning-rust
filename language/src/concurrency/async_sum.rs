use tokio::task::JoinHandle;
use crate::concurrency::sum_of;

const CONCURRENCY_FACTOR: usize = 10;

async fn sum(input: Vec<u64>) -> u64 {
    let task_handles = concurrent_sum(input);
    let mut total: u64 = 0;
    let join_handles: (Vec<JoinHandle<u64>>, ) = tokio::join!(task_handles);
    for handle in join_handles.0 {
        total = total + handle.await.unwrap()
    }
    return total;
}

async fn concurrent_sum(input: Vec<u64>) -> Vec<JoinHandle<u64>> {
    let chunks = divide(&input, CONCURRENCY_FACTOR).clone();
    let mut task_handles = Vec::with_capacity(CONCURRENCY_FACTOR);

    for chunk in chunks {
        task_handles.push(
            tokio::spawn(async move {
                return sum_of(chunk);
            })
        );
    }
    return task_handles;
}

fn divide(input: &Vec<u64>, chunk_size: usize) -> Vec<Vec<u64>> {
    return input.chunks(chunk_size).map(|s| s.into()).collect();
}

#[tokio::test]
async fn test_async_sum() {
    let elements = prepare(500000);
    let result = sum(elements);
    assert_eq!(125_000_250_000, result.await);
}

fn prepare(count: u32) -> Vec<u64> {
    let mut elements: Vec<u64> = Vec::with_capacity(500000);
    (1..count + 1).
        for_each(|e| {
            elements.push(u64::from(e));
        });
    return elements;
}