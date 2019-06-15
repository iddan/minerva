#[macro_export]
macro_rules! sum_type {
    ($(#[$($attrs:tt)*])* enum $name:ident, $($x:ident),+) => {
        $(#[$($attrs)*])* enum $name {
            $(
                $x($x),
            )+
        }
        $(
            impl From<$x> for $name {
                fn from(value: $x) -> $name {
                    $name::$x(value)
                }
            }
        )+
    };
    ($(#[$($attrs:tt)*])* pub enum $name:ident, $($x:ident),+) => {
        $(#[$($attrs)*])* pub enum $name {
            $(
                $x($x),
            )+
        }
        $(
            impl From<$x> for $name {
                fn from(value: $x) -> $name {
                    $name::$x(value)
                }
            }
        )+
    };
}
