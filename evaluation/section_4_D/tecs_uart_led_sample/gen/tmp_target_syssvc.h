/*  1 "./gen/tmp_C_src.c" */
/*  1 "<built-in>" */
/*  1 "<command-line>" */
/*  1 "./gen/tmp_C_src.c" */
/*  36 "./gen/tmp_C_src.c" */
typedef struct { int dummy; } va_list;


/*  1 "../../../target/zybo_z7_gcc/target_syssvc.h" 1 */
/*  16 "../../../target/zybo_z7_gcc/target_syssvc.h" */
/*  1 "../../../target/zybo_z7_gcc/zybo_z7.h" 1 */
/*  17 "../../../target/zybo_z7_gcc/target_syssvc.h" 2 */
/*  1 "../../../arch/arm_gcc/zynq7000/zynq7000.h" 1 */
/*  18 "../../../target/zybo_z7_gcc/target_syssvc.h" 2 */
/*  34 "../../../target/zybo_z7_gcc/target_syssvc.h" */
extern void target_fput_log(char c);
/*  68 "../../../target/zybo_z7_gcc/target_syssvc.h" */
/*  1 "../../../arch/arm_gcc/common/core_syssvc.h" 1 */
/*  52 "../../../arch/arm_gcc/common/core_syssvc.h" */
/*  1 "../../../include/t_stddef.h" 1 */
/*  65 "../../../include/t_stddef.h" */
/*  1 "../../../target/zybo_z7_gcc/target_stddef.h" 1 */
/*  22 "../../../target/zybo_z7_gcc/target_stddef.h" */
/*  1 "../../../arch/arm_gcc/zynq7000/chip_stddef.h" 1 */
/*  23 "../../../arch/arm_gcc/zynq7000/chip_stddef.h" */
/*  1 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stdint.h" 1 3 4 */
/*  34 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stdint.h" 3 4 */

/*  34 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stdint.h" 3 4 */
typedef signed char int8_t;


typedef short int int16_t;


typedef long int int32_t;


typedef long long int int64_t;


typedef unsigned char uint8_t;


typedef short unsigned int uint16_t;


typedef long unsigned int uint32_t;


typedef long long unsigned int uint64_t;




typedef signed char int_least8_t;
typedef short int int_least16_t;
typedef long int int_least32_t;
typedef long long int int_least64_t;
typedef unsigned char uint_least8_t;
typedef short unsigned int uint_least16_t;
typedef long unsigned int uint_least32_t;
typedef long long unsigned int uint_least64_t;



typedef int int_fast8_t;
typedef int int_fast16_t;
typedef int int_fast32_t;
typedef long long int int_fast64_t;
typedef unsigned int uint_fast8_t;
typedef unsigned int uint_fast16_t;
typedef unsigned int uint_fast32_t;
typedef long long unsigned int uint_fast64_t;




typedef int intptr_t;


typedef unsigned int uintptr_t;




typedef long long int intmax_t;
typedef long long unsigned int uintmax_t;
/*  24 "../../../arch/arm_gcc/zynq7000/chip_stddef.h" 2 */



/*  1 "../../../arch/gcc/tool_stddef.h" 1 */
/*  81 "../../../arch/gcc/tool_stddef.h" */
/*  1 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 1 3 4 */
/*  143 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 3 4 */
typedef int ptrdiff_t;
/*  209 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 3 4 */
typedef unsigned int size_t;
/*  321 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 3 4 */
typedef unsigned int wchar_t;
/*  415 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 3 4 */
typedef struct {
  long long __max_align_ll ;
  long double __max_align_ld ;
/*  426 "/usr/lib/gcc/arm-none-eabi/10.3.1/include/stddef.h" 3 4 */
} max_align_t;
/*  82 "../../../arch/gcc/tool_stddef.h" 2 */
/*  1 "/usr/lib/gcc/arm-none-eabi/10.3.1/include-fixed/limits.h" 1 3 4 */
/*  83 "../../../arch/gcc/tool_stddef.h" 2 */
/*  193 "../../../arch/gcc/tool_stddef.h" */

/*  193 "../../../arch/gcc/tool_stddef.h" */
typedef float float32_t;
typedef double double64_t;
/*  28 "../../../arch/arm_gcc/zynq7000/chip_stddef.h" 2 */




/*  1 "../../../arch/arm_gcc/common/core_stddef.h" 1 */
/*  33 "../../../arch/arm_gcc/zynq7000/chip_stddef.h" 2 */
/*  23 "../../../target/zybo_z7_gcc/target_stddef.h" 2 */






static inline void
TOPPERS_assert_abort(void)
{
/*  46 "../../../target/zybo_z7_gcc/target_stddef.h" */
 while (1) ;
}
/*  66 "../../../include/t_stddef.h" 2 */
/*  84 "../../../include/t_stddef.h" */
struct TOPPERS_dummy_t { int TOPPERS_dummy_field; };
typedef void (*TOPPERS_fp_t)(struct TOPPERS_dummy_t);






typedef int bool_t;

typedef signed int int_t;
typedef unsigned int uint_t;

typedef signed long long_t;
typedef unsigned long ulong_t;

typedef int_t FN;
typedef int_t ER;
typedef int_t ID;
typedef uint_t ATR;
typedef uint_t STAT;
typedef uint_t MODE;
typedef int_t PRI;
typedef uint32_t TMO;
typedef intptr_t EXINF;
typedef uint32_t RELTIM;

typedef uint64_t SYSTIM;



typedef uint32_t PRCTIM;

typedef uint32_t HRTCNT;







typedef TOPPERS_fp_t FP;

typedef int_t ER_BOOL;
typedef int_t ER_ID;
typedef int_t ER_UINT;

typedef uintptr_t MB_T;

typedef uint32_t ACPTN;
typedef struct acvct {
 ACPTN acptn1;
 ACPTN acptn2;
 ACPTN acptn3;
 ACPTN acptn4;
} ACVCT;
/*  53 "../../../arch/arm_gcc/common/core_syssvc.h" 2 */
/*  1 "../../../arch/arm_gcc/common/arm.h" 1 */
/*  55 "../../../arch/arm_gcc/common/arm.h" */
/*  1 "../../../arch/arm_gcc/common/arm_insn.h" 1 */
/*  59 "../../../arch/arm_gcc/common/arm_insn.h" */
static inline uint32_t
count_leading_zero(uint32_t val)
{
 uint32_t count;

 __asm__ volatile("clz %0, %1" : "=r"(count) : "r"(val));
 return(count);
}
/*  81 "../../../arch/arm_gcc/common/arm_insn.h" */
static inline uint32_t
current_cpsr(void)
{
 uint32_t cpsr;

 __asm__ volatile("mrs %0, cpsr" : "=r"(cpsr));
 return(cpsr);
}




static inline void
set_cpsr(uint32_t cpsr)
{
 __asm__ volatile("msr cpsr_cxsf, %0" : : "r"(cpsr) : "memory","cc");
}
/*  132 "../../../arch/arm_gcc/common/arm_insn.h" */
static inline void
disable_irq(void)
{
 __asm__ volatile("cpsid i");
}




static inline void
enable_irq(void)
{
 __asm__ volatile("cpsie i");
}




static inline void
disable_fiq(void)
{
 __asm__ volatile("cpsid f");
}




static inline void
enable_fiq(void)
{
 __asm__ volatile("cpsie f");
}




static inline void
disable_fiq_irq(void)
{
 __asm__ volatile("cpsid fi");
}




static inline void
enable_fiq_irq(void)
{
 __asm__ volatile("cpsie fi");
}




static inline bool_t
test_and_set_uint32(uint32_t *p_var)
{
 bool_t failed;

 __asm__ volatile("ldrex	%0, [%1]			\n"
 "	cmp		%0, #0				\n"
 "	strexeq	%0, %2, [%1]		\n"
 : "=&r"(failed) : "r"(p_var),"r"(1) : "cc");
 return(failed);
}




static inline void
arm_wait_for_event(void)
{
 __asm__ volatile("wfe");
}




static inline void
arm_send_event(void)
{
 __asm__ volatile("sev");
}
/*  375 "../../../arch/arm_gcc/common/arm_insn.h" */
static inline void
data_memory_barrier(void)
{





 __asm__ volatile("dmb":::"memory");

}







static inline void
data_sync_barrier(void)
{





 __asm__ volatile("dsb":::"memory");

}
/*  413 "../../../arch/arm_gcc/common/arm_insn.h" */
static inline void
inst_sync_barrier(void)
{





 __asm__ volatile("isb":::"memory");

}
/*  447 "../../../arch/arm_gcc/common/arm_insn.h" */
static inline uint32_t
current_fpexc(void)
{
 uint32_t fpexc;

 __asm__ volatile("vmrs %0, fpexc" : "=r"(fpexc));
 return(fpexc);
}




static inline void
set_fpexc(uint32_t fpexc)
{
 __asm__ volatile("vmsr fpexc, %0" : : "r"(fpexc));
}
/*  56 "../../../arch/arm_gcc/common/arm.h" 2 */
/*  260 "../../../arch/arm_gcc/common/arm.h" */
static inline void
arm_set_high_vectors(void)
{
 uint32_t reg;

 __asm__ volatile("mrc p15, 0, %0, c1, c0, 0":"=r"(reg));
 reg |= (0x00002000U);
 __asm__ volatile("mcr p15, 0, %0, c1, c0, 0"::"r"(reg));
}




static inline void
arm_set_low_vectors(void)
{
 uint32_t reg;

 __asm__ volatile("mrc p15, 0, %0, c1, c0, 0":"=r"(reg));
 reg &= ~(0x00002000U);
 __asm__ volatile("mcr p15, 0, %0, c1, c0, 0"::"r"(reg));
}




static inline void
arm_enable_bp(void)
{
 uint32_t reg;

 __asm__ volatile("mrc p15, 0, %0, c1, c0, 0":"=r"(reg));
 reg |= (0x00000800U);
 __asm__ volatile("mcr p15, 0, %0, c1, c0, 0"::"r"(reg));
}




static inline void
arm_disable_bp(void)
{
 uint32_t reg;

 __asm__ volatile("mrc p15, 0, %0, c1, c0, 0":"=r"(reg));
 reg &= ~(0x00000800U);
 __asm__ volatile("mcr p15, 0, %0, c1, c0, 0"::"r"(reg));
}
/*  316 "../../../arch/arm_gcc/common/arm.h" */
static inline uint_t
get_my_prcidx(void)
{
 uint32_t reg;

 __asm__ volatile("mrc p15, 0, %0, c0, c0, 5":"=r"(reg));
 return((uint_t)(reg & 0xffU));
}
/*  332 "../../../arch/arm_gcc/common/arm.h" */
extern void arm_enable_icache(void);
extern void arm_disable_icache(void);
extern void arm_enable_dcache(void);
extern void arm_disable_dcache(void);




static inline void
arm_enable_cache(void)
{
 arm_enable_icache();
 arm_enable_dcache();
}




static inline void
arm_disable_cache(void)
{
 arm_disable_icache();
 arm_disable_dcache();
}





extern void armv7_invalidate_dcache(void);
extern void armv7_clean_and_invalidate_dcache(void);





static inline void
arm_invalidate_dcache(void)
{




 armv7_invalidate_dcache();

}




static inline void
arm_clean_and_invalidate_dcache(void)
{




 armv7_clean_and_invalidate_dcache();

}




static inline void
arm_invalidate_icache(void)
{
 __asm__ volatile("mcr p15, 0, %0, c7, c5, 0"::"r"(0));
}




static inline void
arm_invalidate_bp(void)
{
 __asm__ volatile("mcr p15, 0, %0, c7, c5, 6"::"r"(0));
 data_sync_barrier();
 inst_sync_barrier();
}




static inline void
arm_invalidate_tlb(void)
{
 __asm__ volatile("mcr p15, 0, %0, c8, c7, 0"::"r"(0));
 data_sync_barrier();
}
/*  54 "../../../arch/arm_gcc/common/core_syssvc.h" 2 */
/*  69 "../../../target/zybo_z7_gcc/target_syssvc.h" 2 */
/*  40 "./gen/tmp_C_src.c" 2 */
