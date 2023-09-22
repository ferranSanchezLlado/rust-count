#[macro_export]
macro_rules! count {
    () => (0_usize);
    ($el: tt $(,)?) => (1_usize);
    ($el: tt, $($other: tt),* $(,)?) => {
        1_usize + count!($($other),*)
    };
}

#[macro_export]
macro_rules! my_vec {
    () => (Vec::new());
    ($($el: expr),+ $(,)?) => {{
        let mut vec = Vec::with_capacity(count!($($el),*));
        $( vec.push($el); )*
        vec
    }};
    ($el: expr; $n: expr) => {{
        let mut vec = Vec::with_capacity($n);
        for _ in 0..$n {
            vec.push($el.clone());
        }
        vec
    }};
    ($el: literal: $n: expr) => {{
        let mut vec = Vec::with_capacity($n);
        for _ in 0..$n {
            vec.push($el);
        }
        vec
    }};
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_empty() {
        let count = count!();
        assert_eq!(count, 0);
    }

    #[test]
    fn count_one() {
        let count = count!(10);
        assert_eq!(count, 1);

        let count = count!(20,);
        assert_eq!(count, 1);
    }

    #[test]
    fn count_multiple() {
        let count = count!(10, 100, 21, 3, "a");
        assert_eq!(count, 5);

        let count = count!(10, 21, 3, "a",);
        assert_eq!(count, 4);
    }

    #[test]
    fn vec_empty() {
        let vec: Vec<()> = my_vec![];

        assert!(vec.is_empty());
    }

    #[test]
    fn vec_list() {
        let vec = my_vec!["a", "v", "ddd"];

        assert_eq!(vec.len(), 3);
        assert_eq!(vec.capacity(), 3);

        let vec = my_vec![1, 2, 3, 4, 5,];

        assert_eq!(vec.len(), 5);
        assert_eq!(vec.capacity(), 5);
    }

    #[test]
    fn vec_clone() {
        let vec = my_vec!["a"; 10];

        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);

        let vec = my_vec![1; 20];

        assert_eq!(vec.len(), 20);
        assert_eq!(vec.capacity(), 20);
    }

    #[test]
    fn vec_literal() {
        let vec = my_vec!["a": 10];

        assert_eq!(vec.len(), 10);
        assert_eq!(vec.capacity(), 10);

        let vec = my_vec![1: 20];

        assert_eq!(vec.len(), 20);
        assert_eq!(vec.capacity(), 20);
    }
}
