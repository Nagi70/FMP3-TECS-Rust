#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT_DIR"

out_all="libitron_compare_totals_all.csv"
printf "ex,variant,total_hex,total_dec\n" > "$out_all"

shopt -s nullglob
for f in ex*_libitron_compare.txt; do
  exname="${f%%_libitron_compare.txt}"
  exnum="${exname#ex}"
  out_file="${exname}_libitron_compare_totals.csv"
  printf "ex,variant,total_hex,total_dec\n" > "$out_file"

  awk -v EX="${exname}" -v OFS="," '
    BEGIN { variant="" }
    # Detect which variant we are in from header lines like: "... (ex ex1//libitron.a):"
    /\(ex [^)]*\):$/ {
      line=$0
      if (index(line, "_no_opt//")>0)      variant="tr_no_opt";
      else if (index(line, "_opt//")>0)   variant="tr_opt";
      else                                  variant="r";
      next
    }
    # Total line: Total  0xNNN
    /^Total[[:space:]]+/ {
      hex=$2; gsub(/^0[xX]/, "", hex); dec=strtonum("0x" hex);
      key=variant; th[key]="0x"hex; td[key]=dec;
      next
    }
    END {
      # Output in stable order r, tr_no_opt, tr_opt if present
      if (length(td["r"]))         printf "%s,%s,%s,%d\n", EX, "r", td["r"]?th["r"]:"", td["r"]; 
      if (length(td["tr_no_opt"])) printf "%s,%s,%s,%d\n", EX, "tr_no_opt", th["tr_no_opt"], td["tr_no_opt"]; 
      if (length(td["tr_opt"]))    printf "%s,%s,%s,%d\n", EX, "tr_opt", th["tr_opt"], td["tr_opt"]; 
    }
  ' "$f" | tee -a "$out_all" >> "$out_file"
done

echo "Wrote: $out_all"

# Build a single wide CSV with one row per ex and columns for r/tr_no_opt/tr_opt
out_wide="libitron_compare_totals_wide.csv"
printf "ex,r_hex,r_dec,tr_no_hex,tr_no_dec,tr_opt_hex,tr_opt_dec\n" > "$out_wide"
awk -F, 'NR==1{next} {
  ex=$1; var=$2; hx=$3; dec=$4;
  if (var=="r")          { rh[ex]=hx; rd[ex]=dec }
  else if (var=="tr_no_opt") { nh[ex]=hx; nd[ex]=dec }
  else if (var=="tr_opt")    { oh[ex]=hx; od[ex]=dec }
  exs[ex]=1
} END {
  for (e in exs) {
    printf "%s,%s,%s,%s,%s,%s,%s\n", e, (e in rh?rh[e]:""),(e in rd?rd[e]:""),(e in nh?nh[e]:""),(e in nd?nd[e]:""),(e in oh?oh[e]:""),(e in od?od[e]:"")
  }
}' "$out_all" | sort -t, -k1,1V >> "$out_wide"

echo "Wrote: $out_wide"
