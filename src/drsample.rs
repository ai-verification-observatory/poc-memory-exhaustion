//! Algorithm 1, ePrint 2017/443. Internal node indices are zero based.
use super::{Hash, tagged};
use std::collections::HashMap;
pub fn uniform(seed:&Hash,node:usize,domain:u8,lo:u64,hi:u64)->u64 {
 let span=hi-lo+1;let threshold=span.wrapping_neg()%span;
 for counter in 0u64.. {let h=tagged(domain,&[seed,&(node as u64).to_le_bytes(),&counter.to_le_bytes()]);let x=u64::from_le_bytes(h[..8].try_into().unwrap());if x>=threshold{return lo+x%span}}
 unreachable!()
}
pub fn parents(seed:&Hash,node:usize)->Vec<usize>{
 if node==0{return vec![]}if node==1{return vec![0]}
 let v=node as u64+1;
 let exponent=uniform(seed,node,40,1,(63-v.leading_zeros())as u64+1);
 let g=(v-1).min(1u64.checked_shl(exponent as u32).unwrap_or(u64::MAX));
 let r=uniform(seed,node,41,(g/2+g%2).max(2),g);
 vec![node-1,(v-r-1)as usize]
}
fn label(seed:&Hash,i:usize,values:&[Hash])->Hash{let mut data=Vec::with_capacity(32+8+64);data.extend_from_slice(seed);data.extend_from_slice(&(i as u64).to_le_bytes());for p in values{data.extend_from_slice(p)}tagged(42,&[&data])}
pub fn full(seed:&Hash,n:usize)->Result<Hash,String>{if n<2{return Err("nodes must be >=2".into())}let mut labels=Vec::new();labels.try_reserve_exact(n).map_err(|e|e.to_string())?;for i in 0..n{let values:Vec<_>=parents(seed,i).iter().map(|p|labels[*p]).collect();labels.push(label(seed,i,&values))}Ok(labels[n-1])}
// Sequential last-use greedy: compute every node once, discard after final use.
// Metadata is additional storage, not charged as retained labels.
pub fn greedy(seed:&Hash,n:usize)->Result<(Hash,usize),String>{
 if n<2{return Err("nodes must be >=2".into())}
 let mut last=Vec::new();last.try_reserve_exact(n).map_err(|e|e.to_string())?;last.resize(n,0usize);
 for i in 0..n{for p in parents(seed,i){last[p]=i}}
 let mut live=HashMap::new();let mut peak=0;let mut peak_capacity=0;
 for i in 0..n{let p=parents(seed,i);let values:Vec<_>=p.iter().map(|j|live[j]).collect();let value=label(seed,i,&values);live.try_reserve(1).map_err(|e|e.to_string())?;live.insert(i,value);peak=peak.max(live.len());peak_capacity=peak_capacity.max(live.capacity());for j in p{if last[j]==i{live.remove(&j);}}}
 eprintln!("hashmap_peak_entry_capacity={peak_capacity} metadata_allocation_bytes={}",last.capacity()*std::mem::size_of::<usize>());
 Ok((live[&(n-1)],peak))
}
#[test]fn boundaries(){for s in 0..8{let seed=[s;32];assert!(parents(&seed,0).is_empty());assert_eq!(parents(&seed,1),vec![0]);for i in 2..1024{let p=parents(&seed,i);assert_eq!(p[0],i-1);assert!(p[1]<i-1)}}}
#[test]fn greedy_agrees(){for n in [2,3,16,127]{let seed=[3;32];let(h,peak)=greedy(&seed,n).unwrap();assert_eq!(h,full(&seed,n).unwrap());assert!(peak<=n)}}
