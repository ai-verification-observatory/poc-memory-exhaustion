#!/usr/bin/env python3
"""Sequential Linux worker measurements. Output contains no machine identifiers."""
import argparse, hashlib, json, os, pathlib, platform, subprocess, time
p=argparse.ArgumentParser();p.add_argument('--nodes',type=int,default=33554432);p.add_argument('--trials',type=int,default=3);p.add_argument('--out',default='results/memory');a=p.parse_args()
out=pathlib.Path(a.out);out.mkdir(parents=True,exist_ok=True)
timebin=next(pathlib.Path('/nix/store').glob('*-time-*/bin/time'))
def command(c):return subprocess.check_output(c,text=True).strip()
meta={'commit':command(['git','rev-parse','HEAD']),'dirty':bool(command(['git','status','--porcelain'])),'rustc':command([str(next(pathlib.Path('/nix/store').glob('*-rustc-*/bin/rustc'))),'--version']),'kernel':platform.release(),'ram':next(l.strip() for l in open('/proc/meminfo') if l.startswith('MemTotal:')),'cpu':next((l.split(':',1)[1].strip() for l in open('/proc/cpuinfo') if l.startswith('model name')),'unknown'),'nodes':a.nodes,'interval':'spawn through worker exit; GNU time excludes wrapper polling','sample_period_ms':100}
(out/'metadata.json').write_text(json.dumps(meta,indent=2))
records=[]
for trial in range(a.trials):
 seed=hashlib.sha256(f'memory-poc-measure-v1:{trial}'.encode()).hexdigest();digests=[]
 for policy in ['dr-worker','dr-greedy-worker']:
  stem=f'{trial}-{policy}';cmd=['target/release/memory-poc',policy,str(a.nodes),seed]
  with (out/(stem+'.stdout')).open('w') as stdout,(out/(stem+'.stderr')).open('w') as stderr:
   proc=subprocess.Popen([str(timebin),'-f','rss_kib=%M elapsed_s=%e user_s=%U system_s=%S major_faults=%F minor_faults=%R exit=%x','-o',str(out/(stem+'.time'))]+cmd,stdout=stdout,stderr=stderr)
   start=time.monotonic();samples=[];timed_out=False
   while proc.poll() is None:
    # GNU time's direct child is the measured worker. Never collect cmdline/identity fields.
    try:
     children=pathlib.Path(f'/proc/{proc.pid}/task/{proc.pid}/children').read_text().split()
     for child in children:
      values={}
      for l in pathlib.Path(f'/proc/{child}/status').read_text().splitlines():
       k,_,v=l.partition(':')
       if k in ['VmRSS','VmHWM','VmSize','RssAnon','VmSwap']:values[k]=v.strip()
      samples.append({'elapsed_s':time.monotonic()-start,**values})
    except (FileNotFoundError,ProcessLookupError):pass
    if time.monotonic()-start>1800:
     timed_out=True
     for child in children:
      try:os.kill(int(child),9)
      except ProcessLookupError:pass
     break
    time.sleep(.1)
   proc.wait()
  text=(out/(stem+'.stdout')).read_text().strip();digests.append(text)
  (out/(stem+'.samples.json')).write_text(json.dumps(samples))
  rec={'trial':trial,'policy':policy,'seed':seed,'command':cmd,'exit':proc.returncode,'timeout':timed_out,'measurement':(out/(stem+'.time')).read_text(),'stderr_tail':(out/(stem+'.stderr')).read_text()[-2000:]};records.append(rec)
  (out/'runs.json').write_text(json.dumps(records,indent=2));print(json.dumps(rec),flush=True)
 if digests[0]!=digests[1] or not digests[0]:raise SystemExit('output mismatch or missing output')
