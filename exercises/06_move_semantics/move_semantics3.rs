// TODO: Fix the compiler error in the function without adding any new line.
fn fill_vec(mut nvec: Vec<i32>) -> Vec<i32> {
    nvec.push(88);

    nvec
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics3() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, [22, 44, 66, 88]);
    }
}
