#!/usr/bin/env bash
set -euo pipefail

TR_ROOT="/home/yosshy/research/FMP3-TECS/tr_opt_sample"
R_ROOT="/home/yosshy/research/FMP3-TECS/r_opt_sample"
OUT_DIR="$TR_ROOT/compare"
mkdir -p "$OUT_DIR"

# Helper: sum per-ex totals from libkernel_objects.csv
sum_totals() {
  local csv_dir="$1"  # directory containing ex*/libkernel_objects.csv
  local out_csv="$2"
  printf "ex,text_dec,rodata_dec,data_dec,bss_dec,other_dec,total_dec\n" > "$out_csv"
  shopt -s nullglob
  for f in "$csv_dir"/ex*/libkernel_objects.csv; do
    local dir
    dir="$(basename "$(dirname "$f")")"
    awk -F, -v EX="$dir" 'NR==1{next} {tt+=$3; rr+=$4; dd+=$5; bb+=$6; oo+=$7; tot+=$8} END{printf "%s,%d,%d,%d,%d,%d,%d\n", EX,tt,rr,dd,bb,oo,tot}' "$f" >> "$out_csv"
  done | true
}

# Build totals per tree
TR_TOT_NO="$OUT_DIR/libkernel_totals_tr_noopt.csv"
TR_TOT_OP="$OUT_DIR/libkernel_totals_tr_opt.csv"
R_TOT="$OUT_DIR/libkernel_totals_r.csv"

sum_totals "$TR_ROOT" "$OUT_DIR/.tmp_all_tr.csv"
# Split TR totals into no_opt and opt by ex suffix
{
  printf "ex,text_dec,rodata_dec,data_dec,bss_dec,other_dec,total_dec\n"
  grep -E '^ex[0-9]+_no_opt,' "$OUT_DIR/.tmp_all_tr.csv" | sed 's/_no_opt,/,/'
} > "$TR_TOT_NO" || true
{
  printf "ex,text_dec,rodata_dec,data_dec,bss_dec,other_dec,total_dec\n"
  grep -E '^ex[0-9]+_opt,' "$OUT_DIR/.tmp_all_tr.csv" | sed 's/_opt,/,/'
} > "$TR_TOT_OP" || true
rm -f "$OUT_DIR/.tmp_all_tr.csv" || true

sum_totals "$R_ROOT" "$R_TOT"

# Join three totals by ex number
THREE_TOT="$OUT_DIR/libkernel_three_totals_all.csv"
printf "ex,r_text,r_rodata,r_data,r_bss,r_other,r_total,tr_no_text,tr_no_rodata,tr_no_data,tr_no_bss,tr_no_other,tr_no_total,tr_opt_text,tr_opt_rodata,tr_opt_data,tr_opt_bss,tr_opt_other,tr_opt_total\n" > "$THREE_TOT"
awk -F, 'NR==1{next} {gsub(/^ex/,"",$1); r[$1]=$0} END{for (k in r) exs[k]=1}' "$R_TOT" >/dev/null
awk -F, 'NR==1{next} {gsub(/^ex/,"",$1); no[$1]=$0} END{for (k in no) exs[k]=1}' "$TR_TOT_NO" >/dev/null
awk -F, 'NR==1{next} {gsub(/^ex/,"",$1); op[$1]=$0} END{for (k in op) exs[k]=1}' "$TR_TOT_OP" >/dev/null
awk -F, -v RTOT="$R_TOT" -v NTT="$TR_TOT_NO" -v OTT="$TR_TOT_OP" -v OFS="," '
  BEGIN{
    # load maps
    while ((getline line < RTOT)>0) { if (NRr++==0) continue; split(line,a,","); ex=a[1]; sub(/^ex/,"",ex); r[ex]=line }
    close(RTOT)
    while ((getline line < NTT)>0) { if (NRn++==0) continue; split(line,a,","); ex=a[1]; sub(/^ex/,"",ex); no[ex]=line }
    close(NTT)
    while ((getline line < OTT)>0) { if (NRo++==0) continue; split(line,a,","); ex=a[1]; sub(/^ex/,"",ex); op[ex]=line }
    close(OTT)
    for (i=1;i<=50;i++) {
      key=sprintf("%d",i)
      if (key in r || key in no || key in op) {
        split(r[key],ra,","); split(no[key],na,","); split(op[key],oa,",")
        # indexes: ex,text,rodata,data,bss,other,total
        printf "ex%s", key
        for (j=2;j<=7;j++) printf ",%s", (length(ra)>0?ra[j]:0)
        for (j=2;j<=7;j++) printf ",%s", (length(na)>0?na[j]:0)
        for (j=2;j<=7;j++) printf ",%s", (length(oa)>0?oa[j]:0)
        printf "\n"
      }
    }
  }
' >> "$THREE_TOT"

echo "Wrote: $THREE_TOT"

# Unique object detection per tree (baseline diff)
printf "dir,object,text_dec,rodata_dec,data_dec,bss_dec,other_dec,total_dec\n" > "$OUT_DIR/libkernel_unique_tr.csv"
printf "dir,object,text_dec,rodata_dec,data_dec,bss_dec,other_dec,total_dec\n" > "$OUT_DIR/libkernel_unique_r.csv"

# TR baseline: ex1_no_opt if exists; otherwise first ex*_no_opt
TR_BASE="$(ls -d "$TR_ROOT"/ex*_no_opt 2>/dev/null | head -n1)"
if [[ -n "$TR_BASE" ]]; then
  BCSV="$TR_BASE/libkernel_objects.csv"
  for d in "$TR_ROOT"/ex*_no_opt "$TR_ROOT"/ex*_opt; do
    [[ -d "$d" ]] || continue
    DCSV="$d/libkernel_objects.csv"
    [[ -f "$BCSV" && -f "$DCSV" ]] || continue
    awk -F, 'NR==FNR{ if(NR>1) base[$2]=$8; next } NR>1 { if ($8 != base[$2]) print }' "$BCSV" "$DCSV" >> "$OUT_DIR/libkernel_unique_tr.csv"
  done
fi

# R baseline: ex1
R_BASE_DIR="/home/yosshy/research/FMP3-TECS/r_opt_sample/ex1"
if [[ -d "$R_BASE_DIR" ]]; then
  BCSV="$R_BASE_DIR/libkernel_objects.csv"
  for d in "$R_ROOT"/ex*; do
    [[ -d "$d" ]] || continue
    DCSV="$d/libkernel_objects.csv"
    [[ -f "$BCSV" && -f "$DCSV" ]] || continue
    awk -F, 'NR==FNR{ if(NR>1) base[$2]=$8; next } NR>1 { if ($8 != base[$2]) print }' "$BCSV" "$DCSV" >> "$OUT_DIR/libkernel_unique_r.csv"
  done
fi

echo "Wrote: $OUT_DIR/libkernel_unique_tr.csv"
echo "Wrote: $OUT_DIR/libkernel_unique_r.csv"

# Build per-ex combined unique summary (R vs TR_no_opt vs TR_opt)
U3="$OUT_DIR/libkernel_unique_three_all.csv"
printf "ex,r_object,r_total,tr_no_object,tr_no_total,tr_opt_object,tr_opt_total\n" > "$U3"
for n in 1 2 3 4 5; do
  exr="ex${n}"
  exno="ex${n}_no_opt"
  exop="ex${n}_opt"
  r_line=$(grep -m1 "^${exr}," "$OUT_DIR/libkernel_unique_r.csv" || true)
  no_line=$(grep -m1 "^${exno}," "$OUT_DIR/libkernel_unique_tr.csv" || true)
  op_line=$(grep -m1 "^${exop}," "$OUT_DIR/libkernel_unique_tr.csv" || true)
  r_obj=$(echo "$r_line" | awk -F, '{print $2}'); r_tot=$(echo "$r_line" | awk -F, '{print $8}')
  no_obj=$(echo "$no_line" | awk -F, '{print $2}'); no_tot=$(echo "$no_line" | awk -F, '{print $8}')
  op_obj=$(echo "$op_line" | awk -F, '{print $2}'); op_tot=$(echo "$op_line" | awk -F, '{print $8}')
  printf "ex%d,%s,%s,%s,%s,%s,%s\n" "$n" "${r_obj:-}" "${r_tot:-}" "${no_obj:-}" "${no_tot:-}" "${op_obj:-}" "${op_tot:-}" >> "$U3"
done

echo "Wrote: $U3"
