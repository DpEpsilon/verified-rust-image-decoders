use crate::{
    adler::Adler,
    bits::Bits,
    huffman::{Huffman, Kind},
    limits::*,
    InflateError as E, InflateOptions, InflateResult, Segments, Status, Window, Work, ZlibTail,
};

const LEN_BASE: [u16; 29] = [
    3, 4, 5, 6, 7, 8, 9, 10, 11, 13, 15, 17, 19, 23, 27, 31, 35, 43, 51, 59, 67, 83, 99, 115, 131,
    163, 195, 227, 258,
];
const LEN_EXTRA: [u8; 29] = [
    0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 1, 2, 2, 2, 2, 3, 3, 3, 3, 4, 4, 4, 4, 5, 5, 5, 5, 0,
];
const DIST_BASE: [u16; NDIST_MAX] = [
    1, 2, 3, 4, 5, 7, 9, 13, 17, 25, 33, 49, 65, 97, 129, 193, 257, 385, 513, 769, 1025, 1537,
    2049, 3073, 4097, 6145, 8193, 12289, 16385, 24577,
];
const DIST_EXTRA: [u8; NDIST_MAX] = [
    0, 0, 0, 0, 1, 1, 2, 2, 3, 3, 4, 4, 5, 5, 6, 6, 7, 7, 8, 8, 9, 9, 10, 10, 11, 11, 12, 12, 13,
    13,
];
const CL_ORDER: [usize; NCL] = [
    16, 17, 18, 0, 8, 7, 9, 6, 10, 5, 11, 4, 12, 3, 13, 2, 14, 1, 15,
];

/// The public signatures require ring.len() == WINDOW_MAX for measuring, and
/// a fresh source satisfying Segments' contract. A typed array inside the sink
/// prevents accidental indexing of a shorter ring. No stream-driven allocation.
enum Sink<'a> {
    Into(&'a mut [u8]),
    Ring(&'a mut [u8; WINDOW_MAX]),
}
struct Output<'a> {
    sink: Sink<'a>,
    produced: u64,
    limit: u64,
    adler: Adler,
}
impl Output<'_> {
    fn append_byte(&mut self, byte: u8) -> Result<(), Status> {
        if self.produced == self.limit {
            return Err(Status::Full);
        }
        match &mut self.sink {
            Sink::Into(out) => out[self.produced as usize] = byte,
            Sink::Ring(ring) => ring[(self.produced % WINDOW_MAX as u64) as usize] = byte,
        }
        self.produced += 1;
        self.adler.update_byte(byte);
        Ok(())
    }
    fn previous(&self, distance: u64) -> u8 {
        let index = self.produced - distance;
        match &self.sink {
            Sink::Into(out) => out[index as usize],
            Sink::Ring(ring) => ring[(index % WINDOW_MAX as u64) as usize],
        }
    }
}

struct Tables {
    fixed_lit: Huffman,
    fixed_dist: Huffman,
    lit: Huffman,
    dist: Huffman,
    code: Huffman,
}
impl Tables {
    fn new(w: &mut Work) -> Result<Self, E> {
        let mut tables = Self {
            fixed_lit: Huffman::new(w),
            fixed_dist: Huffman::new(w),
            lit: Huffman::new(w),
            dist: Huffman::new(w),
            code: Huffman::new(w),
        };
        let mut lengths = [0u8; FIXED_LIT_COUNT];
        w.add(FIXED_LIT_COUNT);
        let mut i = 0;
        while i < FIXED_LIT_COUNT {
            w.add(1);
            lengths[i] = if i < 144 {
                8
            } else if i < 256 {
                9
            } else if i < 280 {
                7
            } else {
                8
            };
            i += 1;
        }
        tables.fixed_lit.build(&lengths, Kind::LitLen, w)?;
        w.add(FIXED_DIST_COUNT);
        tables
            .fixed_dist
            .build(&[5; FIXED_DIST_COUNT], Kind::Dist, w)?;
        Ok(tables)
    }
    fn dynamic<S: Segments>(&mut self, br: &mut Bits<'_, S>, w: &mut Work) -> Result<(), E> {
        let nlen = br.get(5, w)? as usize + 257;
        let ndist = br.get(5, w)? as usize + 1;
        let ncode = br.get(4, w)? as usize + 4;
        if nlen > NLIT_MAX || ndist > NDIST_MAX {
            return Err(E::InvalidCodeLengths);
        }
        let mut cl = [0u8; NCL];
        w.add(NCL);
        let mut i = 0;
        while i < ncode {
            w.add(1);
            cl[CL_ORDER[i]] = br.get(3, w)? as u8;
            i += 1;
        }
        self.code.build(&cl, Kind::CodeLen, w)?;
        let mut lengths = [0u8; DYNAMIC_LENGTHS];
        w.add(DYNAMIC_LENGTHS);
        let total = nlen + ndist;
        i = 0;
        while i < total {
            w.add(1);
            let sym = self.code.decode(br, w)?;
            if sym < 16 {
                lengths[i] = sym as u8;
                i += 1;
            } else {
                let (value, repeat) = if sym == 16 {
                    if i == 0 {
                        return Err(E::InvalidCodeLengths);
                    }
                    (lengths[i - 1], 3 + br.get(2, w)? as usize)
                } else if sym == 17 {
                    (0, 3 + br.get(3, w)? as usize)
                } else {
                    (0, 11 + br.get(7, w)? as usize)
                };
                if repeat > total - i {
                    return Err(E::InvalidCodeLengths);
                }
                let end = i + repeat;
                while i < end {
                    w.add(1);
                    lengths[i] = value;
                    i += 1;
                }
            }
        }
        if lengths[256] == 0 {
            return Err(E::InvalidCodeLengths);
        }
        self.lit.build(&lengths[..nlen], Kind::LitLen, w)?;
        self.dist.build(&lengths[nlen..total], Kind::Dist, w)?;
        Ok(())
    }
}

fn compressed<S: Segments>(
    br: &mut Bits<'_, S>,
    output: &mut Output<'_>,
    lit: &Huffman,
    dist: &Huffman,
    window: u64,
    w: &mut Work,
) -> Result<(), Status> {
    loop {
        w.add(1);
        let sym = lit.decode(br, w).map_err(Status::Err)?;
        if sym < 256 {
            output.append_byte(sym as u8)?;
        } else if sym == 256 {
            return Ok(());
        } else {
            if sym > 285 {
                return Err(Status::Err(E::InvalidSymbol));
            }
            let index = usize::from(sym - 257);
            let length =
                u32::from(LEN_BASE[index]) + br.get(LEN_EXTRA[index], w).map_err(Status::Err)?;
            let ds = usize::from(dist.decode(br, w).map_err(Status::Err)?);
            if ds >= NDIST_MAX {
                return Err(Status::Err(E::InvalidSymbol));
            }
            let distance = u64::from(DIST_BASE[ds])
                + u64::from(br.get(DIST_EXTRA[ds], w).map_err(Status::Err)?);
            if distance > output.produced || distance > window {
                return Err(Status::Err(E::DistanceTooFar));
            }
            let mut j = 0;
            while j < length {
                w.add(1);
                // Resolve against the evolving output/ring, including overlap.
                let byte = output.previous(distance);
                output.append_byte(byte)?;
                j += 1;
            }
        }
    }
}

fn run<S: Segments>(
    br: &mut Bits<'_, S>,
    output: &mut Output<'_>,
    o: InflateOptions,
    w: &mut Work,
) -> Result<(), Status> {
    let cmf = br.byte(w).map_err(Status::Err)?;
    let flg = br.byte(w).map_err(Status::Err)?;
    if (u16::from(cmf) * 256 + u16::from(flg)) % 31 != 0 {
        return Err(Status::Err(E::InvalidZlibHeader));
    }
    if cmf & 15 != 8 || cmf >> 4 > 7 {
        return Err(Status::Err(E::UnsupportedZlibMethod));
    }
    if flg & 0x20 != 0 {
        return Err(Status::Err(E::ZlibPresetDictionary));
    }
    let window = match o.window {
        Window::Enforce => 1u64 << ((cmf >> 4) + 8),
        Window::Ignore => WINDOW_MAX as u64,
    };
    let mut tables = Tables::new(w).map_err(Status::Err)?;
    loop {
        w.add(1);
        let final_block = br.get(1, w).map_err(Status::Err)?;
        let block_type = br.get(2, w).map_err(Status::Err)?;
        match block_type {
            0 => {
                br.align();
                let lo = u16::from(br.byte(w).map_err(Status::Err)?);
                let hi = u16::from(br.byte(w).map_err(Status::Err)?);
                let nlo = u16::from(br.byte(w).map_err(Status::Err)?);
                let nhi = u16::from(br.byte(w).map_err(Status::Err)?);
                let len = lo | hi << 8;
                if len != !(nlo | nhi << 8) {
                    return Err(Status::Err(E::StoredLengthMismatch));
                }
                let mut i = 0;
                while i < len {
                    w.add(1);
                    // D5: availability precedes the limit check for stored bytes.
                    let byte = br.byte(w).map_err(Status::Err)?;
                    output.append_byte(byte)?;
                    i += 1;
                }
            }
            1 => compressed(br, output, &tables.fixed_lit, &tables.fixed_dist, window, w)?,
            2 => {
                tables.dynamic(br, w).map_err(Status::Err)?;
                compressed(br, output, &tables.lit, &tables.dist, window, w)?;
            }
            _ => return Err(Status::Err(E::InvalidBlockType)),
        }
        if final_block != 0 {
            break;
        }
    }
    br.align();
    let mut expected = 0u32;
    let mut i = 0;
    while i < 4 {
        w.add(1);
        expected = (expected << 8) | u32::from(br.byte(w).map_err(Status::Err)?);
        i += 1;
    }
    if expected != output.adler.finish() {
        return Err(Status::Err(E::AdlerMismatch));
    }
    // IgnoreTail is inert for a short raster: only exactly-L output may ignore
    // bytes after the checked Adler-32 (OPTIONS.md's RecommendedBeta condition).
    if o.zlib_tail == ZlibTail::IgnoreTail && output.produced == output.limit {
        return Ok(());
    }
    w.add(1);
    if br.src.read().is_some() {
        return Err(Status::Err(E::TrailingZlibData));
    }
    Ok(())
}

fn entry<S: Segments>(src: &mut S, sink: Sink<'_>, limit: u64, o: InflateOptions) -> InflateResult {
    let mut output = Output {
        sink,
        produced: 0,
        limit,
        adler: Adler::new(),
    };
    let mut work = Work::default();
    let mut br = Bits::new(src);
    let status = match run(&mut br, &mut output, o, &mut work) {
        Ok(()) => Status::Done,
        Err(s) => s,
    };
    InflateResult {
        status,
        produced: output.produced,
        core_steps: work.core,
        seg_steps: br.src.seg_steps(),
    }
}

/// Write at most out.len() bytes; preserve the unwritten suffix on every result.
/// Requires a fresh source satisfying Segments' contract. No allocation.
pub fn inflate_zlib_into<S: Segments>(
    src: &mut S,
    out: &mut [u8],
    o: InflateOptions,
) -> InflateResult {
    let limit = out.len() as u64;
    entry(src, Sink::Into(out), limit, o)
}

/// Measure using exactly WINDOW_MAX caller-owned bytes, whose initial contents
/// may be arbitrary. Requires a fresh source and ring.len() == WINDOW_MAX.
/// An invalid ring is a caller-contract violation, not an input-format error.
pub fn inflate_zlib_measure<S: Segments>(
    src: &mut S,
    ring: &mut [u8],
    limit: u64,
    o: InflateOptions,
) -> InflateResult {
    let ring: &mut [u8; WINDOW_MAX] = ring
        .try_into()
        .expect("inflate measure requires WINDOW_MAX ring bytes");
    entry(src, Sink::Ring(ring), limit, o)
}
