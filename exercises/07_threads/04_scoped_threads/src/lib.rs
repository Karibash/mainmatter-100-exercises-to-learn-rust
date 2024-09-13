use std::thread;

pub fn sum(v: Vec<i32>) -> i32 {
    let mid = v.len() / 2;
    let (left, right) = v.split_at(mid);

    thread::scope(|scope| {
        let handle_left = scope.spawn(|| left.iter().sum::<i32>());
        let handle_right = scope.spawn(|| right.iter().sum::<i32>());
        handle_left.join().unwrap() + handle_right.join().unwrap()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        assert_eq!(sum(vec![]), 0);
    }

    #[test]
    fn one() {
        assert_eq!(sum(vec![1]), 1);
    }

    #[test]
    fn five() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5]), 15);
    }

    #[test]
    fn nine() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9]), 45);
    }

    #[test]
    fn ten() {
        assert_eq!(sum(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 55);
    }
}
