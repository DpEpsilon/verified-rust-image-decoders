//! T10: operational error locations, not an ordering of error names.
use vstd::prelude::*;
use crate::{raster_model as r,icc_model as icc,png::Error,policy::Policy};
use inflate_verus::model::InflateStatus;
verus! {

pub enum DataPoint {Inflate,Length,Row(nat,nat)}
/// Earlier passes and rows succeeded; the current row checks its filter first.
pub open spec fn first_row(input: Seq<u8>,a: r::Raster,k: nat,row: nat,e: Error) -> bool {
    k<r::count(a) && r::pw(a,k)>0 && row<r::ph(a,k)
    && (forall|p: nat| p<k ==> #[trigger] r::pass_result(input,a,p,0).is_ok())
    && (forall|y: nat| y<row ==> #[trigger] r::row_result(input,a,k,y)==Ok(()))
    && r::row_result(input,a,k,row)==Err(e)
}
pub open spec fn data_at(z: Seq<u8>,a: r::Raster,pol: Policy,point: DataPoint,e: Error) -> bool {
    let i=r::inflated(z,a,pol);
    match point {
        DataPoint::Inflate=>match i.status {
            InflateStatus::Error(f)=>e==icc::fault(f),
            InflateStatus::Full=>e==Error::ExcessImageData,
            _=>false,
        },
        DataPoint::Length=>i.status==InflateStatus::Done && i.out.len()<r::raw(a)
            && e==Error::TruncatedImageData,
        DataPoint::Row(k,y)=>i.status==InflateStatus::Done && i.out.len()>=r::raw(a)
            && first_row(i.out,a,k,y,e),
    }
}

pub proof fn row_first(input: Seq<u8>,a: r::Raster,k: nat,start: nat,e: Error) -> (row: nat)
    requires r::pass_result(input,a,k,start)==Err(e),
    ensures start<=row<r::ph(a,k),r::pw(a,k)>0,r::row_result(input,a,k,row)==Err(e),
        forall|y: nat| start<=y<row ==> #[trigger] r::row_result(input,a,k,y)==Ok(()),
    decreases r::ph(a,k)-start,
{
    r::pass_step(input,a,k,start);
    if r::row_result(input,a,k,start).is_err() {start}
    else {
        let row=row_first(input,a,k,start+1,e);
        assert forall|y: nat| start<=y<row implies #[trigger] r::row_result(input,a,k,y)==Ok(()) by {
            if y==start {} else {assert(start+1<=y);}
        }
        row
    }
}
pub proof fn scan_first(input: Seq<u8>,a: r::Raster,start: nat,e: Error) -> (at: (nat,nat))
    requires r::scan(input,a,start)==Err(e),
    ensures start<=at.0<r::count(a),r::pw(a,at.0)>0,at.1<r::ph(a,at.0),
        r::row_result(input,a,at.0,at.1)==Err(e),
        forall|p: nat| start<=p<at.0 ==> #[trigger] r::pass_result(input,a,p,0).is_ok(),
        forall|y: nat| y<at.1 ==> #[trigger] r::row_result(input,a,at.0,y)==Ok(()),
    decreases r::count(a)-start,
{
    r::scan_step(input,a,start);
    if r::pass_result(input,a,start,0).is_err() {
        let row=row_first(input,a,start,0,e);(start,row)
    } else {
        assert(r::pass_result(input,a,start,0).is_ok());
        let at=scan_first(input,a,start+1,e);
        assert forall|p: nat| start<=p<at.0 implies #[trigger] r::pass_result(input,a,p,0).is_ok() by {
            if p==start {assert(r::pass_result(input,a,start,0).is_ok());}
            else {assert(start+1<=p);assert(r::pass_result(input,a,p,0).is_ok());}
        }
        at
    }
}
pub proof fn row_selected(input: Seq<u8>,a: r::Raster,k: nat,start: nat,row: nat,e: Error)
    requires r::pw(a,k)>0,start<=row<r::ph(a,k),r::row_result(input,a,k,row)==Err(e),
        forall|y: nat| start<=y<row ==> #[trigger] r::row_result(input,a,k,y)==Ok(()),
    ensures r::pass_result(input,a,k,start)==Err(e),
    decreases row-start,
{
    r::pass_step(input,a,k,start);
    if start<row {row_selected(input,a,k,start+1,row,e);}
}
pub proof fn scan_selected(input: Seq<u8>,a: r::Raster,start: nat,k: nat,row: nat,e: Error)
    requires start<=k,first_row(input,a,k,row,e),
    ensures r::scan(input,a,start)==Err(e),
    decreases k-start,
{
    r::scan_step(input,a,start);
    if start==k {row_selected(input,a,k,0,row,e);}
    else {
        assert(r::pass_result(input,a,start,0).is_ok());
        scan_selected(input,a,start+1,k,row,e);
        assert(r::scan(input,a,start+1)==Err(e));
        assert(r::chain(r::pass_result(input,a,start,0),r::scan(input,a,start+1))==Err(e));
    }
}
/// Every data error carries the first failing operational point.
pub proof fn data_first(z: Seq<u8>,a: r::Raster,pol: Policy,e: Error) -> (point: DataPoint)
    requires r::data(z,a,pol)==Some(Err(e)),
    ensures data_at(z,a,pol,point,e),
{
    hide(inflate_verus::model::Inflate);
    let i=r::inflated(z,a,pol);
    match i.status {
        InflateStatus::Done=>if i.out.len()<r::raw(a) {DataPoint::Length}
            else {let at=scan_first(i.out,a,0,e);DataPoint::Row(at.0,at.1)},
        _=>DataPoint::Inflate,
    }
}
/// The converse prevents a later failing check from masquerading as the first.
pub proof fn data_selected(z: Seq<u8>,a: r::Raster,pol: Policy,point: DataPoint,e: Error)
    requires data_at(z,a,pol,point,e),
    ensures r::data(z,a,pol)==Some(Err(e)),
{
    hide(inflate_verus::model::Inflate);
    if let DataPoint::Row(k,y)=point {scan_selected(r::inflated(z,a,pol).out,a,0,k,y,e);}
}
pub proof fn data_precedence(z: Seq<u8>,a: r::Raster,pol: Policy,e: Error)
    ensures (r::data(z,a,pol)==Some(Err(e)))
        == (exists|point: DataPoint| data_at(z,a,pol,point,e)),
{
    hide(inflate_verus::model::Inflate);
    if r::data(z,a,pol)==Some(Err(e)) {let point=data_first(z,a,pol,e);}
    if exists|point: DataPoint| data_at(z,a,pol,point,e) {
        let point=choose|point: DataPoint| data_at(z,a,pol,point,e);
        data_selected(z,a,pol,point,e);
    }
}
pub proof fn row_error_kind(input: Seq<u8>,a: r::Raster,k: nat,row: nat,e: Error)
    requires r::row_result(input,a,k,row)==Err(e),
    ensures e==Error::InvalidFilterType || e==Error::PaletteIndexOutOfRange,
        (e==Error::PaletteIndexOutOfRange ==> input[crate::filter_model::position(r::base(a,k),r::rb(a,k),row) as int]<=4),
{}
pub proof fn data_not_profile_cap(z: Seq<u8>,a: r::Raster,pol: Policy,e: Error)
    requires r::data(z,a,pol)==Some(Err(e)),
    ensures e!=Error::IccProfileTooLarge,
{
    hide(inflate_verus::model::Inflate);
    let point=data_first(z,a,pol,e);
    match point {
        DataPoint::Row(k,y)=>row_error_kind(r::inflated(z,a,pol).out,a,k,y,e),
        DataPoint::Inflate=>if let InflateStatus::Error(f)=r::inflated(z,a,pol).status {icc::fault_not_profile_cap(f);},
        _=>{},
    }
}
}
