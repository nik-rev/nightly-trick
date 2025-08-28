/////////////////////////////
//
// Example 1: Using `#[feature]` in tests
//
/////////////////////////////

// Use `#[feature]` in tests. Code continue to compile with our MSRV.
//
// ```sh
// # builds with our MSRV
// cargo +1.72.0 run
// # uses nightly
// cargo test
// ```

// only enables this feature in tests
#![cfg_attr(test, feature(hash_map_macro))]

#[cfg(test)]
mod tests {
    #[test]
    fn we_can_even_use_feature() {
        let wow = hash_map! {
            "there's no way" => true
        };

        assert_eq!(
            wow.into_iter().collect::<Vec<_>>(),
            vec![("there's no way", true)]
        )
    }
}

/////////////////////////////
//
// Example 2: Triggering the `incompatible_msrv` lint when we would have
//            accidentally made our crate no longer compile with our MSRV
//
/////////////////////////////

// If you uncomment this function, and run `cargo clippy` -
// a warning will be raised.
//
// `.next_up()` was stabilized in 1.86, but our MSRV is 1.72

// fn triggers_incompatible_msrv_lint() -> f32 {
//     0_f32.next_up()
// }

// However, we CAN use 1.86 API in tests because tests aren't compiled with the
// final code! This makes writing tests nicer because we can use APIs from modern versions
// despite having a low MSRV.

#[cfg(test)]
#[test]
fn does_not_trigger() {
    0_f32.next_up();
}

/////////////////////////////
//
// Example 3: Using unstable rustfmt features
//
/////////////////////////////

mod it {
    pub struct A;
    pub struct B;
}

// Remove the `rustfmt::skip` attribute and the import will be split into 2
//
// This is an unstable rustfmt option which requires nightly, but we can use it in
// our project that compiles on stable!

#[rustfmt::skip] // comment it out
#[allow(unused)]
use it::{A, B};
