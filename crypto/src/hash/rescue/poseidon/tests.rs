use math::{FieldElement, StarkField};

use super::{BaseElement, Poseidon64, STATE_WIDTH};

#[test]
fn test_vectors() {
    let neg_one = BaseElement::MODULUS - 1;
    let test_vectors: Vec<([u64; 12], [u64; 12])> = vec![
        (
            [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            [
                0x3c18a9786cb0b359,
                0xc4055e3364a246c3,
                0x7953db0ab48808f4,
                0xc71603f33a1144ca,
                0xd7709673896996dc,
                0x46a84e87642f44ed,
                0xd032648251ee0b3c,
                0x1c687363b207df62,
                0xdf8565563e8045fe,
                0x40f5b37ff4254dae,
                0xd070f637b431067c,
                0x1792b1c4342109d7,
            ],
        ),
        (
            [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11],
            [
                0xd64e1e3efc5b8e9e,
                0x53666633020aaa47,
                0xd40285597c6a8825,
                0x613a4f81e81231d2,
                0x414754bfebd051f0,
                0xcb1f8980294a023f,
                0x6eb2a9e4d54a9d0f,
                0x1902bc3af467e056,
                0xf045d5eafdc6021f,
                0xe4150f77caaa3be5,
                0xc9bfd01d39b50cce,
                0x5c0a27fcb0e1459b,
            ],
        ),
        (
            [
                neg_one, neg_one, neg_one, neg_one, neg_one, neg_one, neg_one, neg_one, neg_one,
                neg_one, neg_one, neg_one,
            ],
            [
                0xbe0085cfc57a8357,
                0xd95af71847d05c09,
                0xcf55a13d33c1c953,
                0x95803a74f4530e82,
                0xfcd99eb30a135df1,
                0xe095905e913a3029,
                0xde0392461b42919b,
                0x7d3260e24e81d031,
                0x10d3d0465d9deaa0,
                0xa87571083dfc2a47,
                0xe18263681e9958f8,
                0xe28e96f1ae5e60d3,
            ],
        ),
        (
            [
                0x8ccbbbea4fe5d2b7,
                0xc2af59ee9ec49970,
                0x90f7e1a9e658446a,
                0xdcc0630a3ab8b1b8,
                0x7ff8256bca20588c,
                0x5d99a7ca0c44ecfb,
                0x48452b17a70fbee3,
                0xeb09d654690b6c88,
                0x4a55d3a39c676a88,
                0xc0407a38d2285139,
                0xa234bac9356386d1,
                0xe1633f2bad98a52f,
            ],
            [
                0xa89280105650c4ec,
                0xab542d53860d12ed,
                0x5704148e9ccab94f,
                0xd3a826d4b62da9f5,
                0x8a7a6ca87892574f,
                0xc7017e1cad1a674e,
                0x1f06668922318e34,
                0xa3b203bc8102676f,
                0xfcc781b0ce382bf2,
                0x934c69ff3ed14ba5,
                0x504688a5996e8f13,
                0x401f3f2ed524a2ba,
            ],
        ),
    ];
    let mut state = [BaseElement::ZERO; STATE_WIDTH];
    let mut expected = [BaseElement::ZERO; STATE_WIDTH];

    test_vectors.iter().for_each(|(a, b)| {
        (0..STATE_WIDTH).for_each(|i| {
            state[i] = BaseElement::new(a[i]);
            expected[i] = BaseElement::new(b[i]);
        });
        Poseidon64::apply_permutation(&mut state);
        assert_eq!(state, expected);
    });
}
