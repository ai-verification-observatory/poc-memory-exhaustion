//! Sequential, recomputation-permitting two-tier game; not a parallel-ROM reduction.
use std::collections::{BinaryHeap, HashMap};
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Move { Compute(usize), Load(usize), Store(usize), DeleteRed(usize), DeleteBlue(usize) }
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct State { pub red: u64, pub blue: u64 }
#[derive(Clone, Copy, Debug)]
pub struct Costs { pub compute: u64, pub transfer: u64 }
pub struct Game { pub parents: Vec<u64>, pub capacity: u32, pub sink: usize }
impl Game {
 pub fn validate(&self)->Result<(),String>{
  if self.parents.is_empty() || self.parents.len()>63 || self.sink>=self.parents.len(){return Err("invalid graph size or sink".into())}
  for (i,p) in self.parents.iter().enumerate(){if *p >= (1u64<<i){return Err("parents must precede node".into())}}
  Ok(())
 }
 pub fn step(&self,s:State,m:Move,c:Costs)->Result<(State,u64),String>{
  self.validate()?;if c.compute==0 || c.transfer==0{return Err("costs must be positive".into())}
  let i=match m{Move::Compute(i)|Move::Load(i)|Move::Store(i)|Move::DeleteRed(i)|Move::DeleteBlue(i)=>i};
  if i>=self.parents.len(){return Err("node out of range".into())}let bit=1u64<<i;let mut t=s;
  if (s.red|s.blue)>>self.parents.len()!=0 || s.red.count_ones()>self.capacity{return Err("invalid state".into())}
  let cost=match m {
   Move::Compute(_)=>{if s.red&bit!=0 || s.red&self.parents[i]!=self.parents[i]{return Err("compute precondition".into())}t.red|=bit;c.compute},
   Move::Load(_)=>{if s.blue&bit==0 || s.red&bit!=0{return Err("load precondition".into())}t.red|=bit;c.transfer},
   Move::Store(_)=>{if s.red&bit==0 || s.blue&bit!=0{return Err("store precondition".into())}t.blue|=bit;c.transfer},
   Move::DeleteRed(_)=>{if s.red&bit==0{return Err("missing red".into())}t.red&=!bit;0},
   Move::DeleteBlue(_)=>{if s.blue&bit==0{return Err("missing blue".into())}t.blue&=!bit;0}
  };if t.red.count_ones()>self.capacity{return Err("red capacity exceeded".into())}Ok((t,cost))
 }
 pub fn replay(&self,trace:&[Move],c:Costs)->Result<(State,u64),String>{let mut s=State::default();let mut total=0u64;for m in trace{let(t,cost)=self.step(s,*m,c)?;total=total.checked_add(cost).ok_or("cost overflow")?;s=t}if s.red&(1<<self.sink)==0{return Err("sink absent".into())}Ok((s,total))}
}
#[derive(Debug)]
pub enum Solution { Optimal{cost:u64,trace:Vec<Move>,expanded:usize}, Incomplete{expanded:usize}, Impossible }
pub fn solve(g:&Game,c:Costs,limit:usize)->Result<Solution,String>{
 g.validate()?;if c.compute==0||c.transfer==0{return Err("costs must be positive".into())}
 if g.parents.len()>12{return Err("exact search limited to 12 nodes".into())}
 let start=State::default();let mut queue=BinaryHeap::new();let mut dist=HashMap::new();let mut prev=HashMap::new();dist.insert(start,0u64);queue.push(std::cmp::Reverse((0u64,start)));let mut expanded=0;
 while let Some(std::cmp::Reverse((cost,s)))=queue.pop(){if dist[&s]!=cost{continue}if s.red&(1<<g.sink)!=0{let mut trace=vec![];let mut t=s;while t!=start{let(p,m)=prev[&t];trace.push(m);t=p}trace.reverse();return Ok(Solution::Optimal{cost,trace,expanded})}if expanded>=limit{return Ok(Solution::Incomplete{expanded})}expanded+=1;
  for i in 0..g.parents.len(){for m in [Move::Compute(i),Move::Load(i),Move::Store(i),Move::DeleteRed(i),Move::DeleteBlue(i)]{if let Ok((t,extra))=g.step(s,m,c){let next=cost.checked_add(extra).ok_or("cost overflow")?;if dist.get(&t).map_or(true,|old|next<*old){dist.insert(t,next);prev.insert(t,(s,m));queue.push(std::cmp::Reverse((next,t)));}}}}
 }Ok(Solution::Impossible)
}
#[test]fn chain_optimum(){let g=Game{parents:vec![0,1,2],capacity:2,sink:2};let c=Costs{compute:1,transfer:3};if let Solution::Optimal{cost,trace,..}=solve(&g,c,10000).unwrap(){assert_eq!(cost,3);assert_eq!(g.replay(&trace,c).unwrap().1,cost)}else{panic!("no optimum")}}
#[test]fn capacity_and_limits(){let g=Game{parents:vec![0,1],capacity:1,sink:1};assert!(matches!(solve(&g,Costs{compute:1,transfer:1},1000).unwrap(),Solution::Impossible));let g=Game{capacity:2,..g};assert!(matches!(solve(&g,Costs{compute:1,transfer:1},0).unwrap(),Solution::Incomplete{..}));assert!(g.step(State::default(),Move::Load(0),Costs{compute:1,transfer:1}).is_err());}
