mod do_not_edit;

/// [★] 二つの整数の和を計算せよ
pub fn add(a: i32, b: i32) -> i32 {
    todo!()
}

/// [★★] 配列の和を計算せよ
pub fn sum(nums: &[i32]) -> i32 {
    todo!()
}

/// [★★] 配列の最大値を計算せよ
/// ただし渡された配列は空ではないとする
pub fn max(nums: &[i32]) -> i32 {
    todo!()
}

/// [★★] 配列を文字列に変換せよ
/// 例えば [1, 2, 3] ならば "1, 2, 3" を返す
pub fn nums_to_string(nums: &[i32]) -> String {
    todo!()
}

/// [★★] 配列を文字列の配列に変換せよ
/// 例えば [1, 2, 3] ならば ["1", "2", "3"] を返す
pub fn nums_to_vec_string(nums: &[i32]) -> Vec<String> {
    todo!()
}

/// [★] 整数を文字列に変換せよ
pub fn optional_to_string(opt: Option<i32>) -> Option<String> {
    todo!()
}

/// [★] str を String に変換せよ
pub fn str_to_string(s: &str) -> String {
    todo!()
}

/// [★] 二つの整数の和を計算せよ
/// ただし一個でも None が渡された場合は None を返す
pub fn optional_add(a: Option<i32>, b: Option<i32>) -> Option<i32> {
    todo!()
}

/// [★★★] 重複要素のない配列を生成せよ
pub fn unique(nums: &[i32]) -> Vec<i32> {
    todo!()
}

/// [★★] 配列の各要素の二乗の和を計算せよ
pub fn square_sum(nums: &[i32]) -> i32 {
    todo!()
}

/// [★★★★★] slow_fn への呼び出しを並列化し、この関数を高速化せよ
/// ヒント：futures crate に役立つ関数がある
pub async fn slow_function() -> i32 {
    let mut sum = 0;
    for i in 1..=100 {
        sum += do_not_edit::slow_fn(i).await;
    }
    sum
}