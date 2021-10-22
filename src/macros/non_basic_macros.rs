/// Formats multiple values into an array of `PanicVal`s.
///
/// The `flatten`ing part comes from the fact that each argument
/// is potentially converted to an array of `PanicVal`,
/// which are then concatenated into a single array.
///
/// # Arguments
///
/// The syntax for this macro is
/// ```text
/// flatten_panicvals!(
///     $fmtarg:expr;
///     $(
///         $($Type:ty => )? $($fmt_override:tt :)? $arg_to_fmt:expr
///     ),*
///     $()?
/// )
/// ```
///
/// `$fmtarg` is a [`FmtArg`](crate::FmtArg) argument
/// which determines how non-literal `$arg_to_fmt` arguments are formatted.
///
/// [`$format_override`](#formatting-overrides) overrides the `$fmtarg` argument,
/// changing how that `$arg_to_fmt` argument is formatted.
///
/// `$arg_to_fmt` are the formatted arguments,
/// which must implement the [`PanicFmt`](crate::fmt::PanicFmt) trait.
///
/// If the `$Type =>` syntax is used, this calls the `to_panicvals`
/// method on `$arg_to_fmt`.<br>
/// If the `$Type =>` syntax is *not* used, this calls the `to_panicval`
/// method on `$arg_to_fmt`.
///
/// These is the signature of those methods:
/// ```rust
/// # use const_panic::{FmtArg, PanicFmt, PanicVal};
/// # struct Foo;
/// #
/// # impl PanicFmt for Foo {
/// #    type This = Self;
/// #    type Kind = const_panic::fmt::IsCustomType;
/// #    const PV_COUNT: usize = 1;
/// # }
/// # impl Foo {
/// const fn to_panicvals(&self, f: FmtArg) -> [PanicVal<'_>; <Foo as PanicFmt>::PV_COUNT]
/// #   { loop{} }
/// # }
/// ```
/// ```rust
/// # use const_panic::{FmtArg, PanicVal};
/// # struct Bar;
/// #
/// # impl Bar {
/// const fn to_panicval(&self, f: FmtArg) -> PanicVal<'_>
/// #   { loop{} }
/// # }
/// ```
///
#[doc = formatting_docs!("
- `open`: increments `$fmtarg`'s indentation by [`fmt::INDENTATION_STEP`]
before formatting the argument, doesn't change formatting otherwise.

- `close`: decrements `$fmtarg`'s indentation by [`fmt::INDENTATION_STEP`]
before formatting the argument, doesn't change formatting otherwise.

[`fmt::INDENTATION_STEP`]: crate::fmt::INDENTATION_STEP
")]
///
///
/// # Examples
///
/// [Struct](#struct-formatting) and [Enum](#enum-formatting) Formatting examples below.
///
/// ### Basic
///
/// ```rust
/// use const_panic::{ArrayString, FmtArg, flatten_panicvals};
///
///
/// assert_eq!(
///     ArrayString::<999>::from_panicvals(
///         // Formatting literals
///         &flatten_panicvals!(FmtArg::DEBUG; 100u8, "hello")
///     ).unwrap(),
///     "100hello"
/// );
///
/// assert_eq!(
///     ArrayString::<999>::from_panicvals(
///         // Formatting non-literals.
///         // `"foo"` is considered a non-literal, because it's inside other tokens.
///         &flatten_panicvals!(FmtArg::ALT_DEBUG; ("foo"), [100u8, 200])
///     ).unwrap(),
///     concat!(
///         "\"foo\"[\n",
///         "    100,\n",
///         "    200,\n",
///         "]",
///     )
/// );
///
/// assert_eq!(
///     ArrayString::<999>::from_panicvals(&
///         // Alternate-Debug Formatting composite types.
///         flatten_panicvals!(
///             FmtArg::ALT_DEBUG;
///             Foo => Foo(3, "5"),
///             ", ",
///             Bar => Bar{x: "hello"}
///         )
///     ).unwrap(),
///     concat!(
///         "Foo(\n",
///         "    3,\n",
///         "    \"5\",\n",
///         "), Bar {\n",
///         "    x: \"hello\",\n",
///         "}",
///     )
/// );
///
/// assert_eq!(
///     ArrayString::<999>::from_panicvals(&
///         // Overriding the formatting of arguments.
///         //
///         // The `open` and `close` overrides are demonstrated in the
///         // struct and enum examples.
///         flatten_panicvals!(
///             FmtArg::DEBUG;
///             Foo => display: Foo(3, "5"),
///             debug: ", ",
///             Bar => Bar{x: "hello"}
///         )
///     ).unwrap(),
///     r#"Foo(3, 5)", "Bar { x: "hello" }"#
/// );
///
///
///
/// struct Foo(u32, &'static str);
///
/// const_panic::impl_panicfmt!{
///     impl Foo;
///     struct Foo(u32, &'static str)
/// }
///
/// struct Bar {
///     x: &'static str,
/// }
///
/// const_panic::impl_panicfmt!{
///     impl Bar;
///     struct Bar {
///         x: &'static str,
///     }
/// }
///
/// ```
///
/// ### Struct formatting
///
/// Implementing this trait for braced and tuple structs.
///
/// ```rust
/// use const_panic::{
///     fmt::{self, FmtArg, PanicFmt, PvCountForStruct},
///     ArrayString, PanicVal,
///     flatten_panicvals,
/// };
///
/// fn main(){
///     let foo = Foo {
///         x: &[3, 5, 8, 13],
///         y: 21,
///         z: Bar(false, true),
///     };
///     
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&foo.to_panicvals(FmtArg::DEBUG)).unwrap(),
///         "Foo { x: [3, 5, 8, 13], y: 21, z: Bar(false, true) }",
///     );
///     assert_eq!(
///         ArrayString::<200>::from_panicvals(&foo.to_panicvals(FmtArg::ALT_DEBUG)).unwrap(),
///         concat!(
///             "Foo {\n",
///             "    x: [\n",
///             "        3,\n",
///             "        5,\n",
///             "        8,\n",
///             "        13,\n",
///             "    ],\n",
///             "    y: 21,\n",
///             "    z: Bar(\n",
///             "        false,\n",
///             "        true,\n",
///             "    ),\n",
///             "}",
///         ),
///     );
/// }
///
///
///
/// #[cfg(feature = "non_basic")]
/// #[derive(Debug)]
/// struct Foo<'a> {
///     x: &'a [u8],
///     y: u8,
///     z: Bar,
/// }
///
/// #[cfg(feature = "non_basic")]
/// #[derive(Debug)]
/// struct Bar(bool, bool);
///
///
/// impl PanicFmt for Foo<'_> {
///     type This = Self;
///     type Kind = const_panic::fmt::IsCustomType;
///
///     // `PvCountForStruct` allows computing the length of the array of `PanicVal`s
///     // returned by `Foo::to_panicvals` below.
///     //
///     // Note that PvCountForStruct only calculates the correct number if you
///     // follow the pattern in this example.
///     const PV_COUNT: usize = PvCountForStruct{
///         field_amount: 3,
///         summed_pv_count: <&[u8]>::PV_COUNT
///             + <u8>::PV_COUNT
///             + <Bar>::PV_COUNT,
///         delimiter: fmt::StructDelim::Braced,
///     }.call();
/// }
///
/// impl<'a> Foo<'a> {
///     const fn to_panicvals(&self, fmtarg: FmtArg) -> [PanicVal<'a>; Foo::PV_COUNT] {
///         // These constants from `fmt` add newlines and padding
///         // when the `fmtarg.is_alternate` flag is enabled,
///         // to match the standard behavior for `Debug` formatting.
///         flatten_panicvals! {fmtarg;
///             "Foo",
///             // the `open:` format override increments `fmtarg.indentation`
///             // by `const_panic::fmt::INDENTATION_STEP` spaces.
///             // The indentation field is used by these constants when the
///             // `fmtarg.is_alternate` flag is enabled.
///             open: fmt::OPEN_BRACE,
///                 // fmt::COMMA_SEP must only be used between fields
///                 "x: ", &[u8] => self.x, fmt::COMMA_SEP,
///                 "y: ", u8 => self.y, fmt::COMMA_SEP,
///                 // fmt::COMMA_TERM must only be used after the last field
///                 "z: ", Bar => self.z, fmt::COMMA_TERM,
///             // the `close:` format override decrements the indentation.
///             close: fmt::CLOSE_BRACE,
///         }
///     }
/// }
///
/// impl PanicFmt for Bar {
///     type This = Self;
///     type Kind = const_panic::fmt::IsCustomType;
///
///     const PV_COUNT: usize = PvCountForStruct{
///         field_amount: 2,
///         summed_pv_count: <bool>::PV_COUNT * 2,
///         delimiter: fmt::StructDelim::Tupled,
///     }.call();
/// }
///
/// impl Bar {
///     const fn to_panicvals(&self, f: FmtArg) -> [PanicVal<'static>; Bar::PV_COUNT] {
///         flatten_panicvals! {f;
///             "Bar",
///             open: fmt::OPEN_PAREN,
///                 // fmt::COMMA_SEP must only be used between fields
///                 self.0, fmt::COMMA_SEP,
///                 // fmt::COMMA_TERM must only be used after the last field
///                 self.1, fmt::COMMA_TERM,
///             close: fmt::CLOSE_PAREN,
///         }
///     }
/// }
/// ```
///
/// ### Enum Formatting
///
/// This example demonstrates formatting of generic enum types.
///
/// ```rust
/// use const_panic::{
///     fmt::{self, FmtArg, PanicFmt, PvCountForStruct},
///     ArrayString, PanicVal,
///     flatten_panicvals,
/// };
///
/// fn main() {
///     let up: Qux<u8> = Qux::Up;
///     // Debug formatting the Up variant
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&up.to_panicvals(FmtArg::DEBUG)).unwrap(),
///         "Up",
///     );
///
///
///     let down: Qux<u16> = Qux::Down { x: 21, y: 34, z: 55 };
///     // Debug formatting the Down variant
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&down.to_panicvals(FmtArg::DEBUG)).unwrap(),
///         "Down { x: 21, y: 34, z: 55 }",
///     );
///     // Alternate-Debug formatting the Down variant
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&down.to_panicvals(FmtArg::ALT_DEBUG)).unwrap(),
///         concat!(
///             "Down {\n",
///             "    x: 21,\n",
///             "    y: 34,\n",
///             "    z: 55,\n",
///             "}",
///         )
///     );
///
///
///     let left: Qux<u32> = Qux::Left(89);
///     // Debug formatting the Left variant
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&left.to_panicvals(FmtArg::DEBUG)).unwrap(),
///         "Left(89)",
///     );
///     // Alternate-Debug formatting the Left variant
///     assert_eq!(
///         ArrayString::<100>::from_panicvals(&left.to_panicvals(FmtArg::ALT_DEBUG)).unwrap(),
///         concat!(
///             "Left(\n",
///             "    89,\n",
///             ")",
///         )
///     );
/// }
///
/// #[cfg(feature = "non_basic")]
/// #[derive(Debug)]
/// enum Qux<T> {
///     Up,
///     Down { x: T, y: T, z: T },
///     Left(u64),
/// }
///
///
/// impl<T: PanicFmt> PanicFmt for Qux<T> {
///     type This = Self;
///     type Kind = const_panic::fmt::IsCustomType;
///
///     const PV_COUNT: usize = {
///         // `PvCountForStruct` computes the length of the array of `PanicVal`s
///         // produced by each variant.
///         //
///         // `slice_max_usize` returns the maximum usize in a slice.
///         // In this case, to return the longest array produced by the variants.
///         const_panic::utils::slice_max_usize(&[
///             PvCountForStruct{
///                 field_amount: 0,
///                 summed_pv_count: 0,
///                 delimiter: fmt::StructDelim::Braced,
///             }.call(),
///             PvCountForStruct{
///                 field_amount: 3,
///                 summed_pv_count: <T>::PV_COUNT * 3,
///                 delimiter: fmt::StructDelim::Braced,
///             }.call(),
///             PvCountForStruct{
///                 field_amount: 1,
///                 summed_pv_count: <u64>::PV_COUNT,
///                 delimiter: fmt::StructDelim::Tupled,
///             }.call(),
///         ])
///     };
/// }
///
/// // Because of limitations of stable const evaluation,
/// // you have to use macros to implement the `to_panicvals` method
/// // for more than one concrete type.
/// //
/// // This macro implements panic formatting for
/// // - `Qux<u8>`
/// // - `Qux<u16>`
/// // - `Qux<u32>`
/// const_panic::inline_macro! {
///     (u8),
///     (u16),
///     (u32);
///
///     ($T:ty) =>
///
///     impl Qux<$T> {
///         pub const fn to_panicvals(
///             &self,
///             fmtarg: FmtArg,
///         ) -> [PanicVal<'static>; <Qux<$T>>::PV_COUNT] {
///             match self {
///                 Self::Up =>
///                     // The `<Qux<$T>>::PV_COUNT` argument tells `flatten_panicvals` to
///                     // create an array of that length.
///                     // Variants that would otherwise produce shorter arrays
///                     // pad that array with trailing `PanicVal::EMPTY`.
///                     flatten_panicvals! {fmtarg, <Qux<$T>>::PV_COUNT;
///                         "Up"
///                     },
///                 Self::Down{x, y, z} =>
///                     // These constants from `fmt` add newlines and padding
///                     // when the `fmtarg.is_alternate` flag is enabled,
///                     // to match the standard behavior for `Debug` formatting.
///                     flatten_panicvals! {fmtarg, <Qux<$T>>::PV_COUNT;
///                         "Down",
///                         // the `open:` format override increments `fmtarg.indentation`
///                         // by `const_panic::fmt::INDENTATION_STEP` spaces.
///                         // The indentation field is used by these constants when the
///                         // `fmtarg.is_alternate` flag is enabled.
///                         open: fmt::OPEN_BRACE,
///                             // fmt::COMMA_SEP must only be used between fields
///                             "x: ", x, fmt::COMMA_SEP,
///                             "y: ", y, fmt::COMMA_SEP,
///                             // fmt::COMMA_TERM must only be used after the last field
///                             "z: ", z, fmt::COMMA_TERM,
///                         close: fmt::CLOSE_BRACE,
///                     },
///                 Self::Left(x) => flatten_panicvals! {fmtarg, <Qux<$T>>::PV_COUNT;
///                     "Left",
///                     open: fmt::OPEN_PAREN,
///                         x, fmt::COMMA_TERM,
///                     close: fmt::CLOSE_PAREN,
///                 },
///             }
///         }
///     }
/// }
/// ```
///
///
/// [`PanicVal`]: crate::PanicVal
///
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
#[macro_export]
macro_rules! flatten_panicvals {
    ($fmtargs:expr $(, $length:expr)?; $($args:tt)* ) => {{
        let mut fmtargs: $crate::FmtArg = $fmtargs;
        $crate::__to_pvf_inner!(fmtargs [$(length($length))?][$($args)* ,])
    }};
}

#[doc(hidden)]
#[macro_export]
macro_rules! __to_pvf_inner {
    (
        $fmtargs:ident
        [
            $(length($expected_length:expr))?
            $((($len:expr, $kind:ident $args:tt), $fmt_override:tt, $reff:expr))*
        ]
        [$(,)*]
    ) => ({
        const __ADDED_UP_LEN_SDOFKE09F__: $crate::__::usize = 0 $( + $len )*;
        const __LEN_SDOFKE09F__: $crate::__::usize =
            $crate::__to_pvf_used_length!(__ADDED_UP_LEN_SDOFKE09F__, $($expected_length)?);

        $(
            const _: () =
                $crate::__::assert_flatten_panicvals_length(
                    $expected_length,
                    __ADDED_UP_LEN_SDOFKE09F__,
                );
        )?
        $crate::__::flatten_panicvals::<__LEN_SDOFKE09F__>(&[
            $(
                $crate::__to_pvf_kind!($fmtargs $kind $args, $fmt_override, $reff)
            ),*
        ])
    });
    ($fmtargs:ident [$($prev:tt)*] [_, $($rem:tt)*]) => {
        $crate::__to_pvf_inner!{
            $fmtargs

            [$($prev)* ((1, single()), _, $crate::PanicVal::EMPTY)]

            [$($rem)*]
        }
    };
    // Had to add this to work around `()`/`[]`-delimited expressions getting
    // stuck being parsed as a type in the next branch.
    ($fmtargs:ident $prev:tt [$tt:tt, $($rem:tt)*]) => {
        $crate::__to_pvf_expr!{
            $fmtargs
            $prev
            (1, single())
            [$tt, $($rem)*]
        }
    };
    ($fmtargs:ident $prev:tt [$ty:ty => $($rem:tt)*]) => {
        $crate::__to_pvf_expr!{
            $fmtargs
            $prev
            (<$ty as $crate::__::PanicFmt>::PV_COUNT, many($ty))
            [$($rem)*]
        }
    };
    ($fmtargs:ident $prev:tt [$($rem:tt)*]) => {
        $crate::__to_pvf_expr!{
            $fmtargs
            $prev
            (1, single())
            [$($rem)*]
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __to_pvf_expr {
    ($fmtargs:ident [$($prev:tt)*] $other:tt [$kw:tt: $reff:expr, $($rem:tt)*]) => {
        $crate::__to_pvf_inner!{
            $fmtargs

            [$($prev)* ($other, $kw, $reff)]

            [$($rem)*]
        }
    };
    ($fmtargs:ident [$($prev:tt)*] $other:tt [$reff:literal, $($rem:tt)*])=>{
        $crate::__to_pvf_inner!{
            $fmtargs

            [$($prev)* ($other, display, $reff)]

            [$($rem)*]
        }
    };
    ($fmtargs:ident [$($prev:tt)*] $other:tt [$reff:expr, $($rem:tt)*]) => {
        $crate::__to_pvf_inner!{
            $fmtargs

            [$($prev)* ($other, _, $reff)]

            [$($rem)*]
        }
    };
    ($fmtargs:ident [$($prev:tt)*] $other:tt [$($rem:tt)*]) => {
        $crate::__::compile_error!(concat!(
            "expected expression, found:",
            stringify!($($rem)*)
        ))
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __to_pvf_kind {
    ($fmtargs:ident single (), $fmt_override:tt, $reff:tt) => {
        &match &$reff {
            reff => [$crate::__::PanicFmt::PROOF
                .infer(reff)
                .coerce(reff)
                .to_panicval($crate::__set_fmt_from_kw!($fmt_override, $fmtargs))],
        }
    };
    ($fmtargs:ident many ($ty:ty), $fmt_override:tt, $reff:tt) => {
        $crate::__::panicvals_id::<{ <$ty as $crate::__::PanicFmt>::PV_COUNT }>(&match &$reff {
            reff => <$ty as $crate::__::PanicFmt>::PROOF
                .coerce(reff)
                .to_panicvals($crate::__set_fmt_from_kw!($fmt_override, $fmtargs)),
        })
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! __to_pvf_used_length {
    ( $added_up:expr, $expected_length:expr ) => {
        $crate::utils::max_usize($added_up, $expected_length)
    };
    ( $added_up:expr, ) => {
        $added_up
    };
}

/// Helper macro for defining and using a `macro_rules!` macro inline.
#[cfg_attr(feature = "docsrs", doc(cfg(feature = "non_basic")))]
#[macro_export]
macro_rules! inline_macro{
    (
        $(($($args:tt)*)),* $(,)?;
        ($($params:tt)*)
        =>
        $($code:tt)*
    ) => {
        macro_rules! __dummy__ {
            ($($params)*) => {$($code)*}
        }
        $(
            __dummy__!{ $($args)* }
        )*
    }
}
