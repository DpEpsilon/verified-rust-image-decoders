use std::{io::Cursor,path::Path};
use glycin_utils::{FrameRequest,InitializationDetails,LoaderImplementation,LocalMemory,MemoryFormat,ProcessError};
use image::{ImageDecoder,ImageFormat};
use embedded_graphics::{pixelcolor::Rgb888,prelude::RgbColor};
use serde_json::{json,Value};
use sha2::{Digest,Sha256};

fn ok(w:u32,h:u32,rgba:Vec<u8>,orig:String,selected:String,dump:Option<&str>)->Value{
    if w==0||h==0||w as u64*h as u64>16_000_000||rgba.len()!=w as usize*h as usize*4 {return err("invalid output length or dimensions")}
    if let Some(p)=dump {if let Err(e)=std::fs::write(p,&rgba){return err(e)}}
    json!({"status":"ok","width":w,"height":h,"orig_color_type":orig,"alpha_present":rgba.chunks_exact(4).any(|p|p[3]!=255),"rgba_sha256":format!("{:x}",Sha256::digest(&rgba)),"selection":selected})
}
fn err(e:impl std::fmt::Display)->Value{
    let s=e.to_string();json!({"status":"error","error_class":s.split([':', '(']).next().unwrap_or("error").chars().take(100).collect::<String>(),"message":s.chars().take(400).collect::<String>()})
}
fn glycin_error(e:ProcessError)->Value{
    let class=match &e{
        ProcessError::ExpectedError{..}=>"ExpectedError",
        ProcessError::InternalError{..}=>"InternalError",
        ProcessError::UnsupportedImageFormat(_)=>"UnsupportedImageFormat",
        ProcessError::ConversionTooLargerError=>"ConversionTooLarge",
        ProcessError::OutOfMemory{..}=>"OutOfMemory",
        ProcessError::NoMoreFrames=>"NoMoreFrames",
        _=>"OtherProcessError"
    };
    json!({"status":"error","error_class":class,"message":e.to_string().chars().take(400).collect::<String>()})
}
fn frame_result(frame:glycin_utils::Frame<LocalMemory>,sel:String,dump:Option<&str>)->Value{
    let (w,h)=(frame.width,frame.height);
    if w==0||h==0||w as u64*h as u64>16_000_000{return err("output dimensions outside harness limit")}
    let stride=frame.stride as usize;let fmt=frame.memory_format;
    let mut rgba=Vec::with_capacity(w as usize*h as usize*4);
    for row in frame.texture.chunks(stride).take(h as usize){
        match fmt{
            MemoryFormat::R8g8b8a8 if row.len()>=w as usize*4 => rgba.extend_from_slice(&row[..w as usize*4]),
            MemoryFormat::B8g8r8a8 if row.len()>=w as usize*4 => for p in row[..w as usize*4].chunks_exact(4){rgba.extend([p[2],p[1],p[0],p[3]])},
            MemoryFormat::R8g8b8 if row.len()>=w as usize*3 => for p in row[..w as usize*3].chunks_exact(3){rgba.extend([p[0],p[1],p[2],255])},
            MemoryFormat::G8 if row.len()>=w as usize => for &g in &row[..w as usize]{rgba.extend([g,g,g,255])},
            MemoryFormat::G8a8 if row.len()>=w as usize*2 => for p in row[..w as usize*2].chunks_exact(2){rgba.extend([p[0],p[0],p[0],p[1]])},
            _=>return err(format!("unsupported output memory format {fmt:?}"))
        }
    }
    ok(w,h,rgba,format!("{fmt:?}"),sel,dump)
}
fn run(which:&str,bytes:&[u8],dump:Option<&str>)->Value{
    let mut parts=which.split(':');let fmt=parts.next().unwrap_or("");let mode=parts.next().unwrap_or("");
    match (fmt,mode){
        ("tga","tiny")=>{
            let im=match tinytga::Tga::<Rgb888>::from_slice(bytes){Ok(v)=>v,Err(e)=>return err(format!("{e:?}"))};
            let size=im.as_raw().size();let (w,h)=(size.width,size.height);
            if w==0||h==0||w as u64*h as u64>16_000_000{return err("output dimensions outside harness limit")}
            let mut rgba=vec![0;w as usize*h as usize*4];let mut count=0;
            for embedded_graphics::Pixel(position,color) in im.pixels(){
                if position.x<0||position.y<0||position.x as u32>=w||position.y as u32>=h{return err("pixel position out of bounds")}
                let at=((position.y as usize)*w as usize+position.x as usize)*4;
                rgba[at..at+4].copy_from_slice(&[color.r(),color.g(),color.b(),255]);count+=1;
            }
            if count!=w as usize*h as usize{return err("truncated pixel iterator")}
            ok(w,h,rgba,"RGB888 (alpha unsupported)".into(),"single".into(),dump)
        },
        ("tga","verified")=>match tga_verus::tga::decode(bytes){Ok(d)=>ok(d.width,d.height,d.rgba,"RGBA8".into(),"single".into(),dump),Err(e)=>err(e.name())},
        ("icns","verified-legacy") | ("icns","verified-png")=>{
            let entries=match icns_verus::parse(bytes){Ok(x)=>x,Err(e)=>return err(e.name())};
            let ranked=glycin_icns::candidates(bytes,&entries);
            let candidate=ranked.into_iter().find(|c|matches!(c,glycin_icns::Candidate::Legacy{..})==(mode=="verified-legacy"));
            match candidate {
                Some(glycin_icns::Candidate::Legacy{color,mask,side})=>match icns_verus::combine(bytes,&entries,Some(color),Some(mask)){
                    Ok(d)=>ok(d.width,d.height,d.data,"RGBA8".into(),format!("legacy:{side}:{color}:{mask}"),dump),Err(e)=>err(e.name())},
                Some(glycin_icns::Candidate::Png{entry,..})=>{
                    let e=&entries[entry as usize];let p=&bytes[e.payload[0] as usize..e.payload[1] as usize];
                    image_result(p,ImageFormat::Png,format!("png:{entry}"),dump)
                },None=>err("NoCandidate")
            }
        },
        ("icns","crate")=>{
            let family=match icns::IconFamily::read(Cursor::new(bytes)){Ok(f)=>f,Err(e)=>return err(e)};
            let choice=family.available_icons().into_iter().max_by_key(|t|(t.pixel_width().max(t.pixel_height()),t.pixel_width()));
            match choice {Some(t)=>match family.get_icon_with_type(t){
                Ok(im)=>{let orig=format!("{:?}",im.pixel_format());let d=im.convert_to(icns::PixelFormat::RGBA);
                    ok(d.width(),d.height(),d.data().to_vec(),orig,format!("{:?}",t),dump)},Err(e)=>err(e)},None=>err("NoCandidate")}
        },
        ("ico","crate-first") | ("ico","crate-largest") | ("ico","crate-entry")=>{
            let dir=match ico::IconDir::read(Cursor::new(bytes)){Ok(d)=>d,Err(e)=>return err(e)};
            let entries=dir.entries();let chosen=match mode{
                "crate-first"=>Some(0),"crate-entry"=>parts.next().and_then(|s|s.parse::<usize>().ok()),
                _=>entries.iter().enumerate().max_by_key(|(_,e)|((e.width() as u64)*(e.height() as u64),e.bits_per_pixel())).map(|(i,_)|i)};
            let Some(i)=chosen.filter(|&i|i<entries.len()) else{return err("NoEntry")};
            match entries[i].decode(){Ok(im)=>ok(im.width(),im.height(),im.rgba_data().to_vec(),"RGBA8".into(),format!("entry:{i}:{}",if entries[i].is_png(){"png"}else{"dib"}),dump),Err(e)=>err(e)}
        },
        (_,"image")=>{
            let format=match fmt{"tga"=>ImageFormat::Tga,"ico"=>ImageFormat::Ico,_=>return err("unsupported image format")};
            image_result(bytes,format,"library-policy".into(),dump)
        },
        (_,"glycin")=>{
            let mime=match fmt{"tga"=>"image/x-tga","ico"=>"image/vnd.microsoft.icon","icns"=>"image/x-icns",_=>return err("unsupported mime")};
            if fmt=="icns"{
                let (mut loader,_)=match glycin_icns::ImgLoader::load::<LocalMemory,_>(Cursor::new(bytes.to_vec()),mime.into(),InitializationDetails::default()) {Ok(v)=>v,Err(e)=>return glycin_error(e)};
                let source=format!("{:?}",loader.source());
                match loader.specific_frame::<LocalMemory>(FrameRequest::default()){Ok(f)=>frame_result(f,source,dump),Err(e)=>glycin_error(e)}
            }else{
                let (mut loader,_)=match glycin_image_rs::ImgLoader::load::<LocalMemory,_>(Cursor::new(bytes.to_vec()),mime.into(),InitializationDetails::default()){Ok(v)=>v,Err(e)=>return glycin_error(e)};
                match loader.specific_frame::<LocalMemory>(FrameRequest::default()){Ok(f)=>frame_result(f,"library-policy".into(),dump),Err(e)=>glycin_error(e)}
            }
        },
        _=>err("unknown selector")
    }
}
fn image_result(bytes:&[u8],fmt:ImageFormat,selection:String,dump:Option<&str>)->Value{
    let orig=match fmt{
        ImageFormat::Tga=>match image::codecs::tga::TgaDecoder::new(Cursor::new(bytes)){Ok(d)=>format!("{:?}",d.color_type()),Err(e)=>return err(e)},
        ImageFormat::Ico=>match image::codecs::ico::IcoDecoder::new(Cursor::new(bytes)){Ok(d)=>format!("{:?}",d.color_type()),Err(e)=>return err(e)},
        _=>match image::codecs::png::PngDecoder::new(Cursor::new(bytes)){Ok(d)=>format!("{:?}",d.color_type()),Err(e)=>return err(e)}
    };
    match image::load_from_memory_with_format(bytes,fmt){Ok(im)=>ok(im.width(),im.height(),im.to_rgba8().into_raw(),orig,selection,dump),Err(e)=>err(e)}
}
fn main(){let a:Vec<String>=std::env::args().collect();let result=std::panic::catch_unwind(||{
    if a.len()<3{return err("usage: runner FORMAT:SELECTOR FILE [RGBA_DUMP]")}
    match std::fs::read(Path::new(&a[2])){Ok(b)=>run(&a[1],&b,a.get(3).map(String::as_str)),Err(e)=>err(e)}
}).unwrap_or_else(|_|json!({"status":"crash","error_class":"rust_panic"}));println!("{result}");}
