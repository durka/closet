#[macro_export]
macro_rules! vindaloo {
    (@as_expr $e:expr) => { $e };

    (@emit [$($mov:ident)*] [$(($($param:tt)*))*] [$($ret:tt)*] $($body:tt)*) => {
        vindaloo!(@as_expr $($mov)* $(|$($param)*|)move* $($ret)* $($body)*)
    };

    (| | -> $ret:ty { $($body:tt)* }) => {
        vindaloo!(@emit [] [] [-> $ret] { $($body)* })
    };
    (| | $body:expr) => {
        vindaloo!(@emit [] [] [] $body)
    };
    (|| -> $ret:ty { $($body:tt)* }) => {
        vindaloo!(@emit [] [] [-> $ret] { $($body)* })
    };
    (|| $body:expr) => {
        vindaloo!(@emit [] [] [] $body)
    };
    (|$($param:ident),+| -> $ret:ty { $($body:tt)* }) => {
        vindaloo!(@emit [] [$(($param))+] [-> $ret] { $($body)* })
    };
    (|$($param:ident),+| $body:expr) => {
        vindaloo!(@emit [] [$(($param))+] [] $body)
    };
    (|$($parname:ident: $partyp:ty),+| -> $ret:ty { $($body:tt)* }) => {
        vindaloo!(@emit [] [$(($parname: $partyp))+] [-> $ret] { $($body)* })
    };
    (|$($parname:ident: $partyp:ty),+| $body:expr) => {
        vindaloo!(@emit [] [$(($parname: $partyp))+] [] $body)
    };
    (move | | -> $ret:ty { $($body:tt)* }) => {
        vindaloo!(@emit [] [] [-> $ret] { $($body)* })
    };
    (move | | $body:expr) => {
        vindaloo!(@emit [] [] [] $body)
    };
    (move || -> $ret:ty { $($body:tt)* }) => {
        vindaloo!(@emit [] [] [-> $ret] { $($body)* })
    };
    (move || $body:expr) => {
        vindaloo!(@emit [] [] [] $body)
    };
    (move |$($param:ident),+| -> $ret:ty { $($body:tt)* }) => {
        vindaloo!(@emit [] [$(($param))+] [-> $ret] { $($body)* })
    };
    (move |$($param:ident),+| $body:expr) => {
        vindaloo!(@emit [] [$(($param))+] [] $body)
    };
    (move |$($parname:ident: $partyp:ty),+| -> $ret:ty { $($body:tt)* }) => {
        vindaloo!(@emit [] [$(($parname: $partyp))+] [-> $ret] { $($body)* })
    };
    (move |$($parname:ident: $partyp:ty),+| $body:expr) => {
        vindaloo!(@emit [] [$(($parname: $partyp))+] [] $body)
    };
}


