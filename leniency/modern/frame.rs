//! Shared by the webp/jp2/jxl Rust runners: JSON contract, glycin driver, RGBA8 normalisation.
use std::io::Cursor;
use glycin_utils::{FrameRequest,InitializationDetails,LoaderImplementation,LocalMemory,MemoryFormat,ProcessError};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};

pub const MAX_PIXELS:u64=16_000_000;
pub const MAX_FRAMES:usize=32;

pub fn sha(b:&[u8])->String{format!("{:x}",Sha256::digest(b))}
pub fn err(e:impl std::fmt::Display)->Value{
    let s=e.to_string();
    json!({"status":"error","error_class":s.split([':','(','{']).next().unwrap_or("error").trim().chars().take(80).collect::<String>(),
           "message":s.chars().take(300).collect::<String>()})
}
/// One decoded image: straight (non-premultiplied) top-first RGBA8, origin colour type as a string.
pub struct Img{pub w:u32,pub h:u32,pub rgba:Vec<u8>,pub orig:String}
pub fn ok(frames:&[Img],dump:Option<&str>,extra:Value)->Value{
    let f=&frames[0];
    if f.w==0||f.h==0||f.w as u64*f.h as u64>MAX_PIXELS||f.rgba.len()!=f.w as usize*f.h as usize*4{return err("invalid output dimensions or length")}
    if let Some(p)=dump{let all:Vec<u8>=frames.iter().flat_map(|x|x.rgba.iter().copied()).collect();if let Err(e)=std::fs::write(p,&all){return err(e)}}
    let all:String=frames.iter().map(|x|format!("{}x{}:{}",x.w,x.h,sha(&x.rgba))).collect::<Vec<_>>().join(",");
    let mut v=json!({"status":"ok","width":f.w,"height":f.h,"orig_color_type":f.orig,
        "alpha_present":f.rgba.chunks_exact(4).any(|p|p[3]!=255),"rgba_sha256":sha(&f.rgba),
        "frames":frames.len(),"frames_sha256":sha(all.as_bytes())});
    if let (Value::Object(m),Value::Object(e))=(&mut v,extra){for(k,x) in e{m.insert(k,x);}}
    v
}
pub fn glycin_error(e:ProcessError)->Value{
    let class=match &e{ProcessError::ExpectedError{..}=>"ExpectedError",ProcessError::InternalError{..}=>"InternalError",
        ProcessError::UnsupportedImageFormat(_)=>"UnsupportedImageFormat",ProcessError::ConversionTooLargerError=>"ConversionTooLarge",
        ProcessError::OutOfMemory{..}=>"OutOfMemory",ProcessError::NoMoreFrames=>"NoMoreFrames",_=>"OtherProcessError"};
    json!({"status":"error","error_class":class,"message":e.to_string().chars().take(300).collect::<String>()})
}
fn unpremul(c:u8,a:u8)->u8{if a==0{0}else{((c as u32*255+a as u32/2)/a as u32).min(255) as u8}}
fn f16(h:u16)->f32{let s=if h>>15==1{-1.0f32}else{1.0};let e=((h>>10)&31) as i32;let m=(h&1023) as f32;if e==0{s*m*2f32.powi(-24)}else if e==31{if m==0.0{s*f32::INFINITY}else{f32::NAN}}else{s*(1.0+m/1024.0)*2f32.powi(e-15)}}
fn s16(v:u16)->u8{((v as u32*255+32767)/65535) as u8}
fn sf(v:f32)->u8{(v.clamp(0.0,1.0)*255.0+0.5) as u8}
/// Convert a glycin frame to straight RGBA8. 16-bit to 8-bit: round(v*255/65535); float: clamp, round(v*255).
pub fn frame_to_img(frame:&glycin_utils::Frame<LocalMemory>)->Result<Img,String>{
    let (w,h)=(frame.width,frame.height);
    if w==0||h==0||w as u64*h as u64>MAX_PIXELS{return Err("output dimensions outside harness limit".into())}
    let stride=frame.stride as usize;let fmt=frame.memory_format;
    let (bpp,_)=match fmt{
        MemoryFormat::R8g8b8a8|MemoryFormat::B8g8r8a8|MemoryFormat::A8r8g8b8|MemoryFormat::A8b8g8r8
        |MemoryFormat::R8g8b8a8Premultiplied|MemoryFormat::B8g8r8a8Premultiplied|MemoryFormat::A8r8g8b8Premultiplied=>(4,0),
        MemoryFormat::R8g8b8|MemoryFormat::B8g8r8=>(3,0),MemoryFormat::G8=>(1,0),MemoryFormat::G8a8|MemoryFormat::G8a8Premultiplied=>(2,0),
        MemoryFormat::R16g16b16=>(6,0),MemoryFormat::R16g16b16a16|MemoryFormat::R16g16b16a16Premultiplied=>(8,0),
        MemoryFormat::G16=>(2,0),MemoryFormat::G16a16|MemoryFormat::G16a16Premultiplied=>(4,0),
        MemoryFormat::R16g16b16Float=>(6,0),MemoryFormat::R16g16b16a16Float=>(8,0),MemoryFormat::R32g32b32Float=>(12,0),MemoryFormat::R32g32b32a32Float|MemoryFormat::R32g32b32a32FloatPremultiplied=>(16,0),
        _=>return Err(format!("unsupported output memory format {fmt:?}"))};
    let mut rgba=Vec::with_capacity(w as usize*h as usize*4);
    let rows:Vec<&[u8]>=frame.texture.chunks(stride).take(h as usize).collect();
    if rows.len()<h as usize{return Err("texture shorter than height*stride".into())}
    for row in rows{
        if row.len()<w as usize*bpp{return Err("row shorter than width".into())}
        for p in row[..w as usize*bpp].chunks_exact(bpp){
            let u16at=|i:usize|u16::from_ne_bytes([p[i],p[i+1]]);
            let f32at=|i:usize|f32::from_ne_bytes([p[i],p[i+1],p[i+2],p[i+3]]);
            let px:[u8;4]=match fmt{
                MemoryFormat::R8g8b8a8=>[p[0],p[1],p[2],p[3]],MemoryFormat::B8g8r8a8=>[p[2],p[1],p[0],p[3]],
                MemoryFormat::A8r8g8b8=>[p[1],p[2],p[3],p[0]],MemoryFormat::A8b8g8r8=>[p[3],p[2],p[1],p[0]],
                MemoryFormat::R8g8b8a8Premultiplied=>[unpremul(p[0],p[3]),unpremul(p[1],p[3]),unpremul(p[2],p[3]),p[3]],
                MemoryFormat::B8g8r8a8Premultiplied=>[unpremul(p[2],p[3]),unpremul(p[1],p[3]),unpremul(p[0],p[3]),p[3]],
                MemoryFormat::A8r8g8b8Premultiplied=>[unpremul(p[1],p[0]),unpremul(p[2],p[0]),unpremul(p[3],p[0]),p[0]],
                MemoryFormat::R8g8b8=>[p[0],p[1],p[2],255],MemoryFormat::B8g8r8=>[p[2],p[1],p[0],255],
                MemoryFormat::G8=>[p[0],p[0],p[0],255],MemoryFormat::G8a8=>[p[0],p[0],p[0],p[1]],
                MemoryFormat::G8a8Premultiplied=>{let g=unpremul(p[0],p[1]);[g,g,g,p[1]]},
                MemoryFormat::R16g16b16=>[s16(u16at(0)),s16(u16at(2)),s16(u16at(4)),255],
                MemoryFormat::R16g16b16a16=>[s16(u16at(0)),s16(u16at(2)),s16(u16at(4)),s16(u16at(6))],
                MemoryFormat::R16g16b16a16Premultiplied=>{let a=s16(u16at(6));[unpremul(s16(u16at(0)),a),unpremul(s16(u16at(2)),a),unpremul(s16(u16at(4)),a),a]},
                MemoryFormat::G16=>{let g=s16(u16at(0));[g,g,g,255]},
                MemoryFormat::G16a16=>{let g=s16(u16at(0));[g,g,g,s16(u16at(2))]},
                MemoryFormat::G16a16Premultiplied=>{let a=s16(u16at(2));let g=unpremul(s16(u16at(0)),a);[g,g,g,a]},
                MemoryFormat::R16g16b16Float=>[sf(f16(u16at(0))),sf(f16(u16at(2))),sf(f16(u16at(4))),255],
                MemoryFormat::R16g16b16a16Float=>[sf(f16(u16at(0))),sf(f16(u16at(2))),sf(f16(u16at(4))),sf(f16(u16at(6)))],
                MemoryFormat::R32g32b32Float=>[sf(f32at(0)),sf(f32at(4)),sf(f32at(8)),255],
                MemoryFormat::R32g32b32a32Float=>[sf(f32at(0)),sf(f32at(4)),sf(f32at(8)),sf(f32at(12))],
                MemoryFormat::R32g32b32a32FloatPremultiplied=>{let a=sf(f32at(12));[unpremul(sf(f32at(0)),a),unpremul(sf(f32at(4)),a),unpremul(sf(f32at(8)),a),a]},
                _=>return Err("unreachable format".into())};
            rgba.extend_from_slice(&px);
        }
    }
    Ok(Img{w,h,rgba,orig:format!("{fmt:?}")})
}
/// glycin-image-rs `ImgLoader` as glycin calls it: load, then `specific_frame` until NoMoreFrames
/// (loop_animation=false) or MAX_FRAMES. A later-frame error after a good first frame is recorded, not fatal.
pub fn glycin_run(bytes:&[u8],mime:&str,dump:Option<&str>)->Value{
    let (mut loader,details)=match glycin_image_rs::ImgLoader::load::<LocalMemory,_>(Cursor::new(bytes.to_vec()),mime.into(),InitializationDetails::default()){Ok(v)=>v,Err(e)=>return glycin_error(e)};
    let (dw,dh)=(details.width,details.height);
    let mut frames:Vec<Img>=vec![];let mut later=Value::Null;
    for i in 0..MAX_FRAMES{
        let mut req=FrameRequest::default();req.loop_animation=false;
        match loader.specific_frame::<LocalMemory>(req){
            Ok(f)=>match frame_to_img(&f){Ok(im)=>frames.push(im),Err(e)=>{if i==0{return err(e)}later=json!(e);break}},
            Err(ProcessError::NoMoreFrames)=>break,
            Err(e)=>{if i==0{return glycin_error(e)}later=glycin_error(e);break}
        }
    }
    if frames.is_empty(){return err("no frames")}
    ok(&frames,dump,json!({"header_width":dw,"header_height":dh,"later_frame_error":later,"truncated_frame_list":frames.len()==MAX_FRAMES}))
}
pub fn main_wrap(f:impl Fn(&str,&[u8],Option<&str>)->Value+std::panic::RefUnwindSafe){
    let a:Vec<String>=std::env::args().collect();
    let result=std::panic::catch_unwind(||{
        if a.len()<3{return err("usage: runner SELECTOR FILE [RGBA_DUMP]")}
        match std::fs::read(&a[2]){Ok(b)=>f(&a[1],&b,a.get(3).map(String::as_str)),Err(e)=>err(e)}
    }).unwrap_or_else(|_|json!({"status":"crash","error_class":"rust_panic"}));
    println!("{result}");
}
