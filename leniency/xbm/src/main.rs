// XBM runners: glycin in-tree path, image-extras direct, `xbm` crate. One JSON line per input.
use std::{io::Cursor,path::Path};
use glycin_utils::{FrameRequest,InitializationDetails,LoaderImplementation,LocalMemory,MemoryFormat,ProcessError};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};

const MAX_PIXELS:u64=16_000_000;
fn ok(w:u32,h:u32,mut rgba:Vec<u8>,orig:&str,extra:Value,dump:Option<&str>)->Value{
    if w==0||h==0||w as u64*h as u64>MAX_PIXELS||rgba.len()!=w as usize*h as usize*4{return err("output dimensions outside harness limit")}
    for p in rgba.chunks_exact_mut(4){if p[3]==0{p[0]=0;p[1]=0;p[2]=0}} // canonical RGB under fully transparent pixels
    if let Some(p)=dump{if let Err(e)=std::fs::write(p,&rgba){return err(e)}}
    json!({"status":"ok","width":w,"height":h,"orig_color_type":orig,"alpha_present":rgba.chunks_exact(4).any(|p|p[3]!=255),
           "rgba_sha256":format!("{:x}",Sha256::digest(&rgba)),"hotspot":extra})
}
fn err(e:impl std::fmt::Display)->Value{
    let s=e.to_string();
    json!({"status":"error","error_class":s.split([':','(','{']).next().unwrap_or("error").trim().chars().take(60).collect::<String>(),"message":s.chars().take(300).collect::<String>()})
}
fn glycin_error(e:ProcessError)->Value{
    let class=match &e{ProcessError::ExpectedError{..}=>"ExpectedError",ProcessError::InternalError{..}=>"InternalError",
        ProcessError::UnsupportedImageFormat(_)=>"UnsupportedImageFormat",ProcessError::OutOfMemory{..}=>"OutOfMemory",_=>"OtherProcessError"};
    json!({"status":"error","error_class":class,"message":e.to_string().chars().take(300).collect::<String>()})
}
fn frame_rgba(frame:glycin_utils::Frame<LocalMemory>,dump:Option<&str>)->Value{
    let (w,h)=(frame.width,frame.height);
    if w==0||h==0||w as u64*h as u64>MAX_PIXELS{return err("output dimensions outside harness limit")}
    let stride=frame.stride as usize;let fmt=frame.memory_format;let mut rgba=Vec::with_capacity(w as usize*h as usize*4);
    for row in frame.texture.chunks(stride).take(h as usize){
        match fmt{
            MemoryFormat::R8g8b8a8 if row.len()>=w as usize*4=>rgba.extend_from_slice(&row[..w as usize*4]),
            MemoryFormat::B8g8r8a8 if row.len()>=w as usize*4=>for p in row[..w as usize*4].chunks_exact(4){rgba.extend([p[2],p[1],p[0],p[3]])},
            MemoryFormat::R8g8b8 if row.len()>=w as usize*3=>for p in row[..w as usize*3].chunks_exact(3){rgba.extend([p[0],p[1],p[2],255])},
            MemoryFormat::G8 if row.len()>=w as usize=>for &g in &row[..w as usize]{rgba.extend([g,g,g,255])},
            MemoryFormat::G8a8 if row.len()>=w as usize*2=>for p in row[..w as usize*2].chunks_exact(2){rgba.extend([p[0],p[0],p[0],p[1]])},
            MemoryFormat::R16g16b16a16 if row.len()>=w as usize*8=>for p in row[..w as usize*8].chunks_exact(8){for c in p.chunks_exact(2){let v=u16::from_ne_bytes([c[0],c[1]]) as u32;rgba.push(((v*255+32767)/65535) as u8)}},
            MemoryFormat::R16g16b16 if row.len()>=w as usize*6=>for p in row[..w as usize*6].chunks_exact(6){for c in p.chunks_exact(2){let v=u16::from_ne_bytes([c[0],c[1]]) as u32;rgba.push(((v*255+32767)/65535) as u8)}rgba.push(255)},
            _=>return err(format!("unsupported output memory format {fmt:?}"))}
    }
    ok(w,h,rgba,&format!("{fmt:?}"),Value::Null,dump)
}
fn run(which:&str,bytes:&[u8],dump:Option<&str>)->Value{
    match which{
        "glycin"=>{
            let (mut l,_)=match glycin_image_rs::ImgLoader::load::<LocalMemory,_>(Cursor::new(bytes.to_vec()),"image/x-xbitmap".into(),InitializationDetails::default()){Ok(v)=>v,Err(e)=>return glycin_error(e)};
            match l.specific_frame::<LocalMemory>(FrameRequest::default()){Ok(f)=>frame_rgba(f,dump),Err(e)=>glycin_error(e)}
        },
        "image-extras"=>{
            use image::ImageDecoder;
            let d=match image_extras::xbm::XbmDecoder::new(Cursor::new(bytes)){Ok(d)=>d,Err(e)=>return err(e)};
            let (w,h)=d.dimensions();let hot=d.hotspot();
            if w as u64*h as u64>MAX_PIXELS{return err("output dimensions outside harness limit")}
            let o=format!("{:?}",d.original_color_type());
            match image::DynamicImage::from_decoder(d){Ok(im)=>ok(im.width(),im.height(),im.to_rgba8().into_raw(),&o,json!(hot),dump),Err(e)=>err(e)}
        },
        "crate"=>{
            let d=match xbm::Decoder::new(Cursor::new(bytes)){Ok(d)=>d,Err(e)=>return err(e)};
            let (w,h)=(d.width(),d.height());let hot=(d.x_hot(),d.y_hot());
            if w as u64*h as u64>MAX_PIXELS{return err("output dimensions outside harness limit")}
            match d.decode_to_vec(){Ok(v)=>{
                let mut rgba=Vec::with_capacity(v.len()*4);
                for b in v{let g=if b==0{255}else{0};rgba.extend([g,g,g,255])}
                ok(w,h,rgba,"L1",json!(hot),dump)},Err(e)=>err(e)}
        },
        _=>err("unknown selector")}
}
fn main(){
    let a:Vec<String>=std::env::args().collect();
    std::panic::set_hook(Box::new(|_|{}));
    let r=std::panic::catch_unwind(||{
        if a.len()<3{return err("usage: runner SELECTOR FILE [RGBA_DUMP]")}
        match std::fs::read(Path::new(&a[2])){Ok(b)=>run(&a[1],&b,a.get(3).map(String::as_str)),Err(e)=>err(e)}
    }).unwrap_or_else(|_|json!({"status":"error","error_class":"rust_panic","message":"caught panic in decoder"}));
    println!("{r}");
}
