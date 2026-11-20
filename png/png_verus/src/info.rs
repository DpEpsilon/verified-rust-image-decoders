//! Checked fixed-size Info representation and raw-value storage. Payload
//! descriptors remain borrowed ranges; this layer performs no allocation.
use vstd::prelude::*;
use crate::{png::{Info,Meta,Time}, metadata::Value};
#[cfg(verus_only)]
use crate::{document_model as m, metadata_model as values, walk_model::{Name,Range}};

verus! {

pub(crate) open spec fn slot<T>(value: Meta<T>, f: spec_fn(T)->m::Stored) -> m::Slot<m::Stored> {
    match value {Meta::Absent=>m::Slot::Absent,Meta::Discarded=>m::Slot::Discarded,Meta::Present(v)=>m::Slot::Present(f(v))}
}
impl Info {
    pub closed spec fn model_value(self) -> m::Info {self.view()}
    pub closed spec fn ranges_within(self,input_len: nat) -> bool {self.ranges_wf(input_len)}
    pub(crate) proof fn public_observations(self,input_len: nat)
        ensures self.model_value()==self.view(),self.ranges_within(input_len)==self.ranges_wf(input_len),
    {}
    /// sBIT is the only stored array with a runtime prefix count. bKGD's
    /// prefix is derived from color_type and is always one or three samples.
    pub(crate) open spec fn bkgd_count(self) -> nat {
        if self.color_type==2 || self.color_type==6 {3nat}else{1nat}
    }
    pub(crate) open spec fn repr_wf(self) -> bool {
        (match self.sbit {Meta::Present((_,n))=>n<=4,_=>true})
        && 1<=self.bkgd_count()<=3
    }
    pub(crate) proof fn prefix_domains(self)
        requires self.repr_wf(),
        ensures 1<=self.bkgd_count()<=3,
            match self.sbit {Meta::Present((v,n))=>v@.subrange(0,n as int).len()==n,_=>true},
    {}
    /// A concrete field update; every other metadata field is unchanged.
    pub(crate) open spec fn with_plays(self, plays: Option<u32>) -> Self {
        Self {num_plays:plays,..self}
    }
    pub(crate) fn set_num_plays(&mut self, plays: Option<u32>)
        ensures *final(self)==old(self).with_plays(plays),
    {self.num_plays=plays;}

    pub(crate) proof fn plays_view(before: Self, plays: Option<u32>)
        ensures before.with_plays(plays).view()==(m::Info {num_plays:plays,..before.view()}),
            forall|n: nat| before.with_plays(plays).ranges_wf(n)==before.ranges_wf(n),
            before.with_plays(plays).repr_wf()==before.repr_wf(),
    {
        assert(before.with_plays(plays).view().slots =~= before.view().slots);
    }
    pub(crate) open spec fn ranges_wf(self, input_len: nat) -> bool {
        (match self.iccp_name {Meta::Present((p,n))=>p+n<=input_len,_=>true})
        && (match self.exif {Meta::Present((p,n))=>p+n<=input_len,_=>true})
    }
    /// L-INFO-RANGE for both stored byte-range fields, in subtraction form.
    pub(crate) proof fn present_ranges(self,input_len: nat)
        requires self.ranges_wf(input_len),
        ensures match self.iccp_name {
            Meta::Present((p,n))=>p<=input_len && n<=input_len-p
                && self.view().slots[Name::Iccp]==m::Slot::Present(m::Stored::Bytes(Range {offset:p as nat,len:n as nat})),
            _=>true,
        },match self.exif {
            Meta::Present((p,n))=>p<=input_len && n<=input_len-p
                && self.view().slots[Name::Exif]==m::Slot::Present(m::Stored::Bytes(Range {offset:p as nat,len:n as nat})),
            _=>true,
        },
    {}
    pub(crate) open spec fn view(self) -> m::Info {
        m::Info {header:crate::model::Ihdr {width:self.width,height:self.height,depth:self.bit_depth,
            color_type:self.color_type,interlace:self.interlace},palette_entries:self.palette_entries as nat,
            num_plays:self.num_plays,frame_count:self.frame_count as nat,default_is_frame:self.default_is_frame,
            slots:map![
                Name::Trns=>slot(self.trns,|v: ()|m::Stored::Unit),
                Name::Gama=>slot(self.gama,|v: u32|m::Stored::Raw(values::Value::Gama(v))),
                Name::Chrm=>slot(self.chrm,|v: [u32;8]|m::Stored::Raw(values::Value::Chrm(v@))),
                Name::Srgb=>slot(self.srgb,|v: u8|m::Stored::Raw(values::Value::Srgb(v))),
                Name::Iccp=>slot(self.iccp_name,|v: (usize,usize)|m::Stored::Bytes(Range {offset:v.0 as nat,len:v.1 as nat})),
                Name::Sbit=>slot(self.sbit,|v: ([u8;4],u8)|m::Stored::Raw(values::Value::Sbit(v.0@.subrange(0,v.1 as int)))),
                Name::Cicp=>slot(self.cicp,|v: [u8;4]|m::Stored::Raw(values::Value::Cicp(v@))),
                Name::Mdcv=>slot(self.mdcv,|v: [u8;24]|m::Stored::Raw(values::Value::Mdcv(v@))),
                Name::Clli=>slot(self.clli,|v: [u32;2]|m::Stored::Raw(values::Value::Clli(v[0],v[1]))),
                Name::Bkgd=>slot(self.bkgd,|v: [u16;3]|m::Stored::Raw(values::Value::Bkgd(v@.subrange(0,self.bkgd_count() as int)))),
                Name::Phys=>slot(self.phys,|v: (u32,u32,u8)|m::Stored::Raw(values::Value::Phys(v.0,v.1,v.2))),
                Name::Exif=>slot(self.exif,|v: (usize,usize)|m::Stored::Bytes(Range {offset:v.0 as nat,len:v.1 as nat})),
                Name::Time=>slot(self.time,|v: Time|m::Stored::Raw(values::Value::Time(v.year,v.month,v.day,v.hour,v.minute,v.second))),
            ]}
    }
}

/// Explicit representation premise for bKGD: Info's sample count comes from
/// its colour type, whereas the parser's fixed array carries a count.
pub(crate) open spec fn compatible(info: Info, value: Value) -> bool {
    match value {Value::Bkgd(_,n)=>n==if info.color_type==2 || info.color_type==6 {3int}else{1int},_=>true}
}

pub(crate) fn store(info: &mut Info, b: &[u8], base: usize, size: usize, value: Value)
    -> (profile: Option<(usize,usize)>)
    requires base<=b.len(),size<=b.len()-base,value.wf(size as nat),compatible(*old(info),value),
    ensures old(info).ranges_wf(b.len() as nat) ==> final(info).ranges_wf(b.len() as nat),
        old(info).repr_wf() ==> final(info).repr_wf(),
    {
        let before=m::Doc {ihdr:old(info).view().header,plte:None,trns:None,iccp:None,actl:None,
            info:old(info).view(),idat:Seq::empty(),frames:Seq::empty()};
        let after=m::store(before,value.view(),base as nat);
        final(info).view()==after.info && match profile {
            Some((start,count))=>after.iccp==Some(Range {offset:start as nat,len:count as nat})
                && start<=b.len() && count<=b.len()-start,
            None=>after.iccp.is_none(),
        }
    },
{
    let ghost original=*info;
    let mut profile=None;
    match value {
        Value::Ignored=>{},
        Value::Gama(g)=>info.gama=Meta::Present(g),
        Value::Chrm(v)=>info.chrm=Meta::Present(v),
        Value::Srgb(v)=>info.srgb=Meta::Present(v),
        Value::Iccp {name_len,profile_start,profile_len}=>{
            let range=crate::metadata::rebase(b,base,size,profile_start,profile_len);
            info.iccp_name=Meta::Present(crate::metadata::rebase(b,base,size,0,name_len));
            profile=Some(range);
        },
        Value::Sbit(v,n)=>info.sbit=Meta::Present((v,n)),
        Value::Cicp(v)=>info.cicp=Meta::Present(v),
        Value::Mdcv(v)=>info.mdcv=Meta::Present(v),
        Value::Clli(a,b)=>info.clli=Meta::Present([a,b]),
        Value::Bkgd(v,_)=>info.bkgd=Meta::Present(v),
        Value::Phys(x,y,u)=>info.phys=Meta::Present((x,y,u)),
        Value::Exif(n)=>info.exif=Meta::Present(crate::metadata::rebase(b,base,size,0,n)),
        Value::Time(year,month,day,hour,minute,second)=>info.time=Meta::Present(Time {year,month,day,hour,minute,second}),
    }
    proof {
        let before=m::Doc {ihdr:original.view().header,plte:None,trns:None,iccp:None,actl:None,
            info:original.view(),idat:Seq::empty(),frames:Seq::empty()};
        assert(info.view().slots =~= m::store(before,value.view(),base as nat).info.slots);
    }
    profile
}

pub(crate) proof fn parsed_compatible(info: Info, value: Value, name: Name, data: Seq<u8>, entries: nat)
    requires crate::model::valid_depth(info.color_type,info.bit_depth),entries<=256,
        value.wf(data.len()),values::parse(name,data,info.color_type,info.bit_depth,entries)==values::Outcome::Value(value.view()),
    ensures compatible(info,value),
{
    match value {
        Value::Bkgd(v,n)=>{
            assert(name==Name::Bkgd);
            assert(v@.subrange(0,n as int).len()==n);
            assert(data.len()==values::bkgd_length(info.color_type));
            if info.color_type==3 {assert(n==1);} else {
                assert(v@.subrange(0,n as int)==values::words16(data,data.len()/2));
                assert(n==data.len()/2);
            }
        },_=>{}
    }
}

pub(crate) fn parse_and_store(info: &mut Info, kind: crate::kind::Kind,
    b: &[u8], base: usize, size: usize, entries: usize) -> (r: Result<Option<(usize,usize)>,crate::png::Error>)
    requires base<=b.len(),size<=b.len()-base,entries<=256,
        crate::model::valid_depth(old(info).color_type,old(info).bit_depth),
    ensures old(info).ranges_wf(b.len() as nat) ==> final(info).ranges_wf(b.len() as nat),
        old(info).repr_wf() ==> final(info).repr_wf(),
    {
        let parsed=values::parse(kind.view(),b@.subrange(base as int,base as int+size as int),
            old(info).color_type,old(info).bit_depth,entries as nat);
        match r {
            Err(e)=>*final(info)==*old(info) && parsed==values::Outcome::Error(e),
            Ok(profile)=>exists|value: values::Value| parsed==values::Outcome::Value(value) && {
                let before=m::Doc {ihdr:old(info).view().header,plte:None,trns:None,iccp:None,actl:None,
                    info:old(info).view(),idat:Seq::empty(),frames:Seq::empty()};
                let after=m::store(before,value,base as nat);
                final(info).view()==after.info && match profile {
                    None=>after.iccp.is_none(),Some((start,count))=>after.iccp==Some(Range {offset:start as nat,len:count as nat})
                        && start<=b.len() && count<=b.len()-start,
                }
            },
        }
    },
{
    let d=&b[base..base+size];
    let value=match crate::metadata::parse(kind,d,info.color_type,info.bit_depth,entries) {
        Err(e)=>return Err(e),Ok(v)=>v,
    };
    proof {parsed_compatible(*info,value,kind.view(),d@,entries as nat);}
    let r=store(info,b,base,size,value);
    proof {
        assert(values::parse(kind.view(),d@,old(info).color_type,old(info).bit_depth,entries as nat)
            == values::Outcome::Value(value.view()));
    }
    Ok(r)
}

pub(crate) fn discard(info: &mut Info, kind: crate::kind::Kind)
    ensures forall|n: nat| old(info).ranges_wf(n) ==> final(info).ranges_wf(n),
        old(info).repr_wf() ==> final(info).repr_wf(),
        final(info).view()==(m::Info {slots:if m::metadata_names().contains(kind.view()) {
        old(info).view().slots.insert(kind.view(),m::discarded(old(info).view().slots[kind.view()]))
    }else{old(info).view().slots},..old(info).view()}),
{
    use crate::kind::Kind;
    let ghost before=*info;
    match kind {
        Kind::Trns=>info.trns.discard(),Kind::Gama=>info.gama.discard(),Kind::Chrm=>info.chrm.discard(),
        Kind::Srgb=>info.srgb.discard(),Kind::Iccp=>info.iccp_name.discard(),Kind::Sbit=>info.sbit.discard(),
        Kind::Cicp=>info.cicp.discard(),Kind::Mdcv=>info.mdcv.discard(),Kind::Clli=>info.clli.discard(),
        Kind::Bkgd=>info.bkgd.discard(),Kind::Phys=>info.phys.discard(),Kind::Exif=>info.exif.discard(),
        Kind::Time=>info.time.discard(),_=>{},
    }
    proof {
        let slots=before.view().slots;
        assert(info.view().slots =~= if m::metadata_names().contains(kind.view()) {
            slots.insert(kind.view(),m::discarded(slots[kind.view()]))
        }else{slots});
    }
}

} // verus!
