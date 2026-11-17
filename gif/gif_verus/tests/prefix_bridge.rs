//! Runtime regressions for the universal L-PREFIX and Strict-call bridges.
use gif_verus::{try_decode_still, try_decode_still_with_policy, Frames, Profile, TryError};

// Frozen two-colour-m2: white, black, white; byte 33 is the first terminator.
const TWO_COLOUR: &[u8] = &[
    71, 73, 70, 56, 57, 97, 3, 0, 1, 0, 128, 0, 0,
    255, 255, 255, 0, 0, 0, 44, 0, 0, 0, 0, 3, 0, 1, 0, 0, 2, 2, 68, 80, 0, 59,
];

#[test]
fn still_ignores_every_replacement_tail_after_first_terminator() {
    let expected = try_decode_still(TWO_COLOUR).unwrap();
    assert_eq!(expected.rgba, [255, 255, 255, 255, 0, 0, 0, 255, 255, 255, 255, 255]);
    assert_eq!(try_decode_still(&TWO_COLOUR[..34]).unwrap(), expected);
    for byte in 0..=u8::MAX {
        let mut replacement = TWO_COLOUR[..34].to_vec();
        replacement.extend_from_slice(&[byte, 0x21, 0xf9, 4, 0xff, 0xff, 0xff, 0xff]);
        assert_eq!(try_decode_still(&replacement).unwrap(), expected);
        let (profiled, recovered) = try_decode_still_with_policy(&replacement, Profile::Strict).unwrap();
        assert_eq!(profiled, expected);
        assert!(!recovered);
    }
}

#[test]
fn strict_entry_points_agree_on_all_cuts_and_trailing_error() {
    for cut in 0..=TWO_COLOUR.len() {
        let b = &TWO_COLOUR[..cut];
        match (try_decode_still(b), try_decode_still_with_policy(b, Profile::Strict)) {
            (Ok(a), Ok((c, recovered))) => {
                assert_eq!(a, c);
                assert!(!recovered);
            }
            (Err(TryError::Format(a)), Err(TryError::Format(c))) => assert_eq!(a, c),
            pair => panic!("Strict still bridge failed at cut {cut}: {pair:?}"),
        }
        match (Frames::try_new(b), Frames::try_new_with_policy(b, Profile::Strict)) {
            (Ok(mut a), Ok(mut c)) => {
                assert_eq!(a.info(), c.info());
                for _ in 0..3 {
                    match (a.next_frame(), c.next_frame()) {
                        (None, None) => (),
                        (Some(Err(TryError::Format(x))), Some(Err(TryError::Format(y)))) => assert_eq!(x, y),
                        (Some(Ok(v)), Some(Ok(w))) => {
                            assert_eq!(v.rgba, w.rgba);
                            assert_eq!(v.rect, w.rect);
                            assert_eq!(v.delay_cs, w.delay_cs);
                            assert_eq!(v.disposal, w.disposal);
                            assert_eq!(v.user_input, w.user_input);
                            assert_eq!(v.transparent, w.transparent);
                            assert_eq!(v.interlaced, w.interlaced);
                            assert!(!v.recovered && !w.recovered);
                        }
                        pair => panic!("Strict iterator bridge failed at cut {cut}: {pair:?}"),
                    }
                }
            }
            (Err(TryError::Format(a)), Err(TryError::Format(c))) => assert_eq!(a, c),
            _ => panic!("Strict constructor bridge failed at cut {cut}"),
        }
    }
    let mut trailing = TWO_COLOUR.to_vec();
    trailing.push(0x21);
    for mut decoder in [Frames::try_new(&trailing).unwrap(), Frames::try_new_with_policy(&trailing, Profile::Strict).unwrap()] {
        assert!(!decoder.next_frame().unwrap().unwrap().recovered);
        assert!(matches!(decoder.next_frame(), Some(Err(TryError::Format(gif_verus::Error::TrailingBytes)))));
        assert!(decoder.next_frame().is_none());
    }
}
