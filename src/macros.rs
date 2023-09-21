/// Formats variables to query params
///
/// If no name is provided variable name is used as params name
#[allow(unused_macros)]
macro_rules! query {
    ($value:ident) => {
        format!("&{}={}", stringify!($value), $value)
    };
    ($value:ident, $name:expr) => {
        format!("&{}={}", $name, $value)
    };
}

/// Formats optional query params
///
/// If the Optional is None output is empty String. Variable name is used
/// as param name if no name is provided.
macro_rules! optional_query {
    ($value:ident) => {
        match $value {
            Some(val) => format!("&{}={}", stringify!($value), val),
            None => String::new(),
        }
    };
    ($value:ident, $name:expr) => {
        match $value {
            Some(val) => format!("&{}={}", $name, val),
            None => String::new(),
        }
    };
}

/// Formats vector query params
///
/// If the vector is empty an empty String is outputted. Variable name is used
/// as param name if no name is provided.
macro_rules! vec_query {
    ($value:ident) => {
        if $value.is_empty() {
            String::new()
        } else {
            format!("&{}={}", stringify!($value), $value.join(","))
        }
    };
    ($value:ident, $name:expr) => {
        if $value.is_empty() {
            String::new()
        } else {
            format!("&{}={}", $name, $value.join(","))
        }
    };
}
