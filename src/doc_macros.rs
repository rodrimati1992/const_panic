macro_rules! formatting_docs {($($additional_fmt_overrides:expr)?) => {
concat!("
# Formatting

Literals are Display formatted by default, so that you can pass string literals 
without worrying about what the current formatting settings are.

Expressions are formatted as determined by the `$fmtarg` argument.

### Formatting overrides

You can override how an argument is formatted by prefixing the argument expression with 
any of the options below:
- `debug:` or `{?}:`: `Debug` formats the argument.
- `alt_debug:` or `{#?}:`: alternate-`Debug` formats the argument.
- `display:` or `{}:`: `Display` formats the argument.
- `alt_display:` or `{#}:`: alternate-`Display` formats the argument.
",
$($additional_fmt_overrides,)?
)}}

macro_rules! limitation_docs {() => {
"
Arguments to the formatting/panicking macros must have a fully inferred concrete type, 
because `const_panic` macros use duck typing to call methods on those arguments.

One effect of that limitation is that you will have to pass suffixed 
integer literals (eg: `100u8`) when those integers aren't inferred to be a concrete type.
"
}}
pub(crate) use limitation_docs;
