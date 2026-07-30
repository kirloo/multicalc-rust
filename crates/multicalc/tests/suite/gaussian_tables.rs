#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

#[cfg(any(
    feature = "gauss-legendre",
    feature = "gauss-hermite",
    feature = "gauss-laguerre"
))]
mod gaussian_tables_test {

    use multicalc::gaussian_tables::{MAX_ORDER, nodes};
    use multicalc::numerical_integration::mode::GaussianQuadratureMethod;

    use proptest::prelude::*;

    #[test]
    fn lookup() {
        #[cfg(feature = "gauss-legendre")]
        assert_eq!(
            nodes(GaussianQuadratureMethod::GaussLegendre, 4)
                .unwrap()
                .len(),
            4
        );
        #[cfg(feature = "gauss-legendre")]
        assert!(nodes(GaussianQuadratureMethod::GaussLegendre, 0).is_err());
        #[cfg(feature = "gauss-hermite")]
        assert!(nodes(GaussianQuadratureMethod::GaussHermite, MAX_ORDER + 1).is_err());
    }

    proptest! {
        #[test]
        fn proptest_lookup(order in 1..=MAX_ORDER) {
            #[cfg(feature = "gauss-legendre")]
            prop_assert_eq!(
                nodes(GaussianQuadratureMethod::GaussLegendre, order)
                    .unwrap()
                    .len(),
                order
            );
            #[cfg(feature = "gauss-hermite")]
            prop_assert_eq!(
                nodes(GaussianQuadratureMethod::GaussHermite, order)
                    .unwrap()
                    .len(),
                order
            );
            #[cfg(feature = "gauss-laguerre")]
            prop_assert_eq!(
                nodes(GaussianQuadratureMethod::GaussLaguerre, order)
                    .unwrap()
                    .len(),
                order
            );
        }
    }
}
