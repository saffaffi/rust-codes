/*!
This package contains an implementation of the
[ISO 4217](https://www.iso.org/iso-4217-currency-codes.html)
Currency Codes specification.

This standard establishes internationally recognized codes for the
representation of currencies that enable clarity and reduce errors. Currencies
are represented both numerically and alphabetically, using either three digits
or three letters. Some of the alphabetic codes for major currencies are
familiar, such as "EUR" for Euros. Fortunately, ISO 4217 covers everything
from Afghanis to Zambian Kwacha as well.

This package extends the data model of the ISO specification by adding a
currency symbol string (and Unicode code points for the symbol) where possible
to all symbols.

# Example

```rust
use codes_iso_4217::{Currency, ISO_4217};

let code = BZD;

assert_eq!(code.alpha_code(), "BZD");
assert_eq!(code.numeric_code(), Some(84));

// feature = "currency_name"
// assert_eq!(code.currency_name(), "Belize Dollar");

// feature = "country_name"
// assert_eq!(code.country_name(), "BELIZE");

// feature = "monetary_units"
// assert_eq!(code.monetary_units(), 2);

// feature = "is_fund"
// assert_eq!(code.is_fund(), false);

// feature = "historical_codes"
// assert_eq!(code.is_historical(), false);
// assert_eq!(code.withdrawal_date(), None);

// feature = "symbols"
// assert_eq!(code.currency_symbol_str(), Some("BZ$"));
// assert_eq!(code.currency_symbol_code_points(), Some(&[0x42, 0x5a, 0x24]));

assert_eq!(ISO_4217.title(), "Currency codes");
```

# Features

By default only the `serde` feature is enabled, the [Currency::alpha_code] and
[Currency::numeric_code] methods cannot be excluded.

* `serde` - Enables serialization of the [Currency] type.
* `currency_name` - Adds the [Currency::currency_name] method.
* `country_name` - Adds the [Currency::country_name] method.
* `monetary_units` - Adds the [Currency::monetary_units] method.
* `is_fund` - Adds the [Currency::is_fund] method.
* `historical_codes` - Adds the [Currency::is_historical] and [Currency::withdrawal_date] methods.
* `symbols` - Adds the [Currency::currency_symbol_str] and [Currency::currency_symbol_code_points] methods.

*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    private_in_public,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

// ------------------------------------------------------------------------------------------------
//
// The rest of this file is generated by the package build script.
//
// ------------------------------------------------------------------------------------------------

include!(concat!(env!("OUT_DIR"), "/generated.rs"));
