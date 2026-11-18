use inflate_verus::{inflate_zlib_into, InflateOptions, SegmentSlices, Segments, Status};

// The mathematical views of this wrapper can delegate to `inner` unchanged:
// concat, pos, seg_index, seg_count, seg_steps, core_steps, advance_rank, wf.
// The marker write is straight-line code and adds no loop work under the manifest.
struct MarkingSource<'a> {
    inner: SegmentSlices<'a>,
    marker: &'a mut u8,
}
impl Segments for MarkingSource<'_> {
    fn read(&mut self) -> Option<u8> {
        *self.marker = 99;
        self.inner.read()
    }
}
fn main() {
    let z = [0x78u8, 0x01, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01];
    let parts: [&[u8]; 1] = [&z];
    let mut marker = 7u8;
    let mut out = [0xa5u8; 1];
    let result;
    {
        let mut source = MarkingSource { inner: SegmentSlices::new(&parts).unwrap(), marker: &mut marker };
        result = inflate_zlib_into(&mut source, &mut out, InflateOptions::STRICT);
    }
    assert!(result.status == Status::Done && result.produced == 0);
    assert_eq!(out, [0xa5]);
    assert_eq!(marker, 99);
    println!("Done, produced=0, output unchanged, unrelated caller marker changed 7 -> {}", marker);
}
