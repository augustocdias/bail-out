# Bail Out

This library is inspired in the `ensure` and `bail` macros from `anyhow`. The difference is that it is not tied to anyhow's types.

Many libraries have their own error types and using the anyhow's `ensure` macro doesn't work because it returns an anyhow error. This library intends to work with any type.

This library also provides the `assure` macro, that does the same but evaluates to a `Result` instead of returning. This is useful for using inside `try` blocks.

Please check the docs for usage.
