//! Differential test of icns_verus vs icns_creusot (and a dump for the Python oracle).
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::panic::{catch_unwind, AssertUnwindSafe};

// ---------------------------------------------------------------- canonical form

#[derive(Clone, Debug, PartialEq, Eq)]
struct Ent {
    type_hex: String,
    payload: [u32; 2],
    opaque: bool,
    nominal: Option<u32>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Res {
    Err(String),
    Parsed(Vec<Ent>),
    Pix { w: u32, h: u32, data: Vec<u8> },
    NoEntries, // parse failed, so decoders could not be called
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Key {
    Rgb(u32),
    Mask(u32),
    Comb(Option<u32>, Option<u32>),
}

fn key_str(k: &Key) -> String {
    let o = |x: &Option<u32>| x.map_or("-".to_string(), |v| v.to_string());
    match k {
        Key::Rgb(i) => format!("rgb {i}"),
        Key::Mask(i) => format!("mask {i}"),
        Key::Comb(c, m) => format!("comb {} {}", o(c), o(m)),
    }
}

fn fnv(d: &[u8]) -> u64 {
    let mut h = 0xcbf29ce484222325u64;
    for &b in d {
        h ^= b as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn render(r: &Res) -> String {
    match r {
        Res::Err(n) => format!("ERR:{n}"),
        Res::NoEntries => "NOENTRIES".into(),
        Res::Pix { w, h, data } => format!("OK:{w},{h},{},{:016x}", data.len(), fnv(data)),
        Res::Parsed(es) => {
            let mut s = String::from("OK:");
            for e in es {
                let _ = write!(
                    s,
                    "{}:{}-{}:{}:{};",
                    e.type_hex,
                    e.payload[0],
                    e.payload[1],
                    e.opaque as u8,
                    e.nominal.map_or("-".to_string(), |v| v.to_string())
                );
            }
            s
        }
    }
}

/// Coarse class used to group divergences.
fn class(r: &Res) -> String {
    match r {
        Res::Err(n) => n.clone(),
        Res::NoEntries => "NOENTRIES".into(),
        Res::Pix { .. } => "OK".into(),
        Res::Parsed(_) => "OKparse".into(),
    }
}

trait Api {
    type E;
    fn name() -> &'static str;
    fn parse(b: &[u8]) -> Result<Vec<Self::E>, String>;
    fn ent(e: &Self::E) -> Ent;
    fn rgb(b: &[u8], es: &[Self::E], i: u32) -> Result<(u32, u32, Vec<u8>), String>;
    fn mask(b: &[u8], es: &[Self::E], i: u32) -> Result<(u32, u32, Vec<u8>), String>;
    fn comb(b: &[u8], es: &[Self::E], c: Option<u32>, m: Option<u32>) -> Result<(u32, u32, Vec<u8>), String>;
}

struct V;
struct C;

impl Api for V {
    type E = icns_verus::Entry;
    fn name() -> &'static str { "verus" }
    fn parse(b: &[u8]) -> Result<Vec<Self::E>, String> {
        icns_verus::parse(b).map_err(|e| e.name().to_string())
    }
    fn ent(e: &Self::E) -> Ent {
        Ent { type_hex: e.type_hex.clone(), payload: e.payload, opaque: e.opaque_candidate, nominal: e.nominal_side }
    }
    fn rgb(b: &[u8], es: &[Self::E], i: u32) -> Result<(u32, u32, Vec<u8>), String> {
        icns_verus::decode_rgb(b, es, i).map(|d| (d.width, d.height, d.data)).map_err(|e| e.name().to_string())
    }
    fn mask(b: &[u8], es: &[Self::E], i: u32) -> Result<(u32, u32, Vec<u8>), String> {
        icns_verus::decode_mask(b, es, i).map(|d| (d.width, d.height, d.data)).map_err(|e| e.name().to_string())
    }
    fn comb(b: &[u8], es: &[Self::E], c: Option<u32>, m: Option<u32>) -> Result<(u32, u32, Vec<u8>), String> {
        icns_verus::combine(b, es, c, m).map(|d| (d.width, d.height, d.data)).map_err(|e| e.name().to_string())
    }
}

impl Api for C {
    type E = icns_creusot::Entry;
    fn name() -> &'static str { "creusot" }
    fn parse(b: &[u8]) -> Result<Vec<Self::E>, String> {
        icns_creusot::parse(b).map_err(|e| {
            let n = e.name().to_string();
            if std::env::var("FAULT").is_ok() && n == "EntryOutOfBounds" { "TruncatedEntryHeader".to_string() } else { n }
        })
    }
    fn ent(e: &Self::E) -> Ent {
        Ent { type_hex: e.type_hex.clone(), payload: e.payload, opaque: e.opaque_candidate, nominal: e.nominal_side }
    }
    fn rgb(b: &[u8], es: &[Self::E], i: u32) -> Result<(u32, u32, Vec<u8>), String> {
        icns_creusot::decode_rgb(b, es, i).map_err(|e| e.name().to_string())
    }
    fn mask(b: &[u8], es: &[Self::E], i: u32) -> Result<(u32, u32, Vec<u8>), String> {
        icns_creusot::decode_mask(b, es, i).map_err(|e| e.name().to_string())
    }
    fn comb(b: &[u8], es: &[Self::E], c: Option<u32>, m: Option<u32>) -> Result<(u32, u32, Vec<u8>), String> {
        icns_creusot::combine(b, es, c, m).map_err(|e| e.name().to_string())
    }
}

fn pix(r: Result<(u32, u32, Vec<u8>), String>) -> Res {
    match r {
        Ok((w, h, data)) => Res::Pix { w, h, data },
        Err(e) => Res::Err(e),
    }
}

fn guard<F: FnOnce() -> Res>(f: F) -> Res {
    match catch_unwind(AssertUnwindSafe(f)) {
        Ok(r) => r,
        Err(_) => Res::Err("PANIC".into()),
    }
}

fn parse_res<A: Api>(b: &[u8]) -> (Res, Option<Vec<A::E>>) {
    match catch_unwind(AssertUnwindSafe(|| A::parse(b))) {
        Ok(Ok(es)) => (Res::Parsed(es.iter().map(A::ent).collect()), Some(es)),
        Ok(Err(e)) => (Res::Err(e), None),
        Err(_) => (Res::Err("PANIC".into()), None),
    }
}

fn eval_key<A: Api>(b: &[u8], es: &Option<Vec<A::E>>, k: &Key) -> Res {
    let Some(es) = es else { return Res::NoEntries };
    guard(|| match *k {
        Key::Rgb(i) => pix(A::rgb(b, es, i)),
        Key::Mask(i) => pix(A::mask(b, es, i)),
        Key::Comb(c, m) => pix(A::comb(b, es, c, m)),
    })
}

fn keys_for(es: &[Ent]) -> Vec<Key> {
    let n = es.len() as u32;
    let mut single: BTreeSet<u32> = BTreeSet::new();
    if n <= 12 {
        single.extend(0..n);
    } else {
        single.extend(0..6);
        single.extend(n - 3..n);
    }
    single.extend([n, n.wrapping_add(1), u32::MAX]);
    let mut keys: BTreeSet<Key> = BTreeSet::new();
    for &i in &single {
        keys.insert(Key::Rgb(i));
        keys.insert(Key::Mask(i));
    }
    let mut base: BTreeSet<u32> = (0..n.min(4)).collect();
    base.extend([n, u32::MAX]);
    let mut cs: Vec<Option<u32>> = vec![None];
    cs.extend(base.iter().map(|&x| Some(x)));
    for &c in &cs {
        for &m in &cs {
            keys.insert(Key::Comb(c, m));
        }
    }
    let colour = ["69733332", "696c3332", "69683332", "69743332"];
    let maskt = ["73386d6b", "6c386d6b", "68386d6b", "74386d6b"];
    let ci: Vec<u32> = es.iter().enumerate().filter(|(_, e)| colour.contains(&e.type_hex.as_str())).map(|(i, _)| i as u32).take(3).collect();
    let mi: Vec<u32> = es.iter().enumerate().filter(|(_, e)| maskt.contains(&e.type_hex.as_str())).map(|(i, _)| i as u32).take(4).collect();
    for &c in &ci {
        for &m in &mi {
            keys.insert(Key::Comb(Some(c), Some(m)));
        }
    }
    keys.into_iter().collect()
}

struct Outcome {
    keys: Vec<Key>,
    a: Vec<Res>, // parse first, then keys
    b: Vec<Res>,
}

fn evaluate(bytes: &[u8]) -> Outcome {
    let (pa, ea) = parse_res::<V>(bytes);
    let (pb, eb) = parse_res::<C>(bytes);
    let src = match (&pa, &pb) {
        (Res::Parsed(x), _) => Some(x.clone()),
        (_, Res::Parsed(y)) => Some(y.clone()),
        _ => None,
    };
    let keys = src.map(|e| keys_for(&e)).unwrap_or_default();
    let mut a = vec![pa];
    let mut b = vec![pb];
    for k in &keys {
        a.push(eval_key::<V>(bytes, &ea, k));
        b.push(eval_key::<C>(bytes, &eb, k));
    }
    Outcome { keys, a, b }
}

/// Fast path: just report whether anything differs.
fn quick_differs(bytes: &[u8]) -> bool {
    let (pa, ea) = parse_res::<V>(bytes);
    let (pb, eb) = parse_res::<C>(bytes);
    if pa != pb {
        return true;
    }
    let Res::Parsed(es) = &pa else { return false };
    for k in keys_for(es) {
        if eval_key::<V>(bytes, &ea, &k) != eval_key::<C>(bytes, &eb, &k) {
            return true;
        }
    }
    false
}

fn kind_of(k: &Key) -> &'static str {
    match k {
        Key::Rgb(_) => "rgb",
        Key::Mask(_) => "mask",
        Key::Comb(..) => "comb",
    }
}

/// Set of divergence signatures (op, class A, class B) for a case.
fn signatures(bytes: &[u8]) -> BTreeSet<(String, String, String)> {
    let o = evaluate(bytes);
    let mut s = BTreeSet::new();
    if o.a[0] != o.b[0] {
        s.insert(("parse".into(), class(&o.a[0]), class(&o.b[0])));
    }
    for (i, k) in o.keys.iter().enumerate() {
        if o.a[i + 1] != o.b[i + 1] {
            s.insert((kind_of(k).into(), class(&o.a[i + 1]), class(&o.b[i + 1])));
        }
    }
    s
}

// ---------------------------------------------------------------- rng

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545F4914F6CDD1D)
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn range(&mut self, lo: usize, hi: usize) -> usize {
        lo + self.below(hi - lo + 1)
    }
    fn chance(&mut self, pct: usize) -> bool {
        self.below(100) < pct
    }
    fn byte(&mut self) -> u8 {
        (self.next() >> 24) as u8
    }
    fn pick<T: Copy>(&mut self, s: &[T]) -> T {
        s[self.below(s.len())]
    }
}

// ---------------------------------------------------------------- builders

const COLOUR: [(&[u8; 4], usize); 4] = [(b"is32", 16), (b"il32", 32), (b"ih32", 48), (b"it32", 128)];
const MASKT: [(&[u8; 4], usize); 4] = [(b"s8mk", 16), (b"l8mk", 32), (b"h8mk", 48), (b"t8mk", 128)];
const OPAQUE: [&[u8; 4]; 14] = [b"icp4", b"icp5", b"icp6", b"ic07", b"ic08", b"ic09", b"ic10", b"ic11", b"ic12", b"ic13", b"ic14", b"icsB", b"sb24", b"SB24"];
const OLD: [&[u8; 4]; 17] = [b"ICON", b"ICN#", b"ics#", b"ics4", b"ics8", b"icl4", b"icl8", b"ich#", b"ich4", b"ich8", b"icm#", b"icm4", b"icm8", b"TOC ", b"icnV", b"name", b"info"];
const PNG: [u8; 8] = [0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a];
const JP2: [u8; 12] = [0, 0, 0, 0x0c, 0x6a, 0x50, 0x20, 0x20, 0x0d, 0x0a, 0x87, 0x0a];

fn gen_channel(r: &mut Rng, n: usize, out: &mut Vec<u8>, overrun_at_end: bool) {
    let mut produced = 0;
    while produced < n {
        let rem = n - produced;
        if overrun_at_end && rem <= 130 && rem >= 1 && r.chance(50) {
            // control with count > rem, if representable
            let want = rem + r.range(1, 3);
            if want >= 3 && want <= 130 {
                out.push((want + 125) as u8);
                out.push(r.byte());
            } else if want <= 128 {
                out.push((want - 1) as u8);
                for _ in 0..want {
                    out.push(r.byte());
                }
            }
            return;
        }
        let lit = rem < 3 || r.chance(55);
        if lit {
            let c = if r.chance(20) { rem.min(128) } else { r.range(1, rem.min(128)) };
            out.push((c - 1) as u8);
            let flat = r.chance(30);
            let v = r.byte();
            for _ in 0..c {
                out.push(if flat { v } else { r.byte() });
            }
            produced += c;
        } else {
            let c = if r.chance(20) { rem.min(130) } else { r.range(3, rem.min(130)) };
            out.push((c + 125) as u8);
            out.push(r.byte());
            produced += c;
        }
    }
}

fn gen_rle(r: &mut Rng, side: usize, ty: &[u8; 4]) -> Vec<u8> {
    let n = side * side;
    let mut p = Vec::new();
    let defect = r.below(14);
    let over = defect == 2;
    for ch in 0..3 {
        let ov = over && ch == 2 && r.chance(60);
        gen_channel(r, n, &mut p, ov);
    }
    match defect {
        1 => {
            let cut = r.below(p.len() + 1);
            p.truncate(cut);
        }
        2 => {
            // guarantee at least one overrun: append an oversize run to a fresh channel set
            if r.chance(50) {
                p.clear();
                gen_channel(r, n, &mut p, true);
            }
        }
        3 => {
            for _ in 0..r.range(1, 3) {
                p.push(r.byte());
            }
        }
        4 => p.clear(),
        5 => {
            // drop the last channel
            let mut q = Vec::new();
            gen_channel(r, n, &mut q, false);
            gen_channel(r, n, &mut q, false);
            p = q;
        }
        6 => {
            let l = p.len();
            p.truncate(l.saturating_sub(1));
        }
        7 => {
            // ends with a repeat control lacking its data byte
            let l = p.len();
            p.truncate(l.saturating_sub(2));
            p.push(0x80);
        }
        8 => {
            // starts with two single-byte zero literals (ambiguous with it32 prefix)
            let mut q = vec![0, 0, 0, 0];
            gen_channel(r, n - 2, &mut q, false);
            gen_channel(r, n, &mut q, false);
            gen_channel(r, n, &mut q, false);
            p = q;
        }
        9 => {
            // single huge control values
            p = vec![0xff, 0x00, 0x80, 0x01, 0x7f];
            for _ in 0..r.below(8) {
                p.push(r.pick(&[0u8, 0x7f, 0x80, 0xff]));
            }
        }
        _ => {}
    }
    if ty == b"it32" {
        match r.below(8) {
            0 | 1 | 2 | 3 => {
                let mut q = vec![0, 0, 0, 0];
                q.extend(&p);
                p = q;
            }
            4 => {
                let mut q = vec![0, 0, 0, 1 + r.below(3) as u8];
                q.extend(&p);
                p = q;
            }
            5 => {
                let mut q = vec![0u8; 4];
                q.extend(vec![0u8; 4 * r.below(3)]);
                q.extend(&p);
                p = q;
            }
            6 => p = vec![0; r.below(9)],
            _ => {}
        }
    }
    p
}

fn gen_payload(r: &mut Rng, ty: &[u8; 4]) -> Vec<u8> {
    if let Some(&(_, side)) = COLOUR.iter().find(|(t, _)| *t == ty) {
        return gen_rle(r, side, ty);
    }
    if let Some(&(_, side)) = MASKT.iter().find(|(t, _)| *t == ty) {
        let n = side * side;
        let len = match r.below(10) {
            0 => n - 1,
            1 => n + 1,
            2 => 0,
            3 => r.below(n * 2),
            _ => n,
        };
        let flat = r.chance(30);
        return (0..len).map(|_| if flat { 0xaa } else { r.byte() }).collect();
    }
    // opaque / unknown / old
    let mut p = Vec::new();
    match r.below(8) {
        0 => p.extend(PNG),
        1 => p.extend(JP2),
        2 => {
            let k = r.below(8);
            p.extend(&PNG[..k]);
        }
        3 => {
            let k = r.below(12);
            p.extend(&JP2[..k]);
        }
        4 => {
            p.extend(PNG);
            p[r.below(8)] ^= 1 << r.below(8);
        }
        5 => {
            p.extend(JP2);
            p[r.below(12)] ^= 1 << r.below(8);
        }
        _ => {}
    }
    for _ in 0..r.below(20) {
        p.push(r.byte());
    }
    p
}

fn gen_type(r: &mut Rng) -> [u8; 4] {
    match r.below(12) {
        0 | 1 | 2 | 3 => *COLOUR[r.below(4)].0,
        4 | 5 | 6 => *MASKT[r.below(4)].0,
        7 => *OPAQUE[r.below(14)],
        8 => *OLD[r.below(17)],
        9 => *r.pick(&[b"ic04", b"ic05", b"icsb", b"ICNS", b"icns", b"ic1\x30", b"ic15", b"is3\x32"]),
        10 => [r.byte(), r.byte(), r.byte(), r.byte()],
        _ => [0, 0, 0, 0],
    }
}

/// Returns file bytes and the offsets of the entry headers.
fn build(entries: &[([u8; 4], Vec<u8>)]) -> (Vec<u8>, Vec<usize>) {
    let mut f = b"icns".to_vec();
    f.extend([0; 4]);
    let mut offs = vec![];
    for (t, p) in entries {
        offs.push(f.len());
        f.extend(t);
        f.extend(((p.len() + 8) as u32).to_be_bytes());
        f.extend(p);
    }
    let l = f.len() as u32;
    f[4..8].copy_from_slice(&l.to_be_bytes());
    (f, offs)
}

fn gen_structured(r: &mut Rng) -> (Vec<u8>, Vec<usize>) {
    let mut es: Vec<([u8; 4], Vec<u8>)> = vec![];
    let mode = r.below(10);
    if mode < 4 {
        // a plausible icon: pair(s)
        for _ in 0..r.range(1, 2) {
            let s = if r.chance(85) { r.below(3) } else { 3 };
            let (c, m) = (*COLOUR[s].0, *MASKT[s].0);
            let (c, m) = if r.chance(10) { (*COLOUR[r.below(4)].0, *MASKT[r.below(4)].0) } else { (c, m) };
            let pc = gen_payload(r, &c);
            let pm = gen_payload(r, &m);
            if r.chance(70) {
                es.push((c, pc));
                es.push((m, pm));
            } else {
                es.push((m, pm));
                es.push((c, pc));
            }
        }
    }
    for _ in 0..r.range(0, 5) {
        let mut t = gen_type(r);
        if t == *b"it32" && r.chance(60) {
            t = *b"il32";
        }
        if t == *b"t8mk" && r.chance(60) {
            t = *b"h8mk";
        }
        let p = gen_payload(r, &t);
        es.push((t, p));
    }
    if r.chance(15) && !es.is_empty() {
        let e = es[r.below(es.len())].clone();
        es.push(e);
    }
    if r.chance(40) && es.len() > 1 {
        for i in (1..es.len()).rev() {
            es.swap(i, r.below(i + 1));
        }
    }
    build(&es)
}

// ---------------------------------------------------------------- mutations

const EXT32: [u32; 12] = [0, 1, 7, 8, 9, 15, 16, 0x7fff_ffff, 0x8000_0000, 0xffff_fff8, 0xffff_ffff, 0x0400_0000];

fn set_len_field(b: &mut [u8], at: usize, v: u32) {
    if at + 8 <= b.len() {
        b[at + 4..at + 8].copy_from_slice(&v.to_be_bytes());
    }
}
fn get_len_field(b: &[u8], at: usize) -> Option<u32> {
    (at + 8 <= b.len()).then(|| u32::from_be_bytes(b[at + 4..at + 8].try_into().unwrap()))
}
fn fix_file_len(b: &mut Vec<u8>) {
    if b.len() >= 8 {
        let l = b.len() as u32;
        b[4..8].copy_from_slice(&l.to_be_bytes());
    }
}

fn mutations(r: &mut Rng, base: &[u8], offs: &[usize], out: &mut Vec<Vec<u8>>) {
    let len = base.len();
    if len == 0 {
        for k in 1..=3usize {
            out.push(vec![0u8; k]);
        }
        return;
    }
    // file length field
    for d in [-1i64, 0, 1, 8] {
        let mut m = base.to_vec();
        set_len_field(&mut m, 0, (len as i64 + d) as u32);
        out.push(m);
    }
    for &v in &EXT32 {
        let mut m = base.to_vec();
        set_len_field(&mut m, 0, v);
        out.push(m);
    }
    // entry length fields
    for &o in offs.iter().take(8) {
        let Some(orig) = get_len_field(base, o) else { continue };
        let mut vals: Vec<u32> = [-1i64, 0, 1, 8, -8].iter().map(|d| (orig as i64 + d) as u32).collect();
        vals.extend(EXT32);
        for v in vals {
            let mut m = base.to_vec();
            set_len_field(&mut m, o, v);
            out.push(m.clone());
            // also with the file length unchanged vs. adjusted for growth
            if (v as usize) > 8 && (v as usize) < 200 {
                let mut m2 = m;
                m2.truncate(len.min(m2.len()));
                out.push(m2);
            }
        }
    }
    // truncation
    let cuts: Vec<usize> = if len <= 400 {
        (0..len).collect()
    } else {
        let mut c: BTreeSet<usize> = (0..24).collect();
        for &o in offs {
            for d in 0..10usize {
                c.insert(o + d);
                c.insert((o + d).saturating_sub(5));
            }
        }
        for _ in 0..40 {
            c.insert(r.below(len));
        }
        c.into_iter().filter(|&x| x < len).collect()
    };
    for &c in &cuts {
        let mut m = base[..c].to_vec();
        out.push(m.clone());
        fix_file_len(&mut m);
        out.push(m);
    }
    // single byte changes: header + entry headers exhaustively, random elsewhere
    let mut pos: BTreeSet<usize> = (0..len.min(16)).collect();
    for &o in offs.iter().take(8) {
        pos.extend(o..(o + 8).min(len));
    }
    for _ in 0..40 {
        pos.insert(r.below(len));
    }
    for &p in &pos {
        for v in [0u8, 1, 0x7f, 0x80, 0x81, 0xff, r.byte(), base[p].wrapping_add(1), base[p].wrapping_sub(1)] {
            if v != base[p] {
                let mut m = base.to_vec();
                m[p] = v;
                out.push(m);
            }
        }
        let mut m = base.to_vec();
        m[p] ^= 1 << r.below(8);
        out.push(m);
    }
    for p in 0..len.min(24) {
        for bit in 0..8 {
            let mut m = base.to_vec();
            m[p] ^= 1 << bit;
            out.push(m);
        }
    }
    // insert / delete a byte (raw and with file length fixed)
    for _ in 0..8 {
        let p = r.below(len + 1);
        let mut m = base.to_vec();
        m.insert(p, r.byte());
        out.push(m.clone());
        fix_file_len(&mut m);
        out.push(m);
        if p < len {
            let mut m = base.to_vec();
            m.remove(p);
            out.push(m.clone());
            fix_file_len(&mut m);
            out.push(m);
        }
    }
    // trailing bytes
    for k in [1usize, 7, 8, 9] {
        let mut m = base.to_vec();
        m.extend(vec![0u8; k]);
        out.push(m.clone());
        fix_file_len(&mut m);
        out.push(m);
    }
}

// ---------------------------------------------------------------- minimiser

fn entry_walk(b: &[u8]) -> Vec<(usize, usize)> {
    // tolerant walk of (offset, length) pairs
    let mut v = vec![];
    let mut o = 8usize;
    while o + 8 <= b.len() && v.len() < 5000 {
        let l = u32::from_be_bytes(b[o + 4..o + 8].try_into().unwrap()) as usize;
        v.push((o, l));
        if l < 8 || o + l > b.len() {
            break;
        }
        o += l;
    }
    v
}

fn delete_fixed(b: &[u8], a: usize, e: usize) -> Vec<u8> {
    let mut m = b.to_vec();
    let d = (e - a) as u32;
    let walk = entry_walk(b);
    m.drain(a..e);
    // adjust the entry whose payload fully contains the range
    for (o, l) in walk {
        if a >= o + 8 && e <= o + l {
            let nl = (l as u32).wrapping_sub(d);
            if o + 8 <= m.len() {
                m[o + 4..o + 8].copy_from_slice(&nl.to_be_bytes());
            }
            break;
        }
    }
    fix_file_len(&mut m);
    m
}

fn minimise(input: &[u8], pred: &dyn Fn(&[u8]) -> bool) -> Vec<u8> {
    let mut cur = input.to_vec();
    let mut changed = true;
    let mut guard = 0;
    while changed && guard < 50 {
        guard += 1;
        changed = false;
        let mut chunk = (cur.len() / 2).max(1);
        loop {
            let mut i = 0;
            while i < cur.len() {
                let e = (i + chunk).min(cur.len());
                let mut cands: Vec<Vec<u8>> = vec![];
                cands.push(delete_fixed(&cur, i, e));
                let mut raw = cur.clone();
                raw.drain(i..e);
                cands.push(raw);
                let mut hit = false;
                for c in cands {
                    if c.len() < cur.len() && pred(&c) {
                        cur = c;
                        hit = true;
                        changed = true;
                        break;
                    }
                }
                if !hit {
                    i += chunk;
                }
            }
            if chunk == 1 {
                break;
            }
            chunk = (chunk / 2).max(1);
        }
        // simplify bytes to zero
        for i in 0..cur.len() {
            if cur[i] != 0 {
                let mut c = cur.clone();
                c[i] = 0;
                if pred(&c) {
                    cur = c;
                    changed = true;
                }
            }
        }
    }
    cur
}

// ---------------------------------------------------------------- driver

fn hex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

struct Stats {
    total: BTreeMap<&'static str, u64>,
    div_cases: BTreeMap<&'static str, u64>,
    // signature -> (count, smallest case)
    sigs: BTreeMap<(String, String, String), (u64, Vec<u8>)>,
    sample: Vec<(&'static str, Vec<u8>)>,
}

impl Stats {
    fn record(&mut self, kind: &'static str, b: &[u8], sample_every: u64) {
        let n = self.total.entry(kind).or_insert(0);
        *n += 1;
        let nn = *n;
        if quick_differs(b) {
            *self.div_cases.entry(kind).or_insert(0) += 1;
            for s in signatures(b) {
                let e = self.sigs.entry(s).or_insert((0, b.to_vec()));
                e.0 += 1;
                if b.len() < e.1.len() {
                    e.1 = b.to_vec();
                }
            }
        }
        if sample_every == 0 && kind == "b" && std::env::var("SPECIAL").is_ok() {
            let (pa, _) = parse_res::<V>(b);
            let (pb, _) = parse_res::<C>(b);
            let r = |x: &Res| { let t = render(x); if t.len() > 60 { format!("{}...(entries={})", &t[..60], t.matches(';').count()) } else { t } };
            if b.len() > 100_000 {
                println!("SPECIAL len={} declared={} V={} C={}", b.len(), if b.len() >= 8 { u32::from_be_bytes(b[4..8].try_into().unwrap()) } else { 0 }, r(&pa), r(&pb));
            } else {
                println!("SPECIALSMALL {} V={} C={}", hex(b), render(&pa).chars().take(40).collect::<String>(), render(&pb).chars().take(40).collect::<String>());
            }
        }
        if sample_every > 0 && nn % sample_every == 0 && b.len() <= 300_000 {
            self.sample.push((kind, b.to_vec()));
        }
    }
}

fn header(magic: &[u8; 4], len: u32) -> Vec<u8> {
    let mut v = magic.to_vec();
    v.extend(len.to_be_bytes());
    v
}

fn kind_a(st: &mut Stats) {
    // all byte strings of length 0..=3
    st.record("a", &[], 1);
    for x in 0..256u32 {
        st.record("a", &[x as u8], 1);
    }
    for x in 0..65536u32 {
        st.record("a", &[(x >> 8) as u8, x as u8], 1);
    }
    for x in 0..(1u32 << 24) {
        st.record("a", &[(x >> 16) as u8, (x >> 8) as u8, x as u8], 4001);
    }
    // 8-byte headers + 0..=16 trailing bytes
    let magics: [&[u8; 4]; 9] = [b"icns", b"ICNS", b"icn\0", b"icnt", b"\0\0\0\0", b"\xff\xff\xff\xff", b"snci", b"Icns", b"icns"];
    let mut lens: Vec<u32> = (0..=40).collect();
    lens.extend([0x7fff_ffff, 0x8000_0000, 0xffff_fff8, 0xffff_ffff, 0x0400_0000, 0x0400_0001, 0x0400_0008, 0x1_0000 - 1, 0x1_0000, 0x100_0000]);
    for m in magics {
        for &l in &lens {
            for extra in 0..=24usize {
                let mut v = header(m, l);
                v.extend((0..extra).map(|i| i as u8));
                st.record("a", &v, 3);
                let mut v = header(m, l);
                v.extend(vec![0u8; extra]);
                st.record("a", &v, 7);
            }
        }
    }
    // header + one entry header with boundary types and lengths (file length consistent)
    let types: [[u8; 4]; 8] = [*b"is32", *b"s8mk", *b"it32", *b"t8mk", *b"icp4", *b"ic04", [0; 4], [0xff; 4]];
    let elens: Vec<u32> = (0..=40).chain([0x7fff_ffff, 0x8000_0000, 0xffff_fff0, 0xffff_fff8, 0xffff_ffff, 0x0400_0000, 0xffff_ff00]).collect();
    for t in types {
        for &el in &elens {
            for extra in [0usize, 1, 7, 8, 9, 20, 300] {
                for file_delta in [0i64, -1, 1, 8] {
                    let mut v = header(b"icns", 0);
                    v.extend(t);
                    v.extend(el.to_be_bytes());
                    v.extend(vec![0u8; extra]);
                    let l = (v.len() as i64 + file_delta) as u32;
                    v[4..8].copy_from_slice(&l.to_be_bytes());
                    st.record("a", &v, 3);
                }
            }
        }
    }
    // short entries area: 0..=15 bytes after header, with file length matching
    for extra in 0..=15usize {
        for fill in [0u8, 0xff, 8] {
            let mut v = header(b"icns", (8 + extra) as u32);
            v.extend(vec![fill; extra]);
            st.record("a", &v, 3);
        }
    }
}

fn kind_special(st: &mut Stats, r: &mut Rng) {
    // size limit boundary
    for (len, declared) in [(67_108_864usize, 67_108_864u32), (67_108_864, 8), (67_108_865, 67_108_865), (67_108_865, 8), (67_108_863, 67_108_864), (67_108_863, 67_108_863)] {
        let mut v = vec![0u8; len];
        v[..4].copy_from_slice(b"icns");
        v[4..8].copy_from_slice(&declared.to_be_bytes());
        if len == 67_108_864 && declared == 67_108_864 {
            // a legal container of one huge unknown entry
            v[8..12].copy_from_slice(b"ZZZZ");
            v[12..16].copy_from_slice(&(67_108_856u32).to_be_bytes());
        }
        st.record("b", &v, 0);
        // non-icns big input (R2 before magic)
        let mut w = vec![0x41u8; len];
        w[0] = 0;
        st.record("b", &w, 0);
    }
    // entry count boundaries
    for count in [4094usize, 4095, 4096, 4097, 4098] {
        for tail in 0..4 {
            let mut es: Vec<([u8; 4], Vec<u8>)> = (0..count).map(|i| (*b"ZZZZ", vec![(i & 255) as u8; i % 3])).collect();
            let (mut f, _) = build(&es);
            match tail {
                1 => {
                    // malformed entry after the boundary
                    f.extend(b"ZZZZ\0\0\0\x03");
                    fix_file_len(&mut f);
                }
                2 => {
                    f.extend(b"ZZZZ\xff\xff\xff\xff");
                    fix_file_len(&mut f);
                }
                3 => {
                    f.extend(b"ZZ");
                    fix_file_len(&mut f);
                }
                _ => {}
            }
            es.clear();
            st.record("b", &f, 0);
            // 4097th entry with oversize length / 4096 + short header
            let mut g = f.clone();
            let n = g.len();
            if n > 12 {
                g[n - 4..].copy_from_slice(&0u32.to_be_bytes());
                st.record("b", &g, 0);
            }
        }
    }
    // targeted: R8 types carrying opaque signatures, 4N-byte payloads, R11 prefix edge cases
    let mut payloads: Vec<Vec<u8>> = vec![PNG.to_vec(), JP2.to_vec(), [&PNG[..], &[1, 2, 3][..]].concat(), [&JP2[..], &[9u8; 5][..]].concat(), PNG[..7].to_vec(), JP2[..11].to_vec(), vec![], vec![0; 4], vec![0; 3], vec![0; 5], vec![0; 8], vec![0, 0, 0, 0, 0xff, 0], vec![0, 0, 0, 1]];
    let mut all_types: Vec<[u8; 4]> = COLOUR.iter().map(|x| *x.0).chain(MASKT.iter().map(|x| *x.0)).chain(OPAQUE.iter().map(|x| **x)).collect();
    for t in [b"ic04", b"ic05", b"icsb", b"ZZZZ", b"ICON", b"ic15"] {
        all_types.push(*t);
    }
    for t in &all_types {
        for p in &payloads {
            let (f, _) = build(&[(*t, p.clone())]);
            st.record("b", &f, 1);
        }
    }
    for (t, side) in COLOUR {
        let n = side * side;
        for len in [4 * n, 4 * n + 4, 3 * n, 3 * n + 1, n] {
            payloads.push(vec![0u8; len]);
            let (f, _) = build(&[(*t, vec![0u8; len])]);
            st.record("b", &f, 1);
            let (f, _) = build(&[(*t, (0..len).map(|i| (i % 7) as u8 * 0x20).collect())]);
            st.record("b", &f, 1);
        }
    }
    // many pairs for index combinations
    let mut es = vec![];
    for i in 0..4 {
        es.push((*COLOUR[i].0, gen_rle_valid(r, COLOUR[i].1, COLOUR[i].0)));
        es.push((*MASKT[i].0, vec![0x55; MASKT[i].1 * MASKT[i].1]));
    }
    let (f, offs) = build(&es);
    st.record("b", &f, 1);
    let mut muts = vec![];
    mutations(r, &f, &offs, &mut muts);
    for m in muts {
        st.record("c", &m, 0);
    }
}

fn gen_rle_valid(r: &mut Rng, side: usize, ty: &[u8; 4]) -> Vec<u8> {
    let n = side * side;
    let mut p = vec![];
    if ty == b"it32" {
        p.extend([0u8; 4]);
    }
    for _ in 0..3 {
        gen_channel(r, n, &mut p, false);
    }
    p
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let seed: u64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(0x1C45_D1FF_2026_1107);
    let nstruct: usize = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(30_000);
    let nmutbase: usize = args.get(3).and_then(|s| s.parse().ok()).unwrap_or(250);
    std::panic::set_hook(Box::new(|_| {}));
    println!("seed = {seed} (0x{seed:x}); structured={nstruct} mutation_bases={nmutbase}");
    let mut r = Rng(seed);
    let mut st = Stats { total: BTreeMap::new(), div_cases: BTreeMap::new(), sigs: BTreeMap::new(), sample: vec![] };
    let t0 = std::time::Instant::now();

    if std::env::var("SPECIAL").is_ok() {
        kind_special(&mut st, &mut r);
        return;
    }
    kind_a(&mut st);
    eprintln!("kind a done {:?}", t0.elapsed());
    kind_special(&mut st, &mut r);
    eprintln!("special done {:?}", t0.elapsed());

    // (b) structured
    let mut bases: Vec<(Vec<u8>, Vec<usize>)> = vec![];
    for i in 0..nstruct {
        let (f, offs) = gen_structured(&mut r);
        st.record("b", &f, 8);
        if i < nmutbase {
            bases.push((f, offs));
        }
    }
    eprintln!("kind b done {:?}", t0.elapsed());

    // (c) mutations of structured
    for (f, offs) in &bases {
        let mut muts = vec![];
        mutations(&mut r, f, offs, &mut muts);
        for m in muts {
            st.record("c", &m, 40);
        }
    }
    eprintln!("kind c done {:?}", t0.elapsed());

    // (d) vectors and their mutations
    let vpath = "../../icns/spec-frozen/VECTORS.json";
    let vs: serde_json::Value = serde_json::from_str(&std::fs::read_to_string(vpath).unwrap()).unwrap();
    let mut nvec = 0;
    for v in vs.as_array().unwrap() {
        let h = v["input_hex"].as_str().unwrap();
        let rep = v.get("input_repeat").and_then(|x| x.as_u64()).unwrap_or(1) as usize;
        let one: Vec<u8> = (0..h.len() / 2).map(|i| u8::from_str_radix(&h[2 * i..2 * i + 2], 16).unwrap()).collect();
        let mut data = Vec::with_capacity(one.len() * rep);
        for _ in 0..rep {
            data.extend(&one);
        }
        nvec += 1;
        st.record("d", &data, 1);
        if data.len() > 200_000 {
            continue;
        }
        let offs: Vec<usize> = entry_walk(&data).into_iter().map(|x| x.0).collect();
        let mut muts = vec![];
        mutations(&mut r, &data, &offs, &mut muts);
        for m in muts {
            st.record("d", &m, 60);
        }
    }
    eprintln!("kind d done {:?} ({nvec} vectors)", t0.elapsed());

    // report
    println!("\nTOTALS");
    let mut grand = 0;
    for (k, v) in &st.total {
        println!("  kind {k}: {v} cases, {} with Rust-vs-Rust divergence", st.div_cases.get(k).unwrap_or(&0));
        grand += v;
    }
    println!("  total {grand}");
    println!("  oracle sample size {}", st.sample.len());
    println!("\nDIVERGENCE SIGNATURES (op, verus, creusot): count");
    let mut out = String::new();
    for (sig, (cnt, case)) in &st.sigs {
        println!("  {:?}: {cnt}, first-smallest len {}", sig, case.len());
        let sg = sig.clone();
        let pred = move |b: &[u8]| signatures(b).contains(&sg);
        let min = if case.len() > 200_000 { case.clone() } else { minimise(case, &pred) };
        let o = evaluate(&min);
        let _ = writeln!(out, "SIG {:?} count={cnt}\n  minimal ({} bytes): {}", sig, min.len(), hex(&min));
        if o.a[0] != o.b[0] {
            let _ = writeln!(out, "  parse: verus={} creusot={}", render(&o.a[0]), render(&o.b[0]));
        } else {
            let _ = writeln!(out, "  parse (both): {}", render(&o.a[0]));
        }
        for (i, k) in o.keys.iter().enumerate() {
            if o.a[i + 1] != o.b[i + 1] {
                let _ = writeln!(out, "  {}: verus={} creusot={}", key_str(k), render(&o.a[i + 1]), render(&o.b[i + 1]));
            }
        }
    }
    std::fs::write("out/divergences.txt", &out).unwrap();

    // oracle dump
    let mut dump = String::new();
    for (kind, b) in &st.sample {
        let o = evaluate(b);
        let _ = writeln!(dump, "CASE {kind} {}", hex(b));
        for (name, rs) in [("V", &o.a), ("C", &o.b)] {
            let _ = writeln!(dump, "{name} parse {}", render(&rs[0]));
            for (i, k) in o.keys.iter().enumerate() {
                let _ = writeln!(dump, "{name} {} {}", key_str(k), render(&rs[i + 1]));
            }
        }
    }
    std::fs::write("out/oracle_sample.txt", dump).unwrap();
    println!("elapsed {:?}", t0.elapsed());
}
