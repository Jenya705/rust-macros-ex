#[macro_export]
macro_rules! min {
    ($a: expr, $b: expr) => {
        if $a < $b {
            $a
        }
        else {
            $b
        }
    };
    ($a: expr, $b: expr, $($c: tt)*) => {
        if $a < $b {
            min!($a, $($c)*)
        }
        else {
            min!($b, $($c)*)
        }
    };
}

#[macro_export]
macro_rules! max {
    ($a: expr, $b: expr) => {
        if $a > $b {
            $a
        }
        else {
            $b
        }
    };
    ($a: expr, $b: expr, $($c: tt)*) => {
        if $a > $b {
            max!($a, $($c)*)
        }
        else {
            max!($b, $($c)*)
        }
    };
}