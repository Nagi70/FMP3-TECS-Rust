#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT_DIR"

# Convert each ex*/fmp.elf to size_of_fmp.txt (if present) and then to CSV in the same directory
# Header: dir,section,size_hex,size_dec,addr_hex
shopt -s nullglob
for dir in ex*/; do
  base_dir="$(basename "$dir")"
  elf="$dir/fmp.elf"
  txt="$dir/size_of_fmp.txt"
  out_csv="$dir/size_of_fmp.csv"

  # 1) If fmp.elf exists, (re)generate size_of_fmp.txt from it
  if [[ -f "$elf" ]]; then
    echo "Generating $txt from $elf"
    arm-none-eabi-size -A -x "$elf" > "$txt"
  fi

  # 2) If size_of_fmp.txt does not exist, skip this dir
  if [[ ! -f "$txt" ]]; then
    echo "Skip $base_dir: neither $elf nor $txt present"
    continue
  fi

  awk -v DIR="$base_dir" '
  BEGIN {
    OFS=",";
    print "dir,section,size_hex,size_dec,addr_hex";
  }
  # Skip the first line like "fmp.elf  :"
  NR==1 && $0 ~ /:/ { next }
  # Skip header line
  $1 == "section" { next }
  # Skip empty lines
  NF == 0 { next }
  {
    sec=$1; sz=$2; addr=$3;
    # normalize hex strings (strip 0x if present)
    gsub(/^0[xX]/, "", sz);
    gsub(/^0[xX]/, "", addr);

    if (sec == "Total") {
      # Total line has no addr
      printf "%s,%s,0x%s,%d,\n", DIR, sec, sz, strtonum("0x" sz);
    } else {
      printf "%s,%s,0x%s,%d,0x%s\n", DIR, sec, sz, strtonum("0x" sz), addr;
    }
  }
  ' "$txt" > "$out_csv"

  echo "Wrote: $out_csv"

  # Also create a debug-excluded CSV (exclude only .debug* sections)
  out_nodebug_csv="$dir/size_of_fmp.nodebug.csv"
  awk -v DIR="$base_dir" '
  BEGIN {
    OFS=",";
    print "dir,section,size_hex,size_dec,addr_hex";
  }
  NR==1 && $0 ~ /:/ { next }
  $1 == "section" { next }
  NF == 0 { next }
  $1 ~ /^\.debug/ { next }
  {
    sec=$1; sz=$2; addr=$3;
    gsub(/^0[xX]/, "", sz);
    gsub(/^0[xX]/, "", addr);

    if (sec == "Total") {
      printf "%s,%s,0x%s,%d,\n", DIR, sec, sz, strtonum("0x" sz);
    } else {
      printf "%s,%s,0x%s,%d,0x%s\n", DIR, sec, sz, strtonum("0x" sz), addr;
    }
  }
  ' "$txt" > "$out_nodebug_csv"

  echo "Wrote: $out_nodebug_csv"
done

# Build combined CSV at root
combined="$ROOT_DIR/size_of_fmp_all.csv"
# Write header
printf "dir,section,size_hex,size_dec,addr_hex\n" > "$combined"

for csv in ex*/size_of_fmp.csv; do
  # append skipping header
  if [[ -f "$csv" ]]; then
    tail -n +2 "$csv" >> "$combined"
  fi
done

echo "Wrote: $combined"

# Build combined nodebug CSV at root
combined_nodebug="$ROOT_DIR/size_of_fmp_all.nodebug.csv"
printf "dir,section,size_hex,size_dec,addr_hex\n" > "$combined_nodebug"
for csv in ex*/size_of_fmp.nodebug.csv; do
  if [[ -f "$csv" ]]; then
    tail -n +2 "$csv" >> "$combined_nodebug"
  fi
done
echo "Wrote: $combined_nodebug"
