macro_rules! formatting_docs {() => {
"
# Formatting

Literals are Display formatted by default, so that you can pass string literals 
without worring about what the current formatting settings are.

Expressions are formatted as determined by the `FmtArg` argument.

You can override how an argument is formatted by prefixing the argument with 
any of the options below:
- `debug:` or `{?}:`: `Debug` formats the argument.
- `display:` or `{}:`: `Display` formats the argument.
"
}}
