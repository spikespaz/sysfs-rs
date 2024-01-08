macro_rules! print_object {
    (
        in $module:path
        [$path_fmt:literal, $($path_arg:expr),*] {
            $($attr_name:ident),* $(,)?
        }
    ) => {{
        use $module::*;
        let attrs = print_object!(
            @call_each,
            [ $($attr_name),* ],
            ( $($path_arg),* )
        );
        let longest = attrs.iter().max_by_key(|(key, _)| key.len()).map_or(0, |(key, _)| key.len());

        println!($path_fmt, $($path_arg),*);
        for (name, value) in attrs {
            println!("    {name:<longest$} = {value}")
        }
    }};
    (@call_each, [ $($getter:ident),* ], $args:tt) => {
        &[ $( {
            let printed = match $getter $args {
                Ok(value) => format!("{value:?}"),
                Err(e) => format!("{e:?}"),
            };
            (stringify!($getter), printed)
        } ),* ]
    };
}

pub(crate) use print_object;
