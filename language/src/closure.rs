use std::collections::HashMap;

#[derive(Eq, PartialEq, Hash)]
enum OperationId {
    Addition,
    Multiplication,
}

type ArithmeticFunction = Box<dyn Fn(u32, u32) -> u32>;

struct ArithmeticOperations {
    operation_by_id: HashMap<OperationId, ArithmeticFunction>,
}

impl ArithmeticOperations {
    fn new() -> ArithmeticOperations {
        return ArithmeticOperations {
            operation_by_id: HashMap::new()
        };
    }

    fn add<C>(&mut self, operation: OperationId, code: C)
        where C: Fn(u32, u32) -> u32 + 'static {
        self.operation_by_id.insert(operation, Box::new(code));
    }

    fn execute(&self, operation_by_id: OperationId, a: u32, b: u32) -> Option<u32> {
        let optional_arithmetic_fn = self.operation_by_id.get(&operation_by_id);
        return match optional_arithmetic_fn {
            None => None,
            Some(func) => Some(func(a, b))
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_execute_addition() {
        let mut operation = ArithmeticOperations::new();
        operation.add(OperationId::Addition, |a, b| {
            a + b
        });
        let result = operation.execute(OperationId::Addition, 4, 90);
        assert_eq!(94, result.unwrap());
    }

    #[test]
    fn test_execute_multiplication() {
        let mut operation = ArithmeticOperations::new();
        operation.add(OperationId::Multiplication, |a, b| {
            a * b
        });
        let result = operation.execute(OperationId::Multiplication, 10, 90);
        assert_eq!(900, result.unwrap());
    }
}

