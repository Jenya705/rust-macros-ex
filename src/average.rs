macro_rules! count_and_sum {
    ($c: expr, $a: expr) => {($c, $a)};
    ($c: expr, $a: expr, $($b: tt)*) => {
        {
            let nextsum = count_and_sum!($c + 1, $($b)*);
            (nextsum.0, nextsum.1 + $a)
        }
    };
}

#[macro_export]
macro_rules! average {
    ($a: expr) => {$a};
    ($a: expr, $($b: tt)*) => {
        {
            let sum = count_and_sum!(1, $a, $($b)*);
            (sum.1 as f64) / (sum.0 as f64)
        }
    }
}

