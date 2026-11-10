//! Runner for PNM / QOI / farbfeld. Usage: runner FORMAT:SELECTOR FILE [RGBA8_DUMP]
//! Normalisation: every decoder's output is expanded to canonical RGBA16 (8-bit v -> v*257, grey -> r=g=b,
//! missing alpha -> 65535); RGBA8 = (v16+128)/257 (identity for 8-bit sources). rgba_sha256 hashes RGBA8,
//! rgba16_sha256 hashes RGBA16 little-endian. No decoder-specific scaling is applied by the harness
//! except that expansion; if a decoder returns unscaled low-maxval samples they are hashed as returned.
use std::{io::Cursor,path::Path};
use glycin_utils::{FrameRequest,InitializationDetails,LoaderImplementation,LocalMemory,MemoryFormat,ProcessError};
use image::{ColorType,ImageDecoder};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};
use zune_core::bytestream::ZCursor;

const LIMIT:u64=16_000_000;
fn err(e:impl std::fmt::Display)->Value{
    let s=e.to_string();
    json!({"status":"error","error_class":s.split([':','(','{']).next().unwrap_or("error").trim().chars().take(80).collect::<String>(),"message":s.chars().take(300).collect::<String>()})
}
fn tag(class:&str,msg:&str)->Value{json!({"status":"error","error_class":class,"message":msg})}
fn too_big(w:u64,h:u64)->Option<Value>{
    if w==0||h==0{return Some(tag("empty-image","zero width or height"))}
    if w*h>LIMIT{return Some(tag("harness-limit","more than 16,000,000 pixels; not decoded"))}
    None
}
/// samples: w*h*ch values, already scaled to the 16-bit range.
fn finish(w:u32,h:u32,ch:usize,s:Vec<u16>,orig:String,sel:String,dump:Option<&str>)->Value{
    if let Some(v)=too_big(w as u64,h as u64){return v}
    if s.len()!=w as usize*h as usize*ch{return tag("bad-output-length",&format!("{} samples for {}x{}x{}",s.len(),w,h,ch))}
    let mut r16=Vec::with_capacity(w as usize*h as usize*8);let mut r8=Vec::with_capacity(w as usize*h as usize*4);
    for p in s.chunks_exact(ch){
        let px:[u16;4]=match ch{1=>[p[0],p[0],p[0],65535],2=>[p[0],p[0],p[0],p[1]],3=>[p[0],p[1],p[2],65535],_=>[p[0],p[1],p[2],p[3]]};
        for v in px{r16.extend(v.to_le_bytes());r8.push(((v as u32+128)/257) as u8)}
    }
    if let Some(d)=dump{if let Err(e)=std::fs::write(d,&r8){return err(e)}}
    json!({"status":"ok","width":w,"height":h,"orig_color_type":orig,"alpha_present":r8.chunks_exact(4).any(|p|p[3]!=255),
           "rgba_sha256":format!("{:x}",Sha256::digest(&r8)),"rgba16_sha256":format!("{:x}",Sha256::digest(&r16)),"selection":sel})
}
fn from8(b:&[u8])->Vec<u16>{b.iter().map(|&v|v as u16*257).collect()}
fn from16ne(b:&[u8])->Vec<u16>{b.chunks_exact(2).map(|c|u16::from_ne_bytes([c[0],c[1]])).collect()}
fn from16be(b:&[u8])->Vec<u16>{b.chunks_exact(2).map(|c|u16::from_be_bytes([c[0],c[1]])).collect()}
fn from16le(b:&[u8])->Vec<u16>{b.chunks_exact(2).map(|c|u16::from_le_bytes([c[0],c[1]])).collect()}

fn image_dec<D:ImageDecoder>(d:D,dump:Option<&str>)->Value{
    let (w,h)=d.dimensions();let ct=d.color_type();
    if let Some(v)=too_big(w as u64,h as u64){return v}
    let mut buf=vec![0u8;d.total_bytes() as usize];
    if let Err(e)=d.read_image(&mut buf){return err(e)}
    let (ch,s)=match ct{
        ColorType::L8=>(1,from8(&buf)),ColorType::La8=>(2,from8(&buf)),ColorType::Rgb8=>(3,from8(&buf)),ColorType::Rgba8=>(4,from8(&buf)),
        ColorType::L16=>(1,from16ne(&buf)),ColorType::La16=>(2,from16ne(&buf)),ColorType::Rgb16=>(3,from16ne(&buf)),ColorType::Rgba16=>(4,from16ne(&buf)),
        o=>return tag("unsupported-output",&format!("{o:?}"))};
    finish(w,h,ch,s,format!("{ct:?}"),"single".into(),dump)
}
fn glycin_error(e:ProcessError)->Value{
    let class=match &e{ProcessError::ExpectedError{..}=>"ExpectedError",ProcessError::InternalError{..}=>"InternalError",
        ProcessError::UnsupportedImageFormat(_)=>"UnsupportedImageFormat",ProcessError::ConversionTooLargerError=>"ConversionTooLarge",
        ProcessError::OutOfMemory{..}=>"OutOfMemory",ProcessError::NoMoreFrames=>"NoMoreFrames",_=>"OtherProcessError"};
    json!({"status":"error","error_class":class,"message":e.to_string().chars().take(300).collect::<String>()})
}
fn glycin(mime:&str,bytes:&[u8],dump:Option<&str>)->Value{
    let (mut loader,_)=match glycin_image_rs::ImgLoader::load::<LocalMemory,_>(Cursor::new(bytes.to_vec()),mime.into(),InitializationDetails::default()){Ok(v)=>v,Err(e)=>return glycin_error(e)};
    let f=match loader.specific_frame::<LocalMemory>(FrameRequest::default()){Ok(f)=>f,Err(e)=>return glycin_error(e)};
    let (w,h)=(f.width,f.height);let stride=f.stride as usize;let fmt=f.memory_format;
    if let Some(v)=too_big(w as u64,h as u64){return v}
    let (ch,bytes_per)=match fmt{MemoryFormat::G8=>(1,1),MemoryFormat::G8a8=>(2,1),MemoryFormat::R8g8b8=>(3,1),MemoryFormat::R8g8b8a8=>(4,1),
        MemoryFormat::G16=>(1,2),MemoryFormat::G16a16=>(2,2),MemoryFormat::R16g16b16=>(3,2),MemoryFormat::R16g16b16a16=>(4,2),
        o=>return tag("unsupported-output",&format!("{o:?}"))};
    let row=w as usize*ch*bytes_per;let mut s=Vec::new();
    for r in f.texture.chunks(stride).take(h as usize){
        if r.len()<row{return tag("short-row","glycin frame row shorter than width")}
        s.extend(if bytes_per==1{from8(&r[..row])}else{from16ne(&r[..row])});
    }
    finish(w,h,ch,s,format!("{fmt:?}"),"single".into(),dump)
}
fn zune_pnm(bytes:&[u8],dump:Option<&str>)->Value{
    use zune_core::{bit_depth::BitDepth,colorspace::ColorSpace,result::DecodingResult};
    let mut d=zune_ppm::PPMDecoder::new(ZCursor::new(bytes));
    if let Err(e)=d.decode_headers(){return err(format!("{e:?}"))}
    let (w,h)=d.dimensions().unwrap_or((0,0));
    if let Some(v)=too_big(w as u64,h as u64){return v}
    let cs=d.colorspace();let bd=d.bit_depth();
    let ch=match cs{Some(ColorSpace::Luma)=>1,Some(ColorSpace::LumaA)=>2,Some(ColorSpace::RGB)=>3,Some(ColorSpace::RGBA)=>4,o=>return tag("unsupported-output",&format!("{o:?}"))};
    let r=match d.decode(){Ok(r)=>r,Err(e)=>return err(format!("{e:?}"))};
    let s=match r{DecodingResult::U8(v)=>from8(&v),DecodingResult::U16(v)=>v,_=>return tag("unsupported-output","float")};
    finish(w as u32,h as u32,ch,s,format!("{cs:?}/{bd:?}"),"single".into(),dump)
}
fn oxide_pnm(bytes:&[u8],dump:Option<&str>)->Value{
    use oxideav_pbm::PbmPixelFormat as F;
    let info=match oxideav_pbm::info(bytes){Ok(i)=>i,Err(e)=>return err(format!("{e:?}"))};
    if let Some(v)=too_big(info.width as u64,info.height as u64){return v}
    let im=match oxideav_pbm::decode(bytes){Ok(i)=>i,Err(e)=>return err(format!("{e:?}"))};
    let (w,h)=(im.width,im.height);let p=&im.planes[0];let f=im.format;
    let (ch,bpp):(usize,usize)=match f{F::Gray8=>(1,1),F::Gray16Le=>(1,2),F::Ya8=>(2,1),F::Ya16Le=>(2,2),F::Rgb24=>(3,1),F::Rgb48Le=>(3,2),F::Rgba=>(4,1),F::Rgba64Le=>(4,2),F::MonoBlack=>(1,0),
        o=>return tag("unsupported-output",&format!("{o:?}"))};
    let mut s=Vec::new();
    for y in 0..h as usize{
        let row=&p.data[y*p.stride..];
        if bpp==0{for x in 0..w as usize{s.push(if row[x/8]>>(7-x%8)&1==1{0}else{65535})}}
        else{let n=w as usize*ch*bpp;s.extend(if bpp==1{from8(&row[..n])}else{from16le(&row[..n])})}
    }
    finish(w,h,ch,s,format!("{f:?}"),"single".into(),dump)
}
fn qoi_pixels(w:u32,h:u32,ch:usize,px:Vec<u8>,orig:String,dump:Option<&str>)->Value{finish(w,h,ch,from8(&px),orig,"single".into(),dump)}
fn run(which:&str,bytes:&[u8],dump:Option<&str>)->Value{
    match which{
        "pnm:image"=>match image::codecs::pnm::PnmDecoder::new(Cursor::new(bytes)){Ok(d)=>image_dec(d,dump),Err(e)=>err(e)},
        "qoi:image"=>match image::codecs::qoi::QoiDecoder::new(Cursor::new(bytes)){Ok(d)=>image_dec(d,dump),Err(e)=>err(e)},
        "farbfeld:image"=>match image::codecs::farbfeld::FarbfeldDecoder::new(Cursor::new(bytes)){Ok(d)=>image_dec(d,dump),Err(e)=>err(e)},
        "pnm:glycin"=>glycin("image/x-portable-anymap",bytes,dump),
        "qoi:glycin"=>glycin("image/qoi",bytes,dump),
        "farbfeld:glycin"=>glycin("image/x-ff",bytes,dump),
        "pnm:zune"=>zune_pnm(bytes,dump),
        "pnm:oxideav"=>oxide_pnm(bytes,dump),
        "qoi:qoi-crate"=>{
            let hd=match qoi::decode_header(bytes){Ok(h)=>h,Err(e)=>return err(e)};
            if let Some(v)=too_big(hd.width as u64,hd.height as u64){return v}
            match qoi::decode_to_vec(bytes){Ok((h,px))=>qoi_pixels(h.width,h.height,h.channels.as_u8() as usize,px,format!("{:?}/{:?}",h.channels,h.colorspace),dump),Err(e)=>err(e)}
        },
        "qoi:rapid"=>{
            let hd=match rapid_qoi::Qoi::decode_header(bytes){Ok(h)=>h,Err(e)=>return err(format!("{e:?}"))};
            if let Some(v)=too_big(hd.width as u64,hd.height as u64){return v}
            match rapid_qoi::Qoi::decode_alloc(bytes){Ok((h,px))=>{let ch=if h.colors.has_alpha(){4}else{3};qoi_pixels(h.width,h.height,ch,px,format!("{:?}",h.colors),dump)},Err(e)=>err(format!("{e:?}"))}
        },
        "qoi:zune"=>{
            let mut d=zune_qoi::QoiDecoder::new(ZCursor::new(bytes));
            if let Err(e)=d.decode_headers(){return err(format!("{e:?}"))}
            let (w,h)=d.dimensions().unwrap_or((0,0));
            if let Some(v)=too_big(w as u64,h as u64){return v}
            let cs=d.colorspace().unwrap();
            let ch=cs.num_components();
            match d.decode(){Ok(px)=>qoi_pixels(w as u32,h as u32,ch,px,format!("{cs:?}"),dump),Err(e)=>err(format!("{e:?}"))}
        },
        "farbfeld:crate"=>{
            let mut d=match farbfeld::Decoder::new(Cursor::new(bytes)){Ok(d)=>d,Err(e)=>return err(format!("{e:?}"))};
            let (w,h)=d.dimensions();
            if let Some(v)=too_big(w as u64,h as u64){return v}
            match d.read_image(){Ok(b)=>finish(w,h,4,from16be(&b),"Rgba16".into(),"single".into(),dump),Err(e)=>err(format!("{e:?}"))}
        },
        "farbfeld:zune"=>{
            let mut d=zune_farbfeld::FarbFeldDecoder::new(ZCursor::new(bytes));
            if let Err(e)=d.decode_headers(){return err(format!("{e:?}"))}
            let (w,h)=d.dimensions().unwrap_or((0,0));
            if let Some(v)=too_big(w as u64,h as u64){return v}
            let mut s=vec![0u16;w*h*4];
            match d.decode_into(&mut s){Ok(())=>finish(w as u32,h as u32,4,s,"Rgba16".into(),"single".into(),dump),Err(e)=>err(format!("{e:?}"))}
        },
        _=>err("unknown selector")
    }
}
fn main(){let a:Vec<String>=std::env::args().collect();let r=std::panic::catch_unwind(||{
    if a.len()<3{return err("usage: runner FORMAT:SELECTOR FILE [RGBA_DUMP]")}
    match std::fs::read(Path::new(&a[2])){Ok(b)=>run(&a[1],&b,a.get(3).map(String::as_str)),Err(e)=>err(e)}
}).unwrap_or_else(|_|json!({"status":"crash","error_class":"rust_panic"}));println!("{r}");}
