fn sum_of(chunk: Vec<u64>) -> u64 {
    let mut sum: u64 = 0;
    for value in chunk {
        sum = sum + value;
    }
    return sum;
}

#[test]
fn test_sequential_sum() {
    let elements = prepare(500000);
    let result = sum_of(elements);
    assert_eq!(125_000_250_000, result);
}

fn prepare(count: u32) -> Vec<u64> {
    let mut elements: Vec<u64> = Vec::with_capacity(500000);
    (1..count+1).
        for_each(|e| {
            elements.push(u64::from(e));
        });
    return elements;
}