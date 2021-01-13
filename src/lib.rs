//! This library is inspired in the `ensure` and `bail` macros from `anyhow`. The difference is that it is not tied to anyhow's types.
//! Many libraries have their own error types and using the anyhow's `ensure` macro doesn't work because it returns an anyhow error. This library intends to work with any type.


/// Ensures the condition is met. This evaluates to a `Result<(), ERROR>`
/// This macro is equivalent to `if !cond { Err(error) } else { Ok(()) }`.
/// Example:
/// ```
/// use bail_out::*;
/// fn test_err() -> Result<(), &'static str> {
///    ensure!(false, "error")
/// }
/// assert_eq!(test_err(), Err("error"));
///
/// fn test_ok() -> Result<(), &'static str> {
///    ensure!(true, "error")
/// }
/// assert!(test_ok().is_ok());
/// ```
#[macro_export]
macro_rules! ensure {
    ($cond:expr, $error:expr) => {
        if !$cond {
            Err($error)
        } else {
            Ok(())
        }
    };
}

/// Ensures the condition is met or returns the value. This evaluates to a `Result<VALUE, ERROR>`
/// This macro is equivalent to `if !cond { Err(error) } else { Ok(value) }`.
/// Example:
/// ```
/// use bail_out::*;
/// fn test_err() -> Result<&'static str, &'static str> {
///    ensure_or!(false, "ok", "error")
/// }
/// assert_eq!(test_err(), Err("error"));
///
/// fn test_ok() -> Result<&'static str, &'static str> {
///    ensure_or!(true, "ok", "error")
/// }
/// assert_eq!(test_ok(), Ok("ok"));
/// ```
#[macro_export]
macro_rules! ensure_or {
    ($cond:expr, $ok: expr, $error:expr) => {
        if !$cond {
            Err($error)
        } else {
            Ok($ok)
        }
    };
}

/// Ensures the condition is not met. This evaluates to a `Result<(), ERROR>`
/// This macro is equivalent to `if cond { Err(error) } else { Ok(()) }`.
/// Example:
/// ```
/// use bail_out::*;
/// fn test_err() -> Result<(), &'static str> {
///    ensure_not!(true, "error")
/// }
/// assert_eq!(test_err(), Err("error"));
///
/// fn test_ok() -> Result<(), &'static str> {
///    ensure_not!(false, "error")
/// }
/// assert!(test_ok().is_ok());
/// ```
#[macro_export]
macro_rules! ensure_not {
    ($cond:expr, $error:expr) => {
        if $cond {
            Err($error)
        } else {
            Ok(())
        }
    };
}

/// Ensures the condition is not met or returns the value. This evaluates to a `Result<VALUE, ERROR>`
/// This macro is equivalent to `if cond { Err(error) } else { Ok(value) }`.
/// Example:
/// ```
/// use bail_out::*;
/// fn test_err() -> Result<&'static str, &'static str> {
///    ensure_not_or!(true, "ok", "error")
/// }
/// assert_eq!(test_err(), Err("error"));
///
/// fn test_ok() -> Result<&'static str, &'static str> {
///    ensure_not_or!(false, "ok", "error")
/// }
/// assert_eq!(test_ok(), Ok("ok"));
/// ```
#[macro_export]
macro_rules! ensure_not_or {
    ($cond:expr, $ok:expr, $error:expr) => {
        if $cond {
            Err($error)
        } else {
            Ok($ok)
        }
    };
}

/// Return early with an error if the condition is false. This ensures the condition is met.
/// This macro is equivalent to `if !cond { return Err(error) }`.
/// Example:
/// ```
/// use bail_out::*;
/// fn test_err() -> Result<(), &'static str> {
///    ensure_bail!(false, "error");
///    Ok(())
/// }
/// assert_eq!(test_err(), Err("error"));
///
/// fn test_ok() -> Result<(), &'static str> {
///    ensure_bail!(true, "ok");
///    Ok(())
/// }
/// assert_eq!(test_ok(), Ok(()));
/// ```
#[macro_export]
macro_rules! ensure_bail {
    ($cond:expr, $error:expr) => {
        if !$cond {
            $crate::bail!($error);
        }
    };
}

/// Return early with an error if the condition is true. This ensures the condition is not met.
/// This macro is equivalent to `if cond { return Err(error) }`.
/// Example:
/// ```
/// use bail_out::*;
/// fn test_err() -> Result<(), &'static str> {
///    ensure_bail_not!(true, "error");
///    Ok(())
/// }
/// assert_eq!(test_err(), Err("error"));
///
/// fn test_ok() -> Result<(), &'static str> {
///    ensure_bail_not!(false, "ok");
///    Ok(())
/// }
/// assert_eq!(test_ok(), Ok(()));
/// ```
#[macro_export]
macro_rules! ensure_bail_not {
    ($cond:expr, $error:expr) => {
        if $cond {
            $crate::bail!($error);
        }
    };
}

/// Return early with an error.
/// This macro is equivalent to `return Err(error)`.
/// Example:
/// ```
/// use bail_out::*;
/// fn bail_test() -> Result<(), &'static str> {
///    bail!("error");
///    Ok(())
/// }
/// assert_eq!(bail_test(), Err("error"));
/// ```
#[macro_export]
macro_rules! bail {
    ($error:expr) => {
        return Err($error);
    };
}

#[cfg(test)]
mod tests;
