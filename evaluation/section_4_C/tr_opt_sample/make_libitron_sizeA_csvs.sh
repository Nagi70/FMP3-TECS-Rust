#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "$0")" && pwd)"
cd "$ROOT_DIR"

shopt -s nullglob

# Per-ex: produce size_of_libitron_size.txt, libitron_size_sections.csv, libitron_size_objects.csv
for dir in ex*/; do
  base_dir="$(basename "$dir")"
  ar="$dir/libitron.a"
  txt="$dir/size_of_libitron_size.txt"
  sec_csv="$dir/libitron_size_sections.csv"
  obj_csv="$dir/libitron_size_objects.csv"

  if [[ ! -f "$ar" ]]; then
    echo "Skip $base_dir: missing libitron.a"
    continue
  fi

  echo "Size -A -x $ar -> $txt"
  arm-none-eabi-size -A -x "$ar" > "$txt"

  # Sections CSV from size -A -x (per-member header: '<obj>   (ex */libitron.a):')
  awk -v DIR="$base_dir" '
    BEGIN { OFS=","; print "dir,object,section,size_hex,size_dec,addr_hex" }
    /^[[:alnum:]_\.-]+\.o[[:space:]]+\(ex[[:space:]].*libitron\.a\):/ { 
      obj=$1; sub(/:.*/,"",obj); next 
    }
    /^section[[:space:]]+size[[:space:]]+addr/ { next }
    /^[[:space:]]*$/ { next }
    /^Total/ { 
      sz=$2; gsub(/^0[xX]/, "", sz); printf "%s,%s,%s,0x%s,%d,%s\n", DIR,obj,"Total",sz,strtonum("0x" sz),""; next 
    }
    /^\./ {
      sec=$1; sz=$2; ad=$3; gsub(/^0[xX]/, "", sz); 
      printf "%s,%s,%s,0x%s,%d,%s\n", DIR,obj,sec,sz,strtonum("0x" sz),ad 
    }
  ' "$txt" > "$sec_csv"
  echo "Wrote: $sec_csv"

  # Aggregate per object from size sections
  awk -F, '
    BEGIN { OFS=","; print "dir,object,text_dec,rodata_dec,data_dec,bss_dec,other_dec,total_dec,text_hex,rodata_hex,data_hex,bss_hex,other_hex,total_hex" }
    NR==1 { next }
    { dir=$1; obj=$2; sec=$3; dec=$5; key=dir"\t"obj;
      if (sec==".text")      { t[key]+=dec }
      else if (sec==".rodata"){ r[key]+=dec }
      else if (sec==".data") { d[key]+=dec }
      else if (sec==".bss")  { b[key]+=dec }
      else if (sec=="Total") { tot[key]+=dec }
      else                     { o[key]+=dec } }
    END {
      for (k in t) used[k]=1; for (k in r) used[k]=1; for (k in d) used[k]=1; for (k in b) used[k]=1; for (k in o) used[k]=1; for (k in tot) used[k]=1;
      for (k in used) { split(k,a,"\t"); dir=a[1]; obj=a[2];
        td=(k in t)?t[k]:0; rd=(k in r)?r[k]:0; dd=(k in d)?d[k]:0; bd=(k in b)?b[k]:0; od=(k in o)?o[k]:0; tt=(k in tot)?tot[k]:(td+rd+dd+bd+od);
        printf "%s,%s,%d,%d,%d,%d,%d,%d,0x%x,0x%x,0x%x,0x%x,0x%x,0x%x\n", dir,obj,td,rd,dd,bd,od,tt, td,rd,dd,bd,od,tt }
    }
  ' "$sec_csv" | sort -t, -k2,2 > "$obj_csv"
  echo "Wrote: $obj_csv"

done

# Combined objects CSV across all ex* under this root (size-based)
combined_obj="$ROOT_DIR/libitron_size_objects_all.csv"
printf "dir,object,text_dec,rodata_dec,data_dec,bss_dec,other_dec,total_dec,text_hex,rodata_hex,data_hex,bss_hex,other_hex,total_hex\n" > "$combined_obj"
for f in ex*/libitron_size_objects.csv; do
  [[ -f "$f" ]] && tail -n +2 "$f" >> "$combined_obj"
done

echo "Wrote: $combined_obj"
