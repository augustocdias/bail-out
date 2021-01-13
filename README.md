# Bail Out

This library is inspired in the `ensure` and `bail` macros from `anyhow`. The difference is that it is not tied to anyhow's types.

Many libraries have their own error types and using the anyhow's `ensure` macro doesn't work because it returns an anyhow error. This library intends to work with any type.

Please check the docs for usage.

Disclaimer: I'm not sure about the naming of some of those macros yet (expect some to change). Feedback is welcome for ergonomics.
