mod drsample;
#[allow(dead_code)]
mod pebble;
const K:[u32;64]=[1116352408, 1899447441, 3049323471, 3921009573, 961987163, 1508970993, 2453635748, 2870763221, 3624381080, 310598401, 607225278, 1426881987, 1925078388, 2162078206, 2614888103, 3248222580, 3835390401, 4022224774, 264347078, 604807628, 770255983, 1249150122, 1555081692, 1996064986, 2554220882, 2821834349, 2952996808, 3210313671, 3336571891, 3584528711, 113926993, 338241895, 666307205, 773529912, 1294757372, 1396182291, 1695183700, 1986661051, 2177026350, 2456956037, 2730485921, 2820302411, 3259730800, 3345764771, 3516065817, 3600352804, 4094571909, 275423344, 430227734, 506948616, 659060556, 883997877, 958139571, 1322822218, 1537002063, 1747873779, 1955562222, 2024104815, 2227730452, 2361852424, 2428436474, 2756734187, 3204031479, 3329325298];

use std::{env,fs,io::{self,BufRead,Write},process::{Command,Stdio},time::Instant};
type Hash=[u8;32];
fn hash(data:&[u8])->Hash {
 let mut h=[0x6a09e667u32,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19];
 let mut d=data.to_vec();let bits=(d.len() as u64)*8;d.push(128);while d.len()%64!=56{d.push(0)}d.extend(bits.to_be_bytes());
 for chunk in d.chunks_exact(64){let mut w=[0u32;64];for i in 0..16{w[i]=u32::from_be_bytes(chunk[i*4..i*4+4].try_into().unwrap())}for i in 16..64{let a=w[i-15];let b=w[i-2];w[i]=w[i-16].wrapping_add(a.rotate_right(7)^a.rotate_right(18)^(a>>3)).wrapping_add(w[i-7]).wrapping_add(b.rotate_right(17)^b.rotate_right(19)^(b>>10));}let [mut a,mut b,mut c,mut e0,mut e,mut f,mut g,mut z]=h;for i in 0..64{let t=z.wrapping_add(e.rotate_right(6)^e.rotate_right(11)^e.rotate_right(25)).wrapping_add((e&f)^(!e&g)).wrapping_add(K[i]).wrapping_add(w[i]);let u=(a.rotate_right(2)^a.rotate_right(13)^a.rotate_right(22)).wrapping_add((a&b)^(a&c)^(b&c));z=g;g=f;f=e;e=e0.wrapping_add(t);e0=c;c=b;b=a;a=t.wrapping_add(u);}for (v,t) in h.iter_mut().zip([a,b,c,e0,e,f,g,z]){*v=v.wrapping_add(t)}}
 let mut out=[0;32];for(i,v)in h.iter().enumerate(){out[i*4..i*4+4].copy_from_slice(&v.to_be_bytes())}out
}
fn hex(h:&Hash)->String{h.iter().map(|b|format!("{b:02x}")).collect()}
fn tagged(tag:u8,parts:&[&[u8]])->Hash{let mut b=vec![tag];for p in parts{b.extend_from_slice(p)}hash(&b)}
fn parent(seed:&Hash,i:usize,w:usize)->usize{let h=tagged(1,&[seed,&(i as u64).to_le_bytes()]);u64::from_le_bytes(h[..8].try_into().unwrap())as usize%w}
fn label(seed:&Hash,i:usize,_w:usize,a:Option<(Hash,Hash)>)->Hash{let pos=(i as u64).to_le_bytes();match a{None=>tagged(2,&[seed,&pos]),Some((x,y))=>tagged(3,&[seed,&pos,&x,&y])}}
fn generate(seed:&Hash,w:usize)->Vec<Hash>{let n=w.checked_mul(4).expect("size overflow");let mut s=Vec::new();s.try_reserve_exact(n).expect("allocation failed");for i in 0..n{let p=if i<w{None}else{Some((s[i-w],s[(i/w-1)*w+parent(seed,i,w)]))};s.push(label(seed,i,w,p))}s}
fn tree(s:&[Hash])->Vec<Vec<Hash>>{let mut t=vec![s.iter().map(|x|tagged(4,&[x])).collect::<Vec<_>>()];while t.last().unwrap().len()>1{let v=t.last().unwrap();t.push(v.chunks_exact(2).map(|p|tagged(5,&[&p[0],&p[1]])).collect())}t}
fn recompute(seed:&Hash,i:usize,w:usize,keep:&[Hash])->Hash{if i<keep.len(){return keep[i]}let p=if i<w{None}else{Some((recompute(seed,i-w,w,keep),recompute(seed,(i/w-1)*w+parent(seed,i,w),w,keep)))};label(seed,i,w,p)}
fn config(p:&str)->(usize,usize,usize,u128){let s=fs::read_to_string(p).expect("config");let get=|k:&str|s.lines().find_map(|l|l.strip_prefix(&format!("{k}="))).expect("missing key").parse::<usize>().expect("integer");let w=get("width");assert!(w.is_power_of_two());(w,get("rounds"),get("batch"),get("deadline_ms")as u128)}
fn random()->Hash{use std::io::Read;let mut b=[0;32];fs::File::open("/dev/urandom").unwrap().read_exact(&mut b).unwrap();b}
fn dr_cli(args:&[String])->Result<(),String>{
 let mode=&args[1];
 let n=args.get(2).ok_or("missing node count")?.parse::<usize>().map_err(|e|e.to_string())?;
 if mode=="dr-worker"{
  let raw=args.get(3).ok_or("seed missing")?;if raw.len()!=64{return Err("seed length".into())}let mut seed=[0;32];for i in 0..32{seed[i]=u8::from_str_radix(&raw[i*2..i*2+2],16).map_err(|e|e.to_string())?}
  let h=drsample::full(&seed,n)?;println!("{}",hex(&h));return Ok(())
 }
 if mode=="dr-solve"{
  if n>12{return Err("exact solver supports at most 12 nodes".into())}
  let cap=args.get(3).ok_or("missing red capacity")?.parse::<u32>().map_err(|e|e.to_string())?;
  let seed=[0;32];let parents=(0..n).map(|i|drsample::parents(&seed,i).iter().fold(0u64,|mask,p|mask|(1u64<<p))).collect();
  let g=pebble::Game{parents,capacity:cap,sink:n.checked_sub(1).ok_or("empty graph")?};let c=pebble::Costs{compute:1,transfer:10};
  let solution=pebble::solve(&g,c,100000)?;if let pebble::Solution::Optimal{cost,ref trace,..}=solution{if g.replay(trace,c)?.1!=cost{return Err("trace replay mismatch".into())}}println!("{solution:?}");return Ok(())
 }
 if mode=="dr-run"{
  let seed=random();let expected=drsample::full(&seed,n)?;let start=Instant::now();
  let out=Command::new(env::current_exe().map_err(|e|e.to_string())?).args(["dr-worker",&n.to_string(),&hex(&seed)]).output().map_err(|e|e.to_string())?;
  let elapsed=start.elapsed();if !out.status.success(){return Err(format!("worker exit={:?}",out.status.code()))}
  if String::from_utf8_lossy(&out.stdout).trim()!=hex(&expected){return Err("incorrect result".into())}
  println!("graph=drsample-v1 nodes={n} state_bytes={} seed={} elapsed_ns={} interval=spawn_through_response correct=true",n.checked_mul(32).ok_or("size overflow")?,hex(&seed),elapsed.as_nanos());
  let start=Instant::now();let(h,peak)=drsample::greedy(&seed,n)?;if h!=expected{return Err("greedy mismatch".into())}
  println!("policy=sequential_last_use peak_label_bytes={} metadata_bytes={} elapsed_ns={} correct=true",peak*32,n*std::mem::size_of::<usize>(),start.elapsed().as_nanos());return Ok(())
 }
 Err("unknown dr command".into())
}
fn main(){let args:Vec<_>=env::args().collect();if args.get(1).map_or(false,|s|s.starts_with("dr-")){if let Err(e)=dr_cli(&args){eprintln!("{e}");std::process::exit(1)}return}let mode=args.get(1).map(String::as_str).unwrap_or("help");if mode=="help"{println!("audit CONFIG [retained_percent] | worker CONFIG SEED_HEX retained_percent");return}let(w,rounds,batch,deadline)=config(&args[2]);
 if mode=="worker"{let raw=&args[3];let mut seed=[0;32];for i in 0..32{seed[i]=u8::from_str_radix(&raw[i*2..i*2+2],16).unwrap()}let pct:usize=args[4].parse().unwrap();assert!(pct<=100);let mut s=generate(&seed,w);let t=tree(&s);println!("{}",hex(&t.last().unwrap()[0]));io::stdout().flush().unwrap();s.truncate(s.len()*pct/100);s.shrink_to_fit();for line in io::stdin().lock().lines(){let line=line.unwrap();for word in line.split_whitespace(){let mut i:usize=word.parse().unwrap();let value=recompute(&seed,i,w,&s);let mut out=hex(&value);for level in &t[..t.len()-1]{out.push(' ');out.push_str(&hex(&level[i^1]));i/=2}println!("{out}")}io::stdout().flush().unwrap()}return}
 assert_eq!(mode,"audit");let seed=random();let reference=generate(&seed,w);let rt=tree(&reference);let root=hex(&rt.last().unwrap()[0]);let pct=args.get(3).map(String::as_str).unwrap_or("100");let mut child=Command::new(env::current_exe().unwrap()).args(["worker",&args[2],&hex(&seed),pct]).stdin(Stdio::piped()).stdout(Stdio::piped()).spawn().unwrap();let mut input=child.stdin.take().unwrap();let mut output=io::BufReader::new(child.stdout.take().unwrap());let mut line=String::new();output.read_line(&mut line).unwrap();assert_eq!(line.trim(),root);println!("state_bytes={} seed={} retained_percent={} clock=Instant interval=dispatch_to_complete_response",w*4*32,hex(&seed),pct);
 for r in 0..rounds{let ids:Vec<_>=(0..batch).map(|_|u64::from_le_bytes(random()[..8].try_into().unwrap())as usize%(w*4)).collect();let start=Instant::now();writeln!(input,"{}",ids.iter().map(usize::to_string).collect::<Vec<_>>().join(" ")).unwrap();input.flush().unwrap();let mut valid=true;for id in ids{line.clear();output.read_line(&mut line).unwrap();let expected_path={let mut i=id;let mut v=vec![hex(&reference[id])];for level in &rt[..rt.len()-1]{v.push(hex(&level[i^1]));i/=2}v.join(" ")};valid &=line.trim()==expected_path;}let elapsed=start.elapsed();println!("round={r} elapsed_ns={} correct={valid} accepted={}",elapsed.as_nanos(),valid&&elapsed.as_millis()<=deadline);}
 drop(input);assert!(child.wait().unwrap().success());
}
#[test]fn sha_vectors(){assert_eq!(hex(&hash(b"abc")),"ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");}
#[test]fn attack_matches(){let seed=[7;32];let s=generate(&seed,8);for i in 0..s.len(){assert_eq!(recompute(&seed,i,8,&[]),s[i])}assert_eq!(tree(&s).last().unwrap().len(),1);}
