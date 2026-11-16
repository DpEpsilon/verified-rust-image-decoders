//! Runtime checks of the verified, stack-free executable helper.
use lzw_proof::emission::emit_backwards;

fn check_emission(clear: usize, code: usize, prefix: &[u16], suffix: &[u8], expected: &[u8]) {
    let q0 = 5;
    for available in [expected.len(), expected.len() - 1, 0] {
        let area = q0 + available;
        let mut out = vec![0xcc; area + 7];
        let first = emit_backwards(code, clear, q0, area, expected.len(), suffix, prefix, &mut out);
        assert_eq!(first, expected[0]);
        assert_eq!(&out[q0..area], &expected[..available]);
        assert!(out[..q0].iter().all(|&b| b == 0xcc));
        assert!(out[area..].iter().all(|&b| b == 0xcc));
    }
}

#[test]
fn longest_stored_chain_and_high_minimum_code_size() {
    for clear in [4, 256, 2048] {
        let mut prefix = vec![u16::MAX; 4096];
        let mut suffix = vec![0xee; 4096];
        let root = if clear > 256 { 200 } else { clear - 1 };
        let mut expected = vec![root as u8];
        for c in clear + 2..4096 {
            prefix[c] = if c == clear + 2 { root } else { c - 1 } as u16;
            suffix[c] = (c % 256) as u8;
            expected.push(suffix[c]);
        }
        assert_eq!(expected.len(), 4095 - clear);
        check_emission(clear, 4095, &prefix, &suffix, &expected);
    }
}

#[test]
fn branching_chains_match_independent_forward_strings() {
    let clear = 4;
    let mut prefix = vec![u16::MAX; 4096];
    let mut suffix = vec![0xee; 4096];
    let mut strings = vec![Vec::new(); 4096];
    for (c, string) in strings[..clear].iter_mut().enumerate() { *string = vec![c as u8]; }
    let mut rng = 0x9137_abcd_u64;
    for c in clear + 2..4096 {
        rng = rng.wrapping_mul(6364136223846793005).wrapping_add(1);
        let raw = (rng >> 32) as usize % (c - 2);
        let parent = if raw < clear { raw } else { raw + 2 };
        prefix[c] = parent as u16;
        suffix[c] = (rng >> 24) as u8;
        strings[c] = strings[parent].clone(); strings[c].push(suffix[c]);
        if c % 17 == 0 || c == clear + 2 || c == 4095 {
            check_emission(clear, c, &prefix, &suffix, &strings[c]);
        }
    }
}

#[test]
fn unchecked_high_literal_uses_explicit_modulo() {
    let prefix = vec![u16::MAX; 4096];
    let suffix = vec![0xee; 4096];
    check_emission(2048, 300, &prefix, &suffix, &[44]);
}
