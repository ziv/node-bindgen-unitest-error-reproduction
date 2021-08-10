use crate::example::sum;

#[cfg(test)]

#[test]
fn should_sum_numbers() {
    assert_eq!(sum(1, 3), 4);
}
