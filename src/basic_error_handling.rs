use std::num::ParseIntError;

fn parse(inputs: Vec<&str>) -> Result<Vec<u64>, ParseIntError> {
    let mut result = Vec::with_capacity(inputs.capacity());
    for input in &inputs {
        let parsed: u64 = input.parse()?;
        result.push(parsed)
    }
    return Ok(result);
}

#[test]
fn test_successful_parse() {
    let inputs = vec!["231", "123", "34534", "909"];
    let result = parse(inputs);
    let parsed = result.unwrap();

    let expected: Vec<u64> = vec![231, 123, 34534, 909];
    assert_eq!(expected, parsed);
}

#[test]
fn test_failed_parse() {
    let inputs = vec!["231", "123", "hello", "909"];
    let result = parse(inputs);
    let is_err = result.is_err();

    assert_eq!(is_err, true);
}