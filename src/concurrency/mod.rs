pub mod sequential_sum;
mod async_sum;
mod concurrent_sum;

pub fn sum_of(chunk: Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    for value in chunk {
        sum = sum + value;
    }
    return sum;
}