# peakrss: peak memory of a verifier run

`tools/peakrss.py` runs a command, polls `/proc` every 0.25 s for the command's whole descendant process tree (cargo, rustc, rust_verify, z3 workers), and records the peak. Each `verify.sh` calls it and appends two trailing columns to `RUNLOG.tsv`. The command's stdout, stderr and exit code pass through unchanged, and the run log's content is the same as before.

## Columns

- `peak_rss_sum_mb=<n>`: the largest value, over all samples, of the summed resident set size (RSS: physical memory a process currently holds) of every live process in the tree. This is the number to size a machine or container against.
- `peak_rss_max_proc_mb=<n>:<procname>`: the largest RSS of any single process at any sample, and that process's name (usually `rust_verify` or `z3`). The sum is never below this value.

`n/a` in either column means peakrss produced no result (for example the script path was wrong); the verifier's exit code is still the real one.

## Reading them

- Sum much larger than the single-process maximum means parallel workers (several z3 or rustc processes) dominate; lowering `--num-threads` or the rlimit would cut memory.
- Sum close to the maximum means one process dominates, normally `rust_verify` holding the VIR and AIR/SMT encoding.
- Shared pages (the same library mapped by several processes) are counted once per process, so the sum slightly overstates real memory use.
- Sampling misses processes that live less than 0.25 s, and a spike between two samples, so the figures are lower bounds. Add headroom when sizing.
- Under gVisor the sandbox may report RSS differently from a native kernel; compare runs within one environment.
- A run that fails early (exit 2 or 101) records the memory of whatever ran, so compare only rows with the same crate and `exit=0`.

## Standalone use

    python3 -I tools/peakrss.py --out result.txt -- cargo verus verify

Without `--out` the result line (`peak_rss_sum_mb`, `peak_rss_max_proc_mb`, `wall_s`, tab-separated) goes to stderr. If `/proc/<pid>/status` is unreadable the tool falls back to `ps -e -o pid=,ppid=,rss=,comm=`.
