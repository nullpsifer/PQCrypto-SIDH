mod constant_time {
    fn is_nonzero(x: u64) -> u64 {
        return (x | 0_u64.wrapping_sub(x)) >> 63;
    }

    fn is_zero(x: u64) -> u64 {
        return 1_u64 ^ is_nonzero(x);
    }

    fn is_less_than(x: u64, y: u64) -> u64 {
        // TODO
        return 1_u64;
    }

    #[cfg(test)]
    mod tests {
        use constant_time::*;

        #[test]
        fn is_nonzero_test() {
            assert_eq!(is_nonzero(0), 0);
            assert_eq!(is_nonzero(573), 1);
            assert_eq!(is_nonzero(1), 1);
        }

        #[test]
        fn is_zero_test() {
            assert_eq!(is_zero(0), 1);
            assert_eq!(is_zero(12876), 0);
            assert_eq!(is_zero(1), 0);
        }

        #[test]
        fn is_less_than_test() {
            assert_eq!(is_less_than(0, 0), 0);
            assert_eq!(is_less_than(1, 0), 0);
            assert_eq!(is_less_than(0, 1), 1);
            assert_eq!(is_less_than(2357698, 2357698), 0);
            assert_eq!(is_less_than(295978, 298759789), 1);
            assert_eq!(is_less_than(97289589, 2597), 0);
        }
    }
}


