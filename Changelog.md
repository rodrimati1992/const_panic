This changelog is a summary of the changes made in each release.

# 0.2

### 0.2.0

Added `concat_assert` macro.

Made breaking change to `impl_panicfmt` to allow generic implementations with type and const parameters.

Added `PanicFmt` derive macro.

Added `"derive"` crate feature, to enable the `PanicFmt` derive.

Declared `const_panic_proc_macros`, depended by `const_panic` when the `"derive"` feature is enabled.

# 0.1

### 0.1.1

Added `PanicFmt`-based formatting for these types(all of which require the `"non_basic"` feature):
- `Option`s of integer, bool, and `&str`
- `Option`s of arrays and slices (of integer, bool, and `&str`)
- `NonZero*` integers, and `Option`s of them
- `NonNull`, and `Option`s of them
- `*const T` and `*mut T`
- `std::cmp::Ordering`, and `Option`s of them
- `std::sync::atomic::Ordering`
- `std::ops::Range*` types, parameterized with `usize`.
- `()`
- `std::marker::PhantomData`
- `std::marker::PhantomPinned`
- `StdWrapper`

Added these macros:
- `unwrap_ok`
- `unwrap_err`
- `unwrap_some`

Fixed signature of to_panicvals for arrays and slices of PanicVals, by adding a `FmtArg` parameter.


### 0.1.0

Defined the `fmt::PanicFmt` trait.

Defined these types in the `fmt` module:
- `ComputePvCount`
- `FmtArg`
- `IsCustomType`
- `IsStdType`
- `IsPanicFMt`
- `Separator`
- `Delimiter`
- `FmtKind`
- `IsLast`
- `TypeDelim`
- `ShortString` (type alias for `ArrayString<16>`)

Defined these constants in the `fmt` module:
- `COMMA_SEP`
- `COMMA_TERM`
- `INDENTATION_STEP`

Re-exported these variants from `fmt::Delimiter` in `fmt`:
- `CloseBrace`
- `CloseBracket`
- `CloseParen`
- `Empty` renamed to `EmptyDelimiter`
- `OpenBrace`
- `OpenBracket`
- `OpenParen`

Reexported these items from `fmt` in the root module:
- `FmtArg`
- `IsCustomType`
- `PanicFmt`
- `ComputePvCount`
- `TypeDelim`


Defined these macros:
- `coerce_fmt`
- `concat_panic`: for panicking with formatted arguments.
- `flatten_panicvals`: for flattening the argument slices of `PanicVal`s into an array.
- `impl_panicfmt`: for user-defined structs and enums to implement  `PanicFmt`
- `inline_macro`

Implemented `PanicFmt`-based formatting for:
- All the primitive integer types
- `str`
- `bool`
- Arrays and slices of `PanicVal` integers, `bool`, and `&str`. 

Defined the `ArrayString` stack allocated string type.

Defined the `PanicVal` opaque enum used for formatting.

Defined the `StdWrapper` wrapper type for defining methods on `std` types

Defined the `concat_panic` function, for panicking with formatted arguments.

Defined the `"non_basic"` crate feature, 
which enables all items for doing more than panicking with primitive types.



