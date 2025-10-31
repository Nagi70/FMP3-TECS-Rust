#!/usr/bin/env bash
set -euo pipefail

TR_ROOT="/home/yosshy/research/FMP3-TECS/tr_opt_sample"
R_ROOT="/home/yosshy/research/FMP3-TECS/r_opt_sample"

OUT_DIR="$TR_ROOT/compare"
mkdir -p "$OUT_DIR"

# Use nodebug CSVs by default
CSV_NAME="size_of_fmp.nodebug.csv"

HEADER="ex,section,r_size_dec,tr_no_opt_size_dec,tr_opt_size_dec,d_trnoopt_vs_r_dec,d_tropt_vs_r_dec,d_tropt_vs_trnoopt_dec,r_size_hex,tr_no_opt_size_hex,tr_opt_size_hex,d_trnoopt_vs_r_hex,d_tropt_vs_r_hex,d_tropt_vs_trnoopt_hex"
ALL_OUT="$OUT_DIR/three_compare_all.nodebug.csv"
printf "%s\n" "$HEADER" > "$ALL_OUT"

for n in 1 2 3 4 5; do
  ex="ex${n}"
  r_csv="$R_ROOT/$ex/$CSV_NAME"
  tr_no_csv="$TR_ROOT/${ex}_no_opt/$CSV_NAME"
  tr_op_csv="$TR_ROOT/${ex}_opt/$CSV_NAME"

  if [[ ! -f "$r_csv" || ! -f "$tr_no_csv" || ! -f "$tr_op_csv" ]]; then
    echo "Skip $ex: missing one of CSVs" >&2
    continue
  fi

  out_pair="$OUT_DIR/three_compare_${ex}.nodebug.csv"
  printf "%s\n" "$HEADER" > "$out_pair"

  awk -F, -v EX="$ex" '
    FNR==1 { next }
    {
      if (FILENAME ~ /\/r_opt_sample\//) {
        r_hex[$2]=$3; r_dec[$2]=$4
      } else if (FILENAME ~ /_no_opt\//) {
        trno_hex[$2]=$3; trno_dec[$2]=$4
      } else {
        trop_hex[$2]=$3; trop_dec[$2]=$4
      }
      sections[$2]=1
    }
    END {
      for (s in sections) {
        r  = (s in r_dec)   ? r_dec[s] + 0   : 0
        no = (s in trno_dec)? trno_dec[s] + 0: 0
        op = (s in trop_dec)? trop_dec[s] + 0: 0
        d1 = no - r
        d2 = op - r
        d3 = op - no
        printf "%s,%s,%d,%d,%d,%d,%d,%d,%s,%s,%s,0x%x,0x%x,0x%x\n",
          EX,s,r,no,op,d1,d2,d3,
          ((s in r_hex)?r_hex[s]:"0x0"),
          ((s in trno_hex)?trno_hex[s]:"0x0"),
          ((s in trop_hex)?trop_hex[s]:"0x0"),
          d1,d2,d3
      }
    }
  ' "$r_csv" "$tr_no_csv" "$tr_op_csv" | sort -t, -k2,2 >> "$out_pair"

  tail -n +2 "$out_pair" >> "$ALL_OUT"
  echo "Wrote: $out_pair"

done

echo "Wrote: $ALL_OUT"
