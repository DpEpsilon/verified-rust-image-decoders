from pathlib import Path
p=Path('/home/ws/projects/verified-decoders/gif/gif_verus/src/pixel_model.rs');s=p.read_text()
a='''pub open spec fn active_tables(b:Seq<u8>,h:HeaderM,p:Policy)->Set<Seq<u8>> {
    Set::new(|t:Seq<u8>| exists|k:nat| #[trigger] table_at(b,h,p,k)==Some(t))
}''';assert s.count(a)==1;s=s.replace(a,'''pub open spec fn active_tables(b:Seq<u8>,h:HeaderM,p:Policy,k:nat)->Set<Seq<u8>>
    decreases k,
{
    if k==0 {Set::empty()} else {
        let prev=active_tables(b,h,p,(k-1) as nat);
        match table_at(b,h,p,(k-1) as nat) {Some(t)=>prev.insert(t),None=>prev}
    }
}''')
s=s.replace('active_tables(b,h,p))','active_tables(b,h,p,k))')
s=s.replace('let ts=active_tables(b,h,p);','let ts=active_tables(b,h,p,k);',1)
a='''        let s=crate::frame_model::after_calls(b,h,p,(k-1) as nat).unwrap();''';assert s.count(a)==1;s=s.replace(a,a+'''
        let prev=active_tables(b,h,p,(k-1) as nat);
        assert forall|t:Seq<u8>| #[trigger] prev.contains(t) implies ts.contains(t) by {};
        lemma_state_pixels_subset(s,h.width as u32,h.height as u32,prev,ts);''')
s=s.replace('active_tables(b,crate::model::mh(b).unwrap(),p))','active_tables(b,crate::model::mh(b).unwrap(),p,1))')
s=s.replace('let ts=active_tables(b,h,p);','let ts=active_tables(b,h,p,1);')
s=s.replace('        lemma_after_pixels(b,h,p,0);','        crate::frame_model::lemma_initial_domain(b,p);\n        lemma_initial_pixels(b,h,p,ts);')
pos=s.index('proof fn lemma_disposed_pixels(')
s=s[:pos]+'''proof fn lemma_pixels_subset(cv:Seq<u8>,small:Set<Seq<u8>>,big:Set<Seq<u8>>)
    requires pixels_well_formed(cv,small),forall|t:Seq<u8>| #[trigger] small.contains(t) ==> big.contains(t),
    ensures pixels_well_formed(cv,big),
{
    assert forall|i:int| 0<=i<cv.len()/4 implies pixel_is_clear(cv,i) ||
        exists|t:Seq<u8>,v:int| big.contains(t) && 0<=v && 3*v+2<t.len() && #[trigger] pixel_is_entry(cv,t,i,v) by {
        if !pixel_is_clear(cv,i) {
            let (t,v)=choose|t:Seq<u8>,v:int| small.contains(t) && 0<=v && 3*v+2<t.len() && pixel_is_entry(cv,t,i,v);
            assert(big.contains(t));
        }
    }
}
proof fn lemma_state_pixels_subset(s:crate::frame_model::State,sw:u32,sh:u32,small:Set<Seq<u8>>,big:Set<Seq<u8>>)
    requires state_pixels(s,sw,sh,small),forall|t:Seq<u8>| #[trigger] small.contains(t) ==> big.contains(t),
    ensures state_pixels(s,sw,sh,big),
{
    lemma_pixels_subset(s.canvas,small,big);
    if s.saved is Some {lemma_pixels_subset(s.saved.unwrap(),small,big);}
}
'''+s[pos:];p.write_text(s)
print('Made active-table histories finite by construction and added pixel-set monotonicity')
