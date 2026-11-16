use super::*;
use alloc::vec;
use alloc::vec::Vec;

struct RecordingSink {
    calls: Vec<(u32,u8)>,
    rejected: Option<u8>,
}
impl Sink for RecordingSink {
    fn put(&mut self, q:u32, v:u8) -> bool {
        self.calls.push((q,v));
        self.rejected != Some(v)
    }
}

fn packed(width:u32,codes:&[u32]) -> Vec<u8> {
    let mut bytes=vec![0u8; (width as usize*codes.len()+7)/8];
    for (i,&c) in codes.iter().enumerate() {
        for j in 0..width as usize {
            let bit=i*width as usize+j;
            bytes[bit/8] |= (((c>>j)&1) as u8)<<(bit%8);
        }
    }
    bytes
}

fn run(t:&mut [u8],m:u32,d:&[u8],area:u32,stop:bool,checked:bool,rejected:Option<u8>) -> (LzwReport,Vec<(u32,u8)>) {
    let mut b=Vec::new();
    for chunk in d.chunks(1) {
        b.push(chunk.len() as u8);
        b.extend_from_slice(chunk);
    }
    b.push(0);
    let mut reader=BitReader::new(&b,0,d.len() as u32);
    let mut sink=RecordingSink {calls:Vec::new(),rejected};
    let report=decode(&mut reader,d.len() as u32,m,area,
        LzwOptions {tail:if stop {Tail::StopAtRaster} else {Tail::Strict},checked_literals:checked},t,&mut sink);
    (report,sink.calls)
}

#[test]
fn independent_literal_pair_storage_and_decode() {
    for fill in [0u8,0xa5u8] {
        let mut t=vec![fill;LZW_TABLE_BYTES];
        witness_literal_pair_entry(&mut t);
        assert_eq!((prefix_get(&t,6),suffix_get(&t,6),stored_len(&t,6)),(1,2,2));
        t.fill(fill);
        // Codes 1,2,6 use three bits; adding entry 7 widens EOI to four bits.
        let (r,h)=run(&mut t,2,&[145,11],4,false,false,None);
        assert_eq!(h,vec![(0,1),(1,2),(3,2),(2,1)]);
        assert_eq!((r.written,r.error,r.pos,r.e4),(4,None,13,false));
        assert_eq!((prefix_get(&t,6),suffix_get(&t,6),stored_len(&t,6)),(1,2,2));
    }
}

#[test]
fn reverse_kwkwk_and_crossing_observations() {
    let mut t=vec![0xa5;LZW_TABLE_BYTES];
    let d=[140,11];
    let (r,h)=run(&mut t,2,&d,5,false,false,None);
    assert_eq!(h,vec![(0,1),(2,1),(1,1)]);
    assert_eq!((r.written,r.error,r.pos,r.e4),(3,Some(Error::TooFewPixels),12,false));
    let (r,h)=run(&mut t,2,&d,2,false,false,None);
    assert_eq!(h,vec![(0,1),(1,1)]);
    assert_eq!((r.written,r.error,r.pos,r.e4),(2,Some(Error::TooManyPixels),9,false));
    let (r,h)=run(&mut t,2,&d,2,true,true,None);
    assert_eq!(h,vec![(0,1),(1,1)]);
    assert_eq!((r.written,r.error,r.pos,r.e4),(2,None,9,true));
}

#[test]
fn colour_fault_does_not_stop_the_pure_fold() {
    let mut t=vec![0x67;LZW_TABLE_BYTES];
    let (r,h)=run(&mut t,2,&[140,11],5,false,false,Some(1));
    assert_eq!(h,vec![(0,1),(2,1),(1,1)]);
    assert_eq!((r.written,r.error,r.pos,r.e4),(3,Some(Error::TooFewPixels),12,false));
    let c=r.colour.unwrap();
    assert_eq!((c.q,c.pos,r.public_pos()),(0,6,6));
    assert_eq!(r.public_result(),Err(Error::ColorIndexOutOfRange));

    let d=packed(3,&[4,1,2,5]);
    let (r,h)=run(&mut t,2,&d,2,false,false,Some(1));
    assert_eq!(h,vec![(0,1),(1,2)]);
    assert_eq!((r.written,r.error,r.pos),(2,None,12));
    assert_eq!(r.public_pos(),6);
    assert_eq!(r.public_result(),Err(Error::ColorIndexOutOfRange));
}

#[test]
fn colour_position_precedes_a_later_invalid_code() {
    let mut t=vec![0xff;LZW_TABLE_BYTES];
    let d=packed(3,&[4,1,7]);
    let (r,h)=run(&mut t,2,&d,3,false,false,Some(1));
    assert_eq!(h,vec![(0,1)]);
    assert_eq!((r.written,r.error,r.pos),(1,Some(Error::InvalidLzwCode),9));
    assert_eq!(r.public_pos(),6);
    assert_eq!(r.public_result(),Err(Error::ColorIndexOutOfRange));
}

#[test]
fn r12_before_read_and_after_code_errors() {
    let mut t=vec![0x8d;LZW_TABLE_BYTES];
    let (r,h)=run(&mut t,2,&[],1,false,false,None);
    assert!(h.is_empty());
    assert_eq!((r.error,r.pos),(Some(Error::TooFewPixels),0));
    let d=packed(3,&[4,1]);
    let (r,_)=run(&mut t,2,&d,1,false,false,None);
    assert_eq!((r.error,r.pos),(Some(Error::MissingEndOfInformation),6));
    let mut d=packed(3,&[4,1,5]);
    d.push(0);
    let (r,_)=run(&mut t,2,&d,1,false,false,None);
    assert_eq!((r.error,r.pos),(Some(Error::DataAfterEndOfInformation),9));
    let d=packed(10,&[512,300]);
    let (r,h)=run(&mut t,9,&d,1,true,true,None);
    assert!(h.is_empty());
    assert_eq!((r.error,r.pos),(Some(Error::ColorIndexOutOfRange),20));
}

#[test]
fn literals_ignore_stale_dictionary_bytes_and_size_one_stops() {
    let mut t=vec![0xee;LZW_TABLE_BYTES];
    let _=run(&mut t,2,&[140,11],5,false,false,None);
    assert_eq!(raw_stored_len_for_test(&t,6),2);
    let d=packed(9,&[256,6,257]);
    let (r,h)=run(&mut t,8,&d,1,false,false,None);
    assert_eq!(h,vec![(0,6)]);
    assert_eq!((r.written,r.error,r.pos),(1,None,27));
    let d=packed(2,&[2,1]);
    let (r,h)=run(&mut t,1,&d,1,true,true,None);
    assert_eq!(h,vec![(0,1)]);
    assert_eq!((r.written,r.error,r.pos,r.e4),(1,None,4,false));
}

fn raw_stored_len_for_test(t:&[u8],c:usize)->u16 {
    u16::from_le_bytes([t[LENGTH_OFF+2*c],t[LENGTH_OFF+2*c+1]])
}
