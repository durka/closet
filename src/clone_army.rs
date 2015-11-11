#[macro_export]
macro_rules! clone_army {
    (@as_expr $e:expr) => { $e };

    ([$($var:ident),*] $cl:expr) => {{
        $(let $var = $var.clone();)*
            $cl
    }};
}

