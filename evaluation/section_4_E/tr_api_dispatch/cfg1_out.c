/* cfg1_out.c */
#define TOPPERS_CFG1_OUT
#include "kernel/kernel_int.h"
#line 7 "../target/zybo_z7_gcc/target_timer.cfg"
#include "target_timer.h"
#line 18 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 2
#line 26 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif
#line 28 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 3
#line 36 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif
#line 38 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 4
#line 46 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif
#line 52 "../arch/arm_gcc/common/mpcore_timer.cfg"
#ifdef USE_MPCORE_WDG_OVRTIMER
#line 53 "../arch/arm_gcc/common/mpcore_timer.cfg"
#ifdef TOPPERS_SUPPORT_OVRHDR
#line 65 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 2
#line 74 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif
#line 76 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 3
#line 85 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif
#line 87 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 4
#line 96 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif
#line 98 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif
#line 99 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif
#line 7 "../target/zybo_z7_gcc/target_ipi.cfg"
#include "target_ipi.h"
#line 11 "../arch/arm_gcc/common/gic_ipi.cfg"
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER
#line 14 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 19 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 2
#line 21 "../arch/arm_gcc/common/gic_ipi.cfg"
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER
#line 24 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 28 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 30 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 3
#line 32 "../arch/arm_gcc/common/gic_ipi.cfg"
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER
#line 35 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 39 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 41 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 4
#line 43 "../arch/arm_gcc/common/gic_ipi.cfg"
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER
#line 46 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 50 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 62 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 2
#line 69 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 71 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 3
#line 78 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 80 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 4
#line 87 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 99 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 2
#line 106 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 108 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 3
#line 115 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 117 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 4
#line 124 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif
#line 1 "./gen/tecsgen.cfg"
#include "tTask_tecsgen.h"
#line 3 "./gen/tecsgen.cfg"
#include "tISR_tecsgen.h"
#line 4 "./gen/tecsgen.cfg"
#include "tInitializeRoutine_tecsgen.h"
#line 5 "./gen/tecsgen.cfg"
#include "tTerminateRoutine_tecsgen.h"
#line 70 "./gen/tecsgen.cfg"
#include "rust_tecs.h"

#ifdef INT64_MAX
  typedef int64_t signed_t;
  typedef uint64_t unsigned_t;
#else
  typedef int32_t signed_t;
  typedef uint32_t unsigned_t;
#endif

#include "target_cfg1_out.h"

const uint32_t TOPPERS_magic_number = 0x12345678;
const uint32_t TOPPERS_sizeof_signed_t = sizeof(signed_t);
const uint32_t TOPPERS_sizeof_intptr_t = sizeof(intptr_t);
const uint32_t TOPPERS_sizeof_char_ptr_t = sizeof(char *);

const unsigned_t TOPPERS_cfg_CHAR_BIT = (unsigned_t)(CHAR_BIT);
const signed_t TOPPERS_cfg_SCHAR_MAX = (signed_t)(SCHAR_MAX);
const signed_t TOPPERS_cfg_SCHAR_MIN = (signed_t)(SCHAR_MIN);
const unsigned_t TOPPERS_cfg_UCHAR_MAX = (unsigned_t)(UCHAR_MAX);
const signed_t TOPPERS_cfg_CHAR_MAX = (signed_t)(CHAR_MAX);
const signed_t TOPPERS_cfg_CHAR_MIN = (signed_t)(CHAR_MIN);
const signed_t TOPPERS_cfg_SHRT_MAX = (signed_t)(SHRT_MAX);
const signed_t TOPPERS_cfg_SHRT_MIN = (signed_t)(SHRT_MIN);
const unsigned_t TOPPERS_cfg_USHRT_MAX = (unsigned_t)(USHRT_MAX);
const signed_t TOPPERS_cfg_INT_MAX = (signed_t)(INT_MAX);
const signed_t TOPPERS_cfg_INT_MIN = (signed_t)(INT_MIN);
const unsigned_t TOPPERS_cfg_UINT_MAX = (unsigned_t)(UINT_MAX);
const signed_t TOPPERS_cfg_LONG_MAX = (signed_t)(LONG_MAX);
const signed_t TOPPERS_cfg_LONG_MIN = (signed_t)(LONG_MIN);
const unsigned_t TOPPERS_cfg_ULONG_MAX = (unsigned_t)(ULONG_MAX);
#if defined(SIL_ENDIAN_BIG)
const signed_t TOPPERS_cfg_SIL_ENDIAN_BIG = (signed_t)(true);
#endif
#if defined(SIL_ENDIAN_LITTLE)
const signed_t TOPPERS_cfg_SIL_ENDIAN_LITTLE = (signed_t)(true);
#endif
#if defined(USE_EXTERNAL_ID)
const signed_t TOPPERS_cfg_USE_EXTERNAL_ID = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_EXTERNAL_ID = (signed_t)(false);
#endif
const signed_t TOPPERS_cfg_TNUM_PRCID = (signed_t)(TNUM_PRCID);
const unsigned_t TOPPERS_cfg_TA_NULL = (unsigned_t)(TA_NULL);
const unsigned_t TOPPERS_cfg_TA_ACT = (unsigned_t)(TA_ACT);
const unsigned_t TOPPERS_cfg_TA_NOACTQUE = (unsigned_t)(TA_NOACTQUE);
const unsigned_t TOPPERS_cfg_TA_TPRI = (unsigned_t)(TA_TPRI);
const unsigned_t TOPPERS_cfg_TA_WMUL = (unsigned_t)(TA_WMUL);
const unsigned_t TOPPERS_cfg_TA_CLR = (unsigned_t)(TA_CLR);
const unsigned_t TOPPERS_cfg_TA_CEILING = (unsigned_t)(TA_CEILING);
const unsigned_t TOPPERS_cfg_TA_NATIVE = (unsigned_t)(TA_NATIVE);
const unsigned_t TOPPERS_cfg_TA_STA = (unsigned_t)(TA_STA);
const unsigned_t TOPPERS_cfg_TA_NONKERNEL = (unsigned_t)(TA_NONKERNEL);
const unsigned_t TOPPERS_cfg_TA_ENAINT = (unsigned_t)(TA_ENAINT);
const unsigned_t TOPPERS_cfg_TA_EDGE = (unsigned_t)(TA_EDGE);
const unsigned_t TOPPERS_cfg_TNFY_HANDLER = (unsigned_t)(TNFY_HANDLER);
const unsigned_t TOPPERS_cfg_TNFY_SETVAR = (unsigned_t)(TNFY_SETVAR);
const unsigned_t TOPPERS_cfg_TNFY_INCVAR = (unsigned_t)(TNFY_INCVAR);
const unsigned_t TOPPERS_cfg_TNFY_ACTTSK = (unsigned_t)(TNFY_ACTTSK);
const unsigned_t TOPPERS_cfg_TNFY_WUPTSK = (unsigned_t)(TNFY_WUPTSK);
const unsigned_t TOPPERS_cfg_TNFY_SIGSEM = (unsigned_t)(TNFY_SIGSEM);
const unsigned_t TOPPERS_cfg_TNFY_SETFLG = (unsigned_t)(TNFY_SETFLG);
const unsigned_t TOPPERS_cfg_TNFY_SNDDTQ = (unsigned_t)(TNFY_SNDDTQ);
const unsigned_t TOPPERS_cfg_TENFY_SETVAR = (unsigned_t)(TENFY_SETVAR);
const unsigned_t TOPPERS_cfg_TENFY_INCVAR = (unsigned_t)(TENFY_INCVAR);
const unsigned_t TOPPERS_cfg_TENFY_ACTTSK = (unsigned_t)(TENFY_ACTTSK);
const unsigned_t TOPPERS_cfg_TENFY_WUPTSK = (unsigned_t)(TENFY_WUPTSK);
const unsigned_t TOPPERS_cfg_TENFY_SIGSEM = (unsigned_t)(TENFY_SIGSEM);
const unsigned_t TOPPERS_cfg_TENFY_SETFLG = (unsigned_t)(TENFY_SETFLG);
const unsigned_t TOPPERS_cfg_TENFY_SNDDTQ = (unsigned_t)(TENFY_SNDDTQ);
const signed_t TOPPERS_cfg_TMIN_TPRI = (signed_t)(TMIN_TPRI);
const signed_t TOPPERS_cfg_TMAX_TPRI = (signed_t)(TMAX_TPRI);
const signed_t TOPPERS_cfg_TMIN_DPRI = (signed_t)(TMIN_DPRI);
const signed_t TOPPERS_cfg_TMAX_DPRI = (signed_t)(TMAX_DPRI);
const signed_t TOPPERS_cfg_TMIN_ISRPRI = (signed_t)(TMIN_ISRPRI);
const signed_t TOPPERS_cfg_TMAX_ISRPRI = (signed_t)(TMAX_ISRPRI);
const unsigned_t TOPPERS_cfg_TBIT_FLGPTN = (unsigned_t)(TBIT_FLGPTN);
const unsigned_t TOPPERS_cfg_TMAX_MAXSEM = (unsigned_t)(TMAX_MAXSEM);
const unsigned_t TOPPERS_cfg_TMAX_RELTIM = (unsigned_t)(TMAX_RELTIM);
const signed_t TOPPERS_cfg_TMIN_INTPRI = (signed_t)(TMIN_INTPRI);
const signed_t TOPPERS_cfg_TMAX_INTPRI = (signed_t)(TMAX_INTPRI);
const signed_t TOPPERS_cfg_TMIN_TSKID = (signed_t)(TMIN_TSKID);
const signed_t TOPPERS_cfg_TMIN_SEMID = (signed_t)(TMIN_SEMID);
const signed_t TOPPERS_cfg_TMIN_FLGID = (signed_t)(TMIN_FLGID);
const signed_t TOPPERS_cfg_TMIN_DTQID = (signed_t)(TMIN_DTQID);
const signed_t TOPPERS_cfg_TMIN_PDQID = (signed_t)(TMIN_PDQID);
const signed_t TOPPERS_cfg_TMIN_MTXID = (signed_t)(TMIN_MTXID);
const signed_t TOPPERS_cfg_TMIN_MPFID = (signed_t)(TMIN_MPFID);
const signed_t TOPPERS_cfg_TMIN_CYCID = (signed_t)(TMIN_CYCID);
const signed_t TOPPERS_cfg_TMIN_ALMID = (signed_t)(TMIN_ALMID);
const unsigned_t TOPPERS_cfg_TOPPERS_TEPP_PRC = (unsigned_t)(TOPPERS_TEPP_PRC);
const signed_t TOPPERS_cfg_TOPPERS_MASTER_PRCID = (signed_t)(TOPPERS_MASTER_PRCID);
const signed_t TOPPERS_cfg_TOPPERS_TMASTER_PRCID = (signed_t)(TOPPERS_TMASTER_PRCID);
#if defined(USE_TSKINICTXB)
const signed_t TOPPERS_cfg_USE_TSKINICTXB = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_TSKINICTXB = (signed_t)(false);
#endif
#if defined(TMAX_NATIVE_SPN)
const signed_t TOPPERS_cfg_TMAX_NATIVE_SPN = (signed_t)(TMAX_NATIVE_SPN);
#endif
#if defined(USE_TSPNINIB)
const signed_t TOPPERS_cfg_USE_TSPNINIB = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_TSPNINIB = (signed_t)(false);
#endif
#if defined(USE_TSPNCB)
const signed_t TOPPERS_cfg_USE_TSPNCB = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_TSPNCB = (signed_t)(false);
#endif
#if defined(OMIT_INITIALIZE_INTERRUPT)
const signed_t TOPPERS_cfg_OMIT_INITIALIZE_INTERRUPT = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_OMIT_INITIALIZE_INTERRUPT = (signed_t)(false);
#endif
#if defined(USE_INHINIB_TABLE)
const signed_t TOPPERS_cfg_USE_INHINIB_TABLE = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_INHINIB_TABLE = (signed_t)(false);
#endif
#if defined(USE_INTINIB_TABLE)
const signed_t TOPPERS_cfg_USE_INTINIB_TABLE = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_INTINIB_TABLE = (signed_t)(false);
#endif
#if defined(OMIT_INITIALIZE_EXCEPTION)
const signed_t TOPPERS_cfg_OMIT_INITIALIZE_EXCEPTION = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_OMIT_INITIALIZE_EXCEPTION = (signed_t)(false);
#endif
#if defined(TARGET_TSKATR)
const unsigned_t TOPPERS_cfg_TARGET_TSKATR = (unsigned_t)(TARGET_TSKATR);
#else
const unsigned_t TOPPERS_cfg_TARGET_TSKATR = (unsigned_t)(0);
#endif
#if defined(TARGET_SPNATR)
const unsigned_t TOPPERS_cfg_TARGET_SPNATR = (unsigned_t)(TARGET_SPNATR);
#else
const unsigned_t TOPPERS_cfg_TARGET_SPNATR = (unsigned_t)(0);
#endif
#if defined(TARGET_INTATR)
const unsigned_t TOPPERS_cfg_TARGET_INTATR = (unsigned_t)(TARGET_INTATR);
#else
const unsigned_t TOPPERS_cfg_TARGET_INTATR = (unsigned_t)(0);
#endif
#if defined(TARGET_INHATR)
const unsigned_t TOPPERS_cfg_TARGET_INHATR = (unsigned_t)(TARGET_INHATR);
#else
const unsigned_t TOPPERS_cfg_TARGET_INHATR = (unsigned_t)(0);
#endif
#if defined(TARGET_ISRATR)
const unsigned_t TOPPERS_cfg_TARGET_ISRATR = (unsigned_t)(TARGET_ISRATR);
#else
const unsigned_t TOPPERS_cfg_TARGET_ISRATR = (unsigned_t)(0);
#endif
#if defined(TARGET_EXCATR)
const unsigned_t TOPPERS_cfg_TARGET_EXCATR = (unsigned_t)(TARGET_EXCATR);
#else
const unsigned_t TOPPERS_cfg_TARGET_EXCATR = (unsigned_t)(0);
#endif
#if defined(TARGET_MIN_STKSZ)
const unsigned_t TOPPERS_cfg_TARGET_MIN_STKSZ = (unsigned_t)(TARGET_MIN_STKSZ);
#else
const unsigned_t TOPPERS_cfg_TARGET_MIN_STKSZ = (unsigned_t)(1);
#endif
#if defined(TARGET_MIN_ISTKSZ)
const unsigned_t TOPPERS_cfg_TARGET_MIN_ISTKSZ = (unsigned_t)(TARGET_MIN_ISTKSZ);
#else
const unsigned_t TOPPERS_cfg_TARGET_MIN_ISTKSZ = (unsigned_t)(1);
#endif
#if defined(CHECK_STKSZ_ALIGN)
const unsigned_t TOPPERS_cfg_CHECK_STKSZ_ALIGN = (unsigned_t)(CHECK_STKSZ_ALIGN);
#else
const unsigned_t TOPPERS_cfg_CHECK_STKSZ_ALIGN = (unsigned_t)(1);
#endif
#if defined(CHECK_INTPTR_ALIGN)
const unsigned_t TOPPERS_cfg_CHECK_INTPTR_ALIGN = (unsigned_t)(CHECK_INTPTR_ALIGN);
#else
const unsigned_t TOPPERS_cfg_CHECK_INTPTR_ALIGN = (unsigned_t)(1);
#endif
#if defined(CHECK_INTPTR_NONNULL)
const signed_t TOPPERS_cfg_CHECK_INTPTR_NONNULL = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_CHECK_INTPTR_NONNULL = (signed_t)(false);
#endif
#if defined(CHECK_FUNC_ALIGN)
const unsigned_t TOPPERS_cfg_CHECK_FUNC_ALIGN = (unsigned_t)(CHECK_FUNC_ALIGN);
#else
const unsigned_t TOPPERS_cfg_CHECK_FUNC_ALIGN = (unsigned_t)(1);
#endif
#if defined(CHECK_FUNC_NONNULL)
const signed_t TOPPERS_cfg_CHECK_FUNC_NONNULL = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_CHECK_FUNC_NONNULL = (signed_t)(false);
#endif
#if defined(CHECK_STACK_ALIGN)
const unsigned_t TOPPERS_cfg_CHECK_STACK_ALIGN = (unsigned_t)(CHECK_STACK_ALIGN);
#else
const unsigned_t TOPPERS_cfg_CHECK_STACK_ALIGN = (unsigned_t)(1);
#endif
#if defined(CHECK_STACK_NONNULL)
const signed_t TOPPERS_cfg_CHECK_STACK_NONNULL = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_CHECK_STACK_NONNULL = (signed_t)(false);
#endif
#if defined(CHECK_MPF_ALIGN)
const unsigned_t TOPPERS_cfg_CHECK_MPF_ALIGN = (unsigned_t)(CHECK_MPF_ALIGN);
#else
const unsigned_t TOPPERS_cfg_CHECK_MPF_ALIGN = (unsigned_t)(1);
#endif
#if defined(CHECK_MPF_NONNULL)
const signed_t TOPPERS_cfg_CHECK_MPF_NONNULL = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_CHECK_MPF_NONNULL = (signed_t)(false);
#endif
const unsigned_t TOPPERS_cfg_sizeof_void_ptr = (unsigned_t)(sizeof(void*));
const unsigned_t TOPPERS_cfg_sizeof_uint_t = (unsigned_t)(sizeof(uint_t));
const unsigned_t TOPPERS_cfg_sizeof_size_t = (unsigned_t)(sizeof(size_t));
const unsigned_t TOPPERS_cfg_sizeof_intptr_t = (unsigned_t)(sizeof(intptr_t));
const unsigned_t TOPPERS_cfg_sizeof_ID = (unsigned_t)(sizeof(ID));
const unsigned_t TOPPERS_cfg_sizeof_EXINF = (unsigned_t)(sizeof(EXINF));
const unsigned_t TOPPERS_cfg_sizeof_FP = (unsigned_t)(sizeof(FP));
const unsigned_t TOPPERS_cfg_sizeof_INTNO = (unsigned_t)(sizeof(INTNO));
const unsigned_t TOPPERS_cfg_sizeof_INHNO = (unsigned_t)(sizeof(INHNO));
const unsigned_t TOPPERS_cfg_sizeof_EXCNO = (unsigned_t)(sizeof(EXCNO));
const unsigned_t TOPPERS_cfg_sizeof_TASK = (unsigned_t)(sizeof(TASK));
const unsigned_t TOPPERS_cfg_sizeof_TMEHDR = (unsigned_t)(sizeof(TMEHDR));
const unsigned_t TOPPERS_cfg_sizeof_ISR = (unsigned_t)(sizeof(ISR));
const unsigned_t TOPPERS_cfg_sizeof_INTHDR = (unsigned_t)(sizeof(INTHDR));
const unsigned_t TOPPERS_cfg_sizeof_EXCHDR = (unsigned_t)(sizeof(EXCHDR));
const unsigned_t TOPPERS_cfg_sizeof_INIRTN = (unsigned_t)(sizeof(INIRTN));
const unsigned_t TOPPERS_cfg_sizeof_TERRTN = (unsigned_t)(sizeof(TERRTN));
const unsigned_t TOPPERS_cfg_sizeof_NFYHDR = (unsigned_t)(sizeof(NFYHDR));
const unsigned_t TOPPERS_cfg_sizeof_TINIB = (unsigned_t)(sizeof(TINIB));
const unsigned_t TOPPERS_cfg_offsetof_TINIB_tskatr = (unsigned_t)(offsetof(TINIB,tskatr));
const unsigned_t TOPPERS_cfg_offsetof_TINIB_exinf = (unsigned_t)(offsetof(TINIB,exinf));
const unsigned_t TOPPERS_cfg_offsetof_TINIB_task = (unsigned_t)(offsetof(TINIB,task));
const unsigned_t TOPPERS_cfg_offsetof_TINIB_ipriority = (unsigned_t)(offsetof(TINIB,ipriority));
#if !defined(USE_TSKINICTXB)
const unsigned_t TOPPERS_cfg_offsetof_TINIB_stksz = (unsigned_t)(offsetof(TINIB,stksz));
#endif
#if !defined(USE_TSKINICTXB)
const unsigned_t TOPPERS_cfg_offsetof_TINIB_stk = (unsigned_t)(offsetof(TINIB,stk));
#endif
const unsigned_t TOPPERS_cfg_sizeof_SEMINIB = (unsigned_t)(sizeof(SEMINIB));
const unsigned_t TOPPERS_cfg_offsetof_SEMINIB_sematr = (unsigned_t)(offsetof(SEMINIB,sematr));
const unsigned_t TOPPERS_cfg_offsetof_SEMINIB_isemcnt = (unsigned_t)(offsetof(SEMINIB,isemcnt));
const unsigned_t TOPPERS_cfg_offsetof_SEMINIB_maxsem = (unsigned_t)(offsetof(SEMINIB,maxsem));
const unsigned_t TOPPERS_cfg_sizeof_FLGPTN = (unsigned_t)(sizeof(FLGPTN));
const unsigned_t TOPPERS_cfg_sizeof_FLGINIB = (unsigned_t)(sizeof(FLGINIB));
const unsigned_t TOPPERS_cfg_offsetof_FLGINIB_flgatr = (unsigned_t)(offsetof(FLGINIB,flgatr));
const unsigned_t TOPPERS_cfg_offsetof_FLGINIB_iflgptn = (unsigned_t)(offsetof(FLGINIB,iflgptn));
const unsigned_t TOPPERS_cfg_sizeof_DTQINIB = (unsigned_t)(sizeof(DTQINIB));
const unsigned_t TOPPERS_cfg_offsetof_DTQINIB_dtqatr = (unsigned_t)(offsetof(DTQINIB,dtqatr));
const unsigned_t TOPPERS_cfg_offsetof_DTQINIB_dtqcnt = (unsigned_t)(offsetof(DTQINIB,dtqcnt));
const unsigned_t TOPPERS_cfg_offsetof_DTQINIB_p_dtqmb = (unsigned_t)(offsetof(DTQINIB,p_dtqmb));
const unsigned_t TOPPERS_cfg_sizeof_PDQINIB = (unsigned_t)(sizeof(PDQINIB));
const unsigned_t TOPPERS_cfg_offsetof_PDQINIB_pdqatr = (unsigned_t)(offsetof(PDQINIB,pdqatr));
const unsigned_t TOPPERS_cfg_offsetof_PDQINIB_pdqcnt = (unsigned_t)(offsetof(PDQINIB,pdqcnt));
const unsigned_t TOPPERS_cfg_offsetof_PDQINIB_maxdpri = (unsigned_t)(offsetof(PDQINIB,maxdpri));
const unsigned_t TOPPERS_cfg_offsetof_PDQINIB_p_pdqmb = (unsigned_t)(offsetof(PDQINIB,p_pdqmb));
const unsigned_t TOPPERS_cfg_sizeof_MTXINIB = (unsigned_t)(sizeof(MTXINIB));
const unsigned_t TOPPERS_cfg_offsetof_MTXINIB_mtxatr = (unsigned_t)(offsetof(MTXINIB,mtxatr));
const unsigned_t TOPPERS_cfg_offsetof_MTXINIB_ceilpri = (unsigned_t)(offsetof(MTXINIB,ceilpri));
const unsigned_t TOPPERS_cfg_sizeof_MPFINIB = (unsigned_t)(sizeof(MPFINIB));
const unsigned_t TOPPERS_cfg_offsetof_MPFINIB_mpfatr = (unsigned_t)(offsetof(MPFINIB,mpfatr));
const unsigned_t TOPPERS_cfg_offsetof_MPFINIB_blkcnt = (unsigned_t)(offsetof(MPFINIB,blkcnt));
const unsigned_t TOPPERS_cfg_offsetof_MPFINIB_blksz = (unsigned_t)(offsetof(MPFINIB,blksz));
const unsigned_t TOPPERS_cfg_offsetof_MPFINIB_mpf = (unsigned_t)(offsetof(MPFINIB,mpf));
const unsigned_t TOPPERS_cfg_offsetof_MPFINIB_p_mpfmb = (unsigned_t)(offsetof(MPFINIB,p_mpfmb));
const unsigned_t TOPPERS_cfg_sizeof_CYCINIB = (unsigned_t)(sizeof(CYCINIB));
const unsigned_t TOPPERS_cfg_offsetof_CYCINIB_cycatr = (unsigned_t)(offsetof(CYCINIB,cycatr));
const unsigned_t TOPPERS_cfg_offsetof_CYCINIB_exinf = (unsigned_t)(offsetof(CYCINIB,exinf));
const unsigned_t TOPPERS_cfg_offsetof_CYCINIB_nfyhdr = (unsigned_t)(offsetof(CYCINIB,nfyhdr));
const unsigned_t TOPPERS_cfg_offsetof_CYCINIB_cyctim = (unsigned_t)(offsetof(CYCINIB,cyctim));
const unsigned_t TOPPERS_cfg_offsetof_CYCINIB_cycphs = (unsigned_t)(offsetof(CYCINIB,cycphs));
const unsigned_t TOPPERS_cfg_sizeof_ALMINIB = (unsigned_t)(sizeof(ALMINIB));
const unsigned_t TOPPERS_cfg_offsetof_ALMINIB_almatr = (unsigned_t)(offsetof(ALMINIB,almatr));
const unsigned_t TOPPERS_cfg_offsetof_ALMINIB_exinf = (unsigned_t)(offsetof(ALMINIB,exinf));
const unsigned_t TOPPERS_cfg_offsetof_ALMINIB_nfyhdr = (unsigned_t)(offsetof(ALMINIB,nfyhdr));
#if !defined(OMIT_INITIALIZE_INTERRUPT)
const unsigned_t TOPPERS_cfg_sizeof_INHINIB = (unsigned_t)(sizeof(INHINIB));
#endif
#if !defined(OMIT_INITIALIZE_INTERRUPT)
const unsigned_t TOPPERS_cfg_offsetof_INHINIB_inhno = (unsigned_t)(offsetof(INHINIB,inhno));
#endif
#if !defined(OMIT_INITIALIZE_INTERRUPT)
const unsigned_t TOPPERS_cfg_offsetof_INHINIB_inhatr = (unsigned_t)(offsetof(INHINIB,inhatr));
#endif
#if !defined(OMIT_INITIALIZE_INTERRUPT)
const unsigned_t TOPPERS_cfg_offsetof_INHINIB_int_entry = (unsigned_t)(offsetof(INHINIB,int_entry));
#endif
#if !defined(OMIT_INITIALIZE_INTERRUPT)
const unsigned_t TOPPERS_cfg_sizeof_INTINIB = (unsigned_t)(sizeof(INTINIB));
#endif
#if !defined(OMIT_INITIALIZE_INTERRUPT)
const unsigned_t TOPPERS_cfg_offsetof_INTINIB_intno = (unsigned_t)(offsetof(INTINIB,intno));
#endif
#if !defined(OMIT_INITIALIZE_INTERRUPT)
const unsigned_t TOPPERS_cfg_offsetof_INTINIB_intatr = (unsigned_t)(offsetof(INTINIB,intatr));
#endif
#if !defined(OMIT_INITIALIZE_INTERRUPT)
const unsigned_t TOPPERS_cfg_offsetof_INTINIB_intpri = (unsigned_t)(offsetof(INTINIB,intpri));
#endif
#if !defined(OMIT_INITIALIZE_EXCEPTION)
const unsigned_t TOPPERS_cfg_sizeof_EXCINIB = (unsigned_t)(sizeof(EXCINIB));
#endif
#if !defined(OMIT_INITIALIZE_EXCEPTION)
const unsigned_t TOPPERS_cfg_offsetof_EXCINIB_excno = (unsigned_t)(offsetof(EXCINIB,excno));
#endif
#if !defined(OMIT_INITIALIZE_EXCEPTION)
const unsigned_t TOPPERS_cfg_offsetof_EXCINIB_excatr = (unsigned_t)(offsetof(EXCINIB,excatr));
#endif
#if !defined(OMIT_INITIALIZE_EXCEPTION)
const unsigned_t TOPPERS_cfg_offsetof_EXCINIB_exc_entry = (unsigned_t)(offsetof(EXCINIB,exc_entry));
#endif
const unsigned_t TOPPERS_cfg_sizeof_INIRTNB = (unsigned_t)(sizeof(INIRTNB));
const unsigned_t TOPPERS_cfg_offsetof_INIRTNB_inirtn = (unsigned_t)(offsetof(INIRTNB,inirtn));
const unsigned_t TOPPERS_cfg_offsetof_INIRTNB_exinf = (unsigned_t)(offsetof(INIRTNB,exinf));
const unsigned_t TOPPERS_cfg_sizeof_TERRTNB = (unsigned_t)(sizeof(TERRTNB));
const unsigned_t TOPPERS_cfg_offsetof_TERRTNB_terrtn = (unsigned_t)(offsetof(TERRTNB,terrtn));
const unsigned_t TOPPERS_cfg_offsetof_TERRTNB_exinf = (unsigned_t)(offsetof(TERRTNB,exinf));
const unsigned_t TOPPERS_cfg_TARGET_ARCH_ARM = (unsigned_t)(__TARGET_ARCH_ARM);
#if defined(USE_ARM_MMU)
const signed_t TOPPERS_cfg_USE_ARM_MMU = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_ARM_MMU = (signed_t)(false);
#endif
#if defined(USE_ARM_SSECTION)
const signed_t TOPPERS_cfg_USE_ARM_SSECTION = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_ARM_SSECTION = (signed_t)(false);
#endif
#if defined(USE_ARM_FPU)
const signed_t TOPPERS_cfg_USE_ARM_FPU = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_ARM_FPU = (signed_t)(false);
#endif
#if defined(USE_ARM_FPU_D32)
const signed_t TOPPERS_cfg_USE_ARM_FPU_D32 = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_ARM_FPU_D32 = (signed_t)(false);
#endif
#if defined(USE_INTCFG_TABLE)
const signed_t TOPPERS_cfg_USE_INTCFG_TABLE = (signed_t)(true);
#else
const signed_t TOPPERS_cfg_USE_INTCFG_TABLE = (signed_t)(false);
#endif
const unsigned_t TOPPERS_cfg_TA_FPU = (unsigned_t)(TA_FPU);
const unsigned_t TOPPERS_cfg_sizeof_TCB = (unsigned_t)(sizeof(TCB));
const unsigned_t TOPPERS_cfg_offsetof_TCB_p_tinib = (unsigned_t)(offsetof(TCB,p_tinib));
const unsigned_t TOPPERS_cfg_offsetof_TCB_sp = (unsigned_t)(offsetof(TCB,tskctxb.sp));
const unsigned_t TOPPERS_cfg_offsetof_TCB_pc = (unsigned_t)(offsetof(TCB,tskctxb.pc));
const unsigned_t TOPPERS_cfg_offsetof_T_EXCINF_cpsr = (unsigned_t)(offsetof(T_EXCINF,cpsr));
const unsigned_t TOPPERS_cfg_sizeof_PCB = (unsigned_t)(sizeof(PCB));
const unsigned_t TOPPERS_cfg_offsetof_PCB_p_runtsk = (unsigned_t)(offsetof(PCB,p_runtsk));
const unsigned_t TOPPERS_cfg_offsetof_PCB_p_schedtsk = (unsigned_t)(offsetof(PCB,p_schedtsk));
const unsigned_t TOPPERS_cfg_offsetof_PCB_excpt_nest_count = (unsigned_t)(offsetof(PCB,target_pcb.excpt_nest_count));
const unsigned_t TOPPERS_cfg_offsetof_PCB_istkpt = (unsigned_t)(offsetof(PCB,target_pcb.istkpt));
const unsigned_t TOPPERS_cfg_offsetof_PCB_idstkpt = (unsigned_t)(offsetof(PCB,target_pcb.idstkpt));
const unsigned_t TOPPERS_cfg_offsetof_PCB_p_exc_tbl = (unsigned_t)(offsetof(PCB,target_pcb.p_exc_tbl));
const unsigned_t TOPPERS_cfg_offsetof_PCB_p_inh_tbl = (unsigned_t)(offsetof(PCB,target_pcb.p_inh_tbl));


#line 11 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_1 = 1;
#line 11 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_iniatr_1 = (unsigned_t)(TA_NULL);
#line 10 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_1 = (signed_t)(CLS_PRC1);

#line 12 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_2 = 2;
#line 12 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_teratr_2 = (unsigned_t)(TA_NULL);
#line 10 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_2 = (signed_t)(CLS_PRC1);

#line 14 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_3 = 3;
#line 14 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_3 = (unsigned_t)(INTNO_TIMER_PRC1);
#line 14 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_3 = (unsigned_t)(TA_ENAINT|INTATR_TIMER);
#line 14 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_3 = (signed_t)(INTPRI_TIMER);
#line 10 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_3 = (signed_t)(CLS_PRC1);

#line 15 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_4 = 4;
#line 15 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_4 = (unsigned_t)(INHNO_TIMER_PRC1);
#line 15 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_4 = (unsigned_t)(TA_NULL);
#line 10 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_4 = (signed_t)(CLS_PRC1);

#line 18 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 2

#line 20 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_5 = 5;
#line 20 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_iniatr_5 = (unsigned_t)(TA_NULL);
#line 19 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_5 = (signed_t)(CLS_PRC2);

#line 21 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_6 = 6;
#line 21 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_teratr_6 = (unsigned_t)(TA_NULL);
#line 19 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_6 = (signed_t)(CLS_PRC2);

#line 23 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_7 = 7;
#line 23 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_7 = (unsigned_t)(INTNO_TIMER_PRC2);
#line 23 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_7 = (unsigned_t)(TA_ENAINT|INTATR_TIMER);
#line 23 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_7 = (signed_t)(INTPRI_TIMER);
#line 19 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_7 = (signed_t)(CLS_PRC2);

#line 24 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_8 = 8;
#line 24 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_8 = (unsigned_t)(INHNO_TIMER_PRC2);
#line 24 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_8 = (unsigned_t)(TA_NULL);
#line 19 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_8 = (signed_t)(CLS_PRC2);

#line 26 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif

#line 28 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 3

#line 30 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_9 = 9;
#line 30 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_iniatr_9 = (unsigned_t)(TA_NULL);
#line 29 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_9 = (signed_t)(CLS_PRC3);

#line 31 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_10 = 10;
#line 31 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_teratr_10 = (unsigned_t)(TA_NULL);
#line 29 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_10 = (signed_t)(CLS_PRC3);

#line 33 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_11 = 11;
#line 33 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_11 = (unsigned_t)(INTNO_TIMER_PRC3);
#line 33 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_11 = (unsigned_t)(TA_ENAINT|INTATR_TIMER);
#line 33 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_11 = (signed_t)(INTPRI_TIMER);
#line 29 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_11 = (signed_t)(CLS_PRC3);

#line 34 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_12 = 12;
#line 34 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_12 = (unsigned_t)(INHNO_TIMER_PRC3);
#line 34 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_12 = (unsigned_t)(TA_NULL);
#line 29 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_12 = (signed_t)(CLS_PRC3);

#line 36 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif

#line 38 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 4

#line 40 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_13 = 13;
#line 40 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_iniatr_13 = (unsigned_t)(TA_NULL);
#line 39 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_13 = (signed_t)(CLS_PRC4);

#line 41 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_14 = 14;
#line 41 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_teratr_14 = (unsigned_t)(TA_NULL);
#line 39 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_14 = (signed_t)(CLS_PRC4);

#line 43 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_15 = 15;
#line 43 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_15 = (unsigned_t)(INTNO_TIMER_PRC4);
#line 43 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_15 = (unsigned_t)(TA_ENAINT|INTATR_TIMER);
#line 43 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_15 = (signed_t)(INTPRI_TIMER);
#line 39 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_15 = (signed_t)(CLS_PRC4);

#line 44 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_16 = 16;
#line 44 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_16 = (unsigned_t)(INHNO_TIMER_PRC4);
#line 44 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_16 = (unsigned_t)(TA_NULL);
#line 39 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_16 = (signed_t)(CLS_PRC4);

#line 46 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif

#line 52 "../arch/arm_gcc/common/mpcore_timer.cfg"
#ifdef USE_MPCORE_WDG_OVRTIMER

#line 53 "../arch/arm_gcc/common/mpcore_timer.cfg"
#ifdef TOPPERS_SUPPORT_OVRHDR

#line 56 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_17 = 17;
#line 56 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_iniatr_17 = (unsigned_t)(TA_NULL);
#line 55 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_17 = (signed_t)(CLS_PRC1);

#line 57 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_18 = 18;
#line 57 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_teratr_18 = (unsigned_t)(TA_NULL);
#line 55 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_18 = (signed_t)(CLS_PRC1);

#line 59 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_19 = 19;
#line 59 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_19 = (unsigned_t)(INTNO_OVRTIMER_PRC1);
#line 59 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_19 = (unsigned_t)(TA_ENAINT|INTATR_OVRTIMER);
#line 59 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_19 = (signed_t)(INTPRI_OVRTIMER);
#line 55 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_19 = (signed_t)(CLS_PRC1);

#line 61 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_20 = 20;
#line 61 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_20 = (unsigned_t)(INHNO_OVRTIMER_PRC1);
#line 61 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_20 = (unsigned_t)(TA_NULL);
#line 55 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_20 = (signed_t)(CLS_PRC1);

#line 65 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 2

#line 67 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_21 = 21;
#line 67 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_iniatr_21 = (unsigned_t)(TA_NULL);
#line 66 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_21 = (signed_t)(CLS_PRC2);

#line 68 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_22 = 22;
#line 68 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_teratr_22 = (unsigned_t)(TA_NULL);
#line 66 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_22 = (signed_t)(CLS_PRC2);

#line 69 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_23 = 23;
#line 69 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_23 = (unsigned_t)(INTNO_OVRTIMER_PRC2);
#line 69 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_23 = (unsigned_t)(TA_ENAINT|INTATR_OVRTIMER);
#line 69 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_23 = (signed_t)(INTPRI_OVRTIMER);
#line 66 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_23 = (signed_t)(CLS_PRC2);

#line 71 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_24 = 24;
#line 71 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_24 = (unsigned_t)(INHNO_OVRTIMER_PRC2);
#line 71 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_24 = (unsigned_t)(TA_NULL);
#line 66 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_24 = (signed_t)(CLS_PRC2);

#line 74 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif

#line 76 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 3

#line 78 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_25 = 25;
#line 78 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_iniatr_25 = (unsigned_t)(TA_NULL);
#line 77 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_25 = (signed_t)(CLS_PRC3);

#line 79 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_26 = 26;
#line 79 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_teratr_26 = (unsigned_t)(TA_NULL);
#line 77 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_26 = (signed_t)(CLS_PRC3);

#line 80 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_27 = 27;
#line 80 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_27 = (unsigned_t)(INTNO_OVRTIMER_PRC3);
#line 80 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_27 = (unsigned_t)(TA_ENAINT|INTATR_OVRTIMER);
#line 80 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_27 = (signed_t)(INTPRI_OVRTIMER);
#line 77 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_27 = (signed_t)(CLS_PRC3);

#line 82 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_28 = 28;
#line 82 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_28 = (unsigned_t)(INHNO_OVRTIMER_PRC3);
#line 82 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_28 = (unsigned_t)(TA_NULL);
#line 77 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_28 = (signed_t)(CLS_PRC3);

#line 85 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif

#line 87 "../arch/arm_gcc/common/mpcore_timer.cfg"
#if TNUM_PRCID >= 4

#line 89 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_29 = 29;
#line 89 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_iniatr_29 = (unsigned_t)(TA_NULL);
#line 88 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_29 = (signed_t)(CLS_PRC4);

#line 90 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_30 = 30;
#line 90 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_teratr_30 = (unsigned_t)(TA_NULL);
#line 88 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_30 = (signed_t)(CLS_PRC4);

#line 91 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_31 = 31;
#line 91 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_31 = (unsigned_t)(INTNO_OVRTIMER_PRC4);
#line 91 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_31 = (unsigned_t)(TA_ENAINT|INTATR_OVRTIMER);
#line 91 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_31 = (signed_t)(INTPRI_OVRTIMER);
#line 88 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_31 = (signed_t)(CLS_PRC4);

#line 93 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_static_api_32 = 32;
#line 93 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_32 = (unsigned_t)(INHNO_OVRTIMER_PRC4);
#line 93 "../arch/arm_gcc/common/mpcore_timer.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_32 = (unsigned_t)(TA_NULL);
#line 88 "../arch/arm_gcc/common/mpcore_timer.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_32 = (signed_t)(CLS_PRC4);

#line 96 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif

#line 98 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif

#line 99 "../arch/arm_gcc/common/mpcore_timer.cfg"
#endif

#line 11 "../arch/arm_gcc/common/gic_ipi.cfg"
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER

#line 12 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_33 = 33;
#line 12 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_33 = (unsigned_t)(INHNO_IPI_DISPATCH_PRC1);
#line 12 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_33 = (unsigned_t)(TA_NULL);
#line 10 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_33 = (signed_t)(CLS_PRC1);

#line 14 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 15 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_34 = 34;
#line 15 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_34 = (unsigned_t)(INTNO_IPI_DISPATCH_PRC1);
#line 15 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_34 = (unsigned_t)(TA_ENAINT);
#line 15 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_34 = (signed_t)(INTPRI_IPI_DISPATCH_PRC1);
#line 10 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_34 = (signed_t)(CLS_PRC1);

#line 19 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 2

#line 21 "../arch/arm_gcc/common/gic_ipi.cfg"
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER

#line 22 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_35 = 35;
#line 22 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_35 = (unsigned_t)(INHNO_IPI_DISPATCH_PRC2);
#line 22 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_35 = (unsigned_t)(TA_NULL);
#line 20 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_35 = (signed_t)(CLS_PRC2);

#line 24 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 25 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_36 = 36;
#line 25 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_36 = (unsigned_t)(INTNO_IPI_DISPATCH_PRC2);
#line 25 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_36 = (unsigned_t)(TA_ENAINT);
#line 25 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_36 = (signed_t)(INTPRI_IPI_DISPATCH_PRC2);
#line 20 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_36 = (signed_t)(CLS_PRC2);

#line 28 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 30 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 3

#line 32 "../arch/arm_gcc/common/gic_ipi.cfg"
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER

#line 33 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_37 = 37;
#line 33 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_37 = (unsigned_t)(INHNO_IPI_DISPATCH_PRC3);
#line 33 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_37 = (unsigned_t)(TA_NULL);
#line 31 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_37 = (signed_t)(CLS_PRC3);

#line 35 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 36 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_38 = 38;
#line 36 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_38 = (unsigned_t)(INTNO_IPI_DISPATCH_PRC3);
#line 36 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_38 = (unsigned_t)(TA_ENAINT);
#line 36 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_38 = (signed_t)(INTPRI_IPI_DISPATCH_PRC3);
#line 31 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_38 = (signed_t)(CLS_PRC3);

#line 39 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 41 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 4

#line 43 "../arch/arm_gcc/common/gic_ipi.cfg"
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER

#line 44 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_39 = 39;
#line 44 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_39 = (unsigned_t)(INHNO_IPI_DISPATCH_PRC4);
#line 44 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_39 = (unsigned_t)(TA_NULL);
#line 42 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_39 = (signed_t)(CLS_PRC4);

#line 46 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 47 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_40 = 40;
#line 47 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_40 = (unsigned_t)(INTNO_IPI_DISPATCH_PRC4);
#line 47 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_40 = (unsigned_t)(TA_ENAINT);
#line 47 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_40 = (signed_t)(INTPRI_IPI_DISPATCH_PRC4);
#line 42 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_40 = (signed_t)(CLS_PRC4);

#line 50 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 56 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_41 = 41;
#line 56 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_41 = (unsigned_t)(INHNO_IPI_EXT_KER_PRC1);
#line 56 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_41 = (unsigned_t)(TA_NULL);
#line 55 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_41 = (signed_t)(CLS_PRC1);

#line 58 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_42 = 42;
#line 58 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_42 = (unsigned_t)(INTNO_IPI_EXT_KER_PRC1);
#line 58 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_42 = (unsigned_t)(TA_ENAINT);
#line 58 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_42 = (signed_t)(INTPRI_IPI_EXT_KER_PRC1);
#line 55 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_42 = (signed_t)(CLS_PRC1);

#line 62 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 2

#line 64 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_43 = 43;
#line 64 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_43 = (unsigned_t)(INHNO_IPI_EXT_KER_PRC2);
#line 64 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_43 = (unsigned_t)(TA_NULL);
#line 63 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_43 = (signed_t)(CLS_PRC2);

#line 66 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_44 = 44;
#line 66 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_44 = (unsigned_t)(INTNO_IPI_EXT_KER_PRC2);
#line 66 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_44 = (unsigned_t)(TA_ENAINT);
#line 66 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_44 = (signed_t)(INTPRI_IPI_EXT_KER_PRC2);
#line 63 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_44 = (signed_t)(CLS_PRC2);

#line 69 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 71 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 3

#line 73 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_45 = 45;
#line 73 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_45 = (unsigned_t)(INHNO_IPI_EXT_KER_PRC3);
#line 73 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_45 = (unsigned_t)(TA_NULL);
#line 72 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_45 = (signed_t)(CLS_PRC3);

#line 75 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_46 = 46;
#line 75 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_46 = (unsigned_t)(INTNO_IPI_EXT_KER_PRC3);
#line 75 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_46 = (unsigned_t)(TA_ENAINT);
#line 75 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_46 = (signed_t)(INTPRI_IPI_EXT_KER_PRC3);
#line 72 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_46 = (signed_t)(CLS_PRC3);

#line 78 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 80 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 4

#line 82 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_47 = 47;
#line 82 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_47 = (unsigned_t)(INHNO_IPI_EXT_KER_PRC4);
#line 82 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_47 = (unsigned_t)(TA_NULL);
#line 81 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_47 = (signed_t)(CLS_PRC4);

#line 84 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_48 = 48;
#line 84 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_48 = (unsigned_t)(INTNO_IPI_EXT_KER_PRC4);
#line 84 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_48 = (unsigned_t)(TA_ENAINT);
#line 84 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_48 = (signed_t)(INTPRI_IPI_EXT_KER_PRC4);
#line 81 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_48 = (signed_t)(CLS_PRC4);

#line 87 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 93 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_49 = 49;
#line 93 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_49 = (unsigned_t)(INHNO_IPI_SET_HRT_EVT_PRC1);
#line 93 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_49 = (unsigned_t)(TA_NULL);
#line 92 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_49 = (signed_t)(CLS_PRC1);

#line 95 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_50 = 50;
#line 95 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_50 = (unsigned_t)(INTNO_IPI_SET_HRT_EVT_PRC1);
#line 95 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_50 = (unsigned_t)(TA_ENAINT);
#line 95 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_50 = (signed_t)(INTPRI_IPI_SET_HRT_EVT_PRC1);
#line 92 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_50 = (signed_t)(CLS_PRC1);

#line 99 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 2

#line 101 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_51 = 51;
#line 101 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_51 = (unsigned_t)(INHNO_IPI_SET_HRT_EVT_PRC2);
#line 101 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_51 = (unsigned_t)(TA_NULL);
#line 100 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_51 = (signed_t)(CLS_PRC2);

#line 103 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_52 = 52;
#line 103 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_52 = (unsigned_t)(INTNO_IPI_SET_HRT_EVT_PRC2);
#line 103 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_52 = (unsigned_t)(TA_ENAINT);
#line 103 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_52 = (signed_t)(INTPRI_IPI_SET_HRT_EVT_PRC2);
#line 100 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_52 = (signed_t)(CLS_PRC2);

#line 106 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 108 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 3

#line 110 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_53 = 53;
#line 110 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_53 = (unsigned_t)(INHNO_IPI_SET_HRT_EVT_PRC3);
#line 110 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_53 = (unsigned_t)(TA_NULL);
#line 109 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_53 = (signed_t)(CLS_PRC3);

#line 112 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_54 = 54;
#line 112 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_54 = (unsigned_t)(INTNO_IPI_SET_HRT_EVT_PRC3);
#line 112 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_54 = (unsigned_t)(TA_ENAINT);
#line 112 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_54 = (signed_t)(INTPRI_IPI_SET_HRT_EVT_PRC3);
#line 109 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_54 = (signed_t)(CLS_PRC3);

#line 115 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 117 "../arch/arm_gcc/common/gic_ipi.cfg"
#if TNUM_PRCID >= 4

#line 119 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_55 = 55;
#line 119 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhno_55 = (unsigned_t)(INHNO_IPI_SET_HRT_EVT_PRC4);
#line 119 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_inhatr_55 = (unsigned_t)(TA_NULL);
#line 118 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_55 = (signed_t)(CLS_PRC4);

#line 121 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_static_api_56 = 56;
#line 121 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_56 = (unsigned_t)(INTNO_IPI_SET_HRT_EVT_PRC4);
#line 121 "../arch/arm_gcc/common/gic_ipi.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_56 = (unsigned_t)(TA_ENAINT);
#line 121 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_56 = (signed_t)(INTPRI_IPI_SET_HRT_EVT_PRC4);
#line 118 "../arch/arm_gcc/common/gic_ipi.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_56 = (signed_t)(CLS_PRC4);

#line 124 "../arch/arm_gcc/common/gic_ipi.cfg"
#endif

#line 24 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_57 = 57;
#define TSKID_tTask_rProcessor1Migratable_LogTask_Task	(<>)
#line 24 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_tskatr_57 = (unsigned_t)(TA_ACT);
#line 24 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_itskpri_57 = (signed_t)(3);
#line 24 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_stksz_57 = (unsigned_t)(4096);
#line 22 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_57 = (signed_t)(CLS_ALL_PRC1);

#line 29 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_58 = 58;
#define SEMID_tSemaphore_rProcessor1Migratable_SerialPort1_ReceiveSemaphore	(<>)
#line 29 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_sematr_58 = (unsigned_t)(TA_TPRI);
#line 29 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_isemcnt_58 = (unsigned_t)(0);
#line 29 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_maxsem_58 = (unsigned_t)(1);
#line 27 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_58 = (signed_t)(CLS_ALL_PRC1);

#line 31 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_59 = 59;
#define SEMID_tSemaphore_rProcessor1Migratable_SerialPort1_SendSemaphore	(<>)
#line 31 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_sematr_59 = (unsigned_t)(TA_TPRI);
#line 31 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_isemcnt_59 = (unsigned_t)(1);
#line 31 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_maxsem_59 = (unsigned_t)(1);
#line 27 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_59 = (signed_t)(CLS_ALL_PRC1);

#line 36 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_60 = 60;
#line 36 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_60 = (unsigned_t)(INTNO_SIO);
#line 36 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_intatr_60 = (unsigned_t)(TA_NULL);
#line 36 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_intpri_60 = (signed_t)(INTPRI_SIO);
#line 34 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_60 = (signed_t)(CLS_PRC1);

#line 41 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_61 = 61;
#define ISRID_tISR_rProcessor1Migratable_SIOPortTarget1_ISRInstance	(<>)
#line 41 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_isratr_61 = (unsigned_t)(TA_NULL);
#line 41 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_intno_61 = (unsigned_t)(INTNO_SIO);
#line 41 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_isrpri_61 = (signed_t)(ISRPRI_SIO);
#line 39 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_61 = (signed_t)(CLS_PRC1);

#line 46 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_62 = 62;
#line 46 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_iniatr_62 = (unsigned_t)(TA_NULL);
#line 44 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_62 = (signed_t)(CLS_PRC1);

#line 51 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_63 = 63;
#line 51 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_teratr_63 = (unsigned_t)(TA_NULL);
#line 49 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_63 = (signed_t)(CLS_PRC1);

#line 53 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_64 = 64;
#line 53 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_teratr_64 = (unsigned_t)(TA_NULL);
#line 49 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_64 = (signed_t)(CLS_PRC1);

#line 58 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_65 = 65;
#define TSKID_1_1	(<>)
#line 58 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_tskatr_65 = (unsigned_t)(TA_ACT);
#line 58 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_itskpri_65 = (signed_t)(4);
#line 58 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_stksz_65 = (unsigned_t)(2048);
#line 56 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_65 = (signed_t)(CLS_PRC1);

#line 62 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_66 = 66;
#define TSKID_MIG	(<>)
#line 62 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_tskatr_66 = (unsigned_t)(TA_NULL);
#line 62 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_itskpri_66 = (signed_t)(6);
#line 62 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_stksz_66 = (unsigned_t)(2048);
#line 60 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_66 = (signed_t)(CLS_ALL_PRC1);

#line 66 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_67 = 67;
#define TSKID_2_1	(<>)
#line 66 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_tskatr_67 = (unsigned_t)(TA_ACT);
#line 66 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_itskpri_67 = (signed_t)(7);
#line 66 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_stksz_67 = (unsigned_t)(2048);
#line 64 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_67 = (signed_t)(CLS_PRC2);

#line 68 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_static_api_68 = 68;
#define TSKID_2_2	(<>)
#line 68 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_tskatr_68 = (unsigned_t)(TA_NULL);
#line 68 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_itskpri_68 = (signed_t)(10);
#line 68 "./gen/tecsgen.cfg"
const unsigned_t TOPPERS_cfg_valueof_stksz_68 = (unsigned_t)(2048);
#line 64 "./gen/tecsgen.cfg"
const signed_t TOPPERS_cfg_valueof_CLASS_68 = (signed_t)(CLS_PRC2);

