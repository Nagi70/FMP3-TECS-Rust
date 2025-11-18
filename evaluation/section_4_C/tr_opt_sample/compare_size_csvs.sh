#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT_DIR"

OUT_DIR="$ROOT_DIR/compare"
mkdir -p "$OUT_DIR"

# Which per-dir CSV to use for comparison: nodebug or full
# Default to nodebug CSVs; change to "size_of_fmp.csv" if you want full comparison.
CSV_NAME="size_of_fmp.nodebug.csv"

# Header for per-pair compare output
PAIR_HEADER="ex,section,no_opt_size_dec,opt_size_dec,diff_dec,no_opt_size_hex,opt_size_hex,diff_hex"

# Combined output
ALL_OUT="$OUT_DIR/size_compare_all.nodebug.csv"
printf "%s\n" "$PAIR_HEADER" > "$ALL_OUT"

shopt -s nullglob
for left in ex*_no_opt; do
  exbase="${left%_no_opt}"
  right="${exbase}_opt"
  [[ -d "$right" ]] || { echo "Skip $exbase: missing $right"; continue; }

  left_csv="$left/$CSV_NAME"
  right_csv="$right/$CSV_NAME"
  [[ -f "$left_csv" && -f "$right_csv" ]] || { echo "Skip $exbase: missing CSV(s)"; continue; }

  out_pair="$OUT_DIR/size_compare_${exbase##*/}.nodebug.csv"
  printf "%s\n" "$PAIR_HEADER" > "$out_pair"

  awk -F, -v EX="${exbase##*/}" '
    FNR==1 { next }
    FNR>1 {
      if (FILENAME ~ /_no_opt\//) {
        no_opt_hex[$2]=$3; no_opt_dec[$2]=$4
      } else {
        opt_hex[$2]=$3; opt_dec[$2]=$4
      }
      sections[$2]=1
    }
    END {
      for (s in sections) {
        ndec = (s in no_opt_dec) ? no_opt_dec[s] + 0 : 0
        odec = (s in opt_dec) ? opt_dec[s] + 0 : 0
        dhex = sprintf("0x%x", odec - ndec)
        printf "%s,%s,%d,%d,%d,%s,%s,%s\n", EX, s, ndec, odec, odec-ndec,
               ((s in no_opt_hex)?no_opt_hex[s]:"0x0"),
               ((s in opt_hex)?opt_hex[s]:"0x0"),
               dhex
      }
    }
  ' "$left_csv" "$right_csv" | sort -t, -k2,2 >> "$out_pair"

  # Append to combined output
  tail -n +2 "$out_pair" >> "$ALL_OUT"
  echo "Wrote: $out_pair"
done

echo "Wrote: $ALL_OUT"
