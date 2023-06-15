use rstest::rstest;
use std::collections::HashSet;

#[rstest]
#[case(1, 2)]
#[case(25, 75)]
#[case(100, 200)]
fn test_add(#[case] a: i32, #[case] b: i32) {
    assert_eq!(lesson01::add(a, b), a + b);
}

#[rstest]
#[case(&[])]
#[case(&[1])]
#[case(&[1, 2, 3])]
#[case(&[1, 2, 3, 4, 5])]
fn test_sum(#[case] nums: &[i32]) {
    assert_eq!(lesson01::sum(nums), nums.iter().sum());
}

#[rstest]
#[case(&[1])]
#[case(&[4, 1, 3, 2])]
#[case(&[3, 1, 2, 4])]
#[case(&[1, 2, 4, 3])]
fn test_max(#[case] nums: &[i32]) {
    assert_eq!(lesson01::max(nums), *nums.iter().max().unwrap());
}

#[rstest]
#[case(&[])]
#[case(&[1])]
#[case(&[1, 2, 3])]
fn test_nums_to_string(#[case] nums: &[i32]) {
    assert_eq!(
        lesson01::nums_to_string(nums),
        nums.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(", ")
    );
}

#[rstest]
#[case(&[])]
#[case(&[1])]
#[case(&[1, 2, 3])]
fn test_nums_to_vec_string(#[case] nums: &[i32]) {
    assert_eq!(
        lesson01::nums_to_vec_string(nums),
        nums.iter().map(|n| n.to_string()).collect::<Vec<_>>()
    );
}

#[rstest]
#[case(None)]
#[case(Some(1))]
#[case(Some(100))]
fn test_optional_to_string(#[case] opt: Option<i32>) {
    assert_eq!(
        lesson01::optional_to_string(opt),
        opt.map(|n| n.to_string())
    );
}

#[rstest]
#[case(&"")]
#[case(&"Hello, world!")]
fn test_str_to_string(#[case] s: &str) {
    assert_eq!(lesson01::str_to_string(s), s.to_string());
}

#[rstest]
#[case(None, None)]
#[case(Some(1), None)]
#[case(None, Some(1))]
#[case(Some(1), Some(2))]
fn test_optional_add(#[case] a: Option<i32>, #[case] b: Option<i32>) {
    assert_eq!(
        lesson01::optional_add(a, b),
        match (a, b) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        }
    );
}

#[rstest]
#[case(&[])]
#[case(&[1])]
#[case(&[1, 2, 3])]
#[case(&[4, 5, 6, 5, 4])]
fn test_unique(#[case] nums: &[i32]) {
    assert_eq!(
        lesson01::unique(nums).into_iter().collect::<HashSet<_>>(),
        nums.into_iter().cloned().collect::<HashSet<_>>()
    );
}

#[rstest]
#[case(&[])]
#[case(&[1])]
#[case(&[1, 2, 3])]
fn test_square_sum(#[case] nums: &[i32]) {
    assert_eq!(
        lesson01::square_sum(nums),
        nums.iter().map(|n| n * n).sum::<i32>()
    );
}

#[tokio::test]
async fn test_slow_function() {
    let result = tokio::time::timeout(
        std::time::Duration::from_millis(1500),
        lesson01::slow_function(),
    )
    .await;
    assert_eq!(result, Ok(338350));
}
