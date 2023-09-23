pub const fn __count_helper<const N: usize>(_: [(); N]) -> usize {
    N
}

#[macro_export]
macro_rules! replace {
    ($_t:tt $sub:expr) => {
        $sub
    };
}

#[macro_export]
macro_rules! count {
    ($($el: tt),* $(,)?) => {{
        use ::count::__count_helper;
        __count_helper([$(replace!($el ())),*])
    }};
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
        let count = $n;
        let mut vec = Vec::with_capacity(count);
        vec.resize(count, $el);
        vec
    }};
    ($n: expr => $el: expr) => {{
        let count = $n;
        let mut vec = Vec::with_capacity(count);
        vec.resize_with(count, || $el);
        vec
    }};
}
