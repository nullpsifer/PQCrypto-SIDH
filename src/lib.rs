// While we're still stubbing things out...
#![allow(dead_code, unused_variables)]

// Assume we're using 64-bit words for all this
const RADIX: usize = 64;

enum Actor {
    Alice,
    Bob,
}

struct ActorMap<T> {
    alice: T,
    bob: T,
}

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

mod p503 {
    use RADIX;
    use ActorMap;
    
    // p503_ZERO_WORDS
    const P503_PLUS_ONE_SUFFIX_ZERO_WORD_COUNT: usize = 3;

    // NBITS_FIELD
    const ELEMENT_BITS: usize = 503;
    // MAXWORDS_FIELD
    const ELEMENT_WORDS: usize = (ELEMENT_BITS + RADIX - 1) / RADIX;
    // OALICE_BITS, OBOB_BITS
    const ORDER_BITS: ActorMap<usize> = ActorMap {
        alice: 250,
        bob: 253,
    };
    // NWORDS_ORDER
    const ORDER_WORDS: usize = (ORDER_BITS.bob + RADIX - 1) / RADIX;
    // MASK_ALICE, MASK_BOB
    const MASK: ActorMap<u8> = ActorMap {
        alice: 0x03,
        bob: 0x0F,
    };
    
    // felm_t
    type Element = [u64; ELEMENT_WORDS];
    // dfelm_t
    type DoubleElement = [u64; 2*ELEMENT_WORDS];
    // f2elm_t
    type TwoElements = [Element; 2];

    // struct point_proj, point_proj_t (wtf do they need both of these?)
    struct PointProj {
        x: TwoElements,
        z: TwoElements,
    }

    fn copy_words(a: &[u64], c: &mut[u64], num_words: usize) {
        // TODO
    }

    fn mp_add(a: &[u64], b: &[u64], c: &mut[u64], num_words: usize) {
        // TODO
    }

    fn mp_sub(a: &[u64], b: &[u64], c: &mut[u64], num_words: usize) {
        // TODO
    }

    fn mp_shiftleft(x: &mut[u64], shift: usize, num_words: usize) {
        // TODO
    }

    fn mp_shiftr1(x: &mut[u64], num_words: usize) {
        // TODO
    }

    // TODO should pull request to fix the comment on Microsoft's repo for this one
    fn mp_shiftl1(x: &mut[u64], num_words: usize) {
        // TODO
    }

    fn digit_x_digit(a: u64, b: u64, c: &mut[u64], num_words: usize) {
        // TODO
    }

    fn mp_mul(a: &[u64], b: &[u64], c: &mut[u64], num_words: usize) {
        // TODO
    }

    fn fpcopy503(a: &Element, c: &mut Element) {
        // TODO
    }

    fn fpzero503(a: &mut Element) {
        // TODO
    }

    fn fpequal503_non_constant_time(a: &Element, c: &Element) -> bool {
        // TODO
        return true;
    }

    fn fpadd503(a: &Element, b: &Element, c: &mut Element) {
        // TODO
    }

    fn fpsub503(a: &Element, b: &Element, c: &mut Element) {
        // TODO
    }

    fn fpneg503(a: &mut Element) {
        // TODO
    }

    fn fpdiv2_503(a: &Element, c: &mut Element) {
        // TODO
    }

    fn fpcorrection503(a: &mut Element) {
        // TODO
    }

    fn rdc_mont(a: &Element, c: &mut Element) {
        // TODO
    }

    fn fpmul503_mont(a: &Element, b: &Element, c: &mut Element) {
        // TODO
    }

    fn fpsqr503_mont(ma: &Element, mc: &mut Element) {
        // TODO
    }

    fn to_mont(a: &Element, mc: &mut Element) {
        // TODO
    }

    fn from_mont(ma: &Element, c: &mut Element) {
        // TODO
    }

    fn fpinv503_mont(a: &mut Element) {
        // TODO
    }

    fn fpinv503_mont_bingcd(a: &mut Element) {
        // TODO
    }

    fn fpinv503_chain_mont(a: &mut Element) {
        // TODO
    }
}
