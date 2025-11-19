/*  1 "./gen/tmp_C_src.c" */
/*  1 "<built-in>" */
/*  1 "<command-line>" */
/*  1 "./gen/tmp_C_src.c" */
/*  36 "./gen/tmp_C_src.c" */
typedef struct { int dummy; } va_list;


/*  1 "../../../arch/arm_gcc/zynq7000/tXUartPs.h" 1 */
/*  108 "../../../arch/arm_gcc/zynq7000/tXUartPs.h" */
/*  1 "../../../include/sil.h" 1 */
/*  63 "../../../include/sil.h" */
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
/*  64 "../../../include/sil.h" 2 */




/*  1 "../../../target/zybo_z7_gcc/target_sil.h" 1 */
/*  22 "../../../target/zybo_z7_gcc/target_sil.h" */
/*  1 "../../../arch/arm_gcc/common/core_sil.h" 1 */
/*  86 "../../../arch/arm_gcc/common/core_sil.h" */
static inline uint32_t
TOPPERS_current_cpsr(void)
{
 uint32_t cpsr;

 __asm__ volatile("mrs %0, cpsr" : "=r"(cpsr));
 return(cpsr);
}




static inline void
TOPPERS_set_cpsr(uint32_t cpsr)
{
 __asm__ volatile("msr cpsr_cxsf, %0" : : "r"(cpsr) : "memory","cc");
}
/*  127 "../../../arch/arm_gcc/common/core_sil.h" */
static inline uint32_t
TOPPERS_disint(void)
{
 uint32_t cpsr;
 uint32_t fiq_irq_mask;

 cpsr = TOPPERS_current_cpsr();
 fiq_irq_mask = cpsr & (0x40U|0x80U);
 __asm__ volatile("cpsid fi" ::: "memory");
 return(fiq_irq_mask);
}




static inline void
TOPPERS_set_fiq_irq(uint32_t fiq_irq_mask)
{
 uint32_t cpsr;

 cpsr = TOPPERS_current_cpsr();
 cpsr &= ~(0x40U|0x80U);
 cpsr |= fiq_irq_mask;
 TOPPERS_set_cpsr(cpsr);
}
/*  163 "../../../arch/arm_gcc/common/core_sil.h" */
static inline void
sil_get_pid(ID *p_prcid)
{
 uint32_t reg;

 __asm__ volatile("mrc p15, 0, %0, c0, c0, 5":"=r"(reg));
 *p_prcid = (uint_t)(reg & 0x0ffU) + 1;
}




extern uint32_t TOPPERS_sil_spn_var;




static inline bool_t
TOPPERS_test_and_assign(uint32_t *p_var, uint32_t prcid)
{
 bool_t failed;

 __asm__ volatile("ldrex	%0, [%1]			\n"
 "	cmp		%0, #0				\n"
 "	strexeq	%0, %2, [%1]		\n"
 : "=&r"(failed) : "r"(p_var),"r"(prcid) : "cc");
 return(failed);
}




static inline uint32_t
TOPPERS_sil_loc_spn(void)
{
 uint32_t cpsr;
 uint32_t fiq_irq_mask;
 ID prcid;


 cpsr = TOPPERS_current_cpsr();
 fiq_irq_mask = cpsr & (0x40U|0x80U);
 __asm__ volatile("cpsid fi" ::: "memory");


 sil_get_pid(&prcid);
 if (TOPPERS_sil_spn_var == prcid) {

  fiq_irq_mask |= 0x01U;
 }
 else {

  while (TOPPERS_test_and_assign(&TOPPERS_sil_spn_var, prcid)) {
   TOPPERS_set_cpsr(cpsr);

   __asm__ volatile("dsb":::"memory");
   __asm__ volatile("wfe");

   __asm__ volatile("cpsid fi" ::: "memory");
  }

  __asm__ volatile("dmb":::"memory");
  __asm__ volatile("":::"memory");
 }
 return(fiq_irq_mask);
}




static inline void
TOPPERS_sil_unl_spn(uint32_t fiq_irq_mask)
{
 if ((fiq_irq_mask & 0x01U) != 0U) {

  fiq_irq_mask &= ~(0x01U);
 }
 else {
  __asm__ volatile("":::"memory");
  __asm__ volatile("dmb":::"memory");
  TOPPERS_sil_spn_var = 0U;

  __asm__ volatile("dsb":::"memory");
  __asm__ volatile("sev");

 }
 TOPPERS_set_fiq_irq(fiq_irq_mask);
}
/*  263 "../../../arch/arm_gcc/common/core_sil.h" */
static inline void
TOPPERS_sil_force_unl_spn(void)
{
 ID prcid;

 sil_get_pid(&prcid);
 if (TOPPERS_sil_spn_var == prcid) {
  TOPPERS_sil_spn_var = 0U;

  __asm__ volatile("dsb":::"memory");
  __asm__ volatile("sev");

 }
}
/*  23 "../../../target/zybo_z7_gcc/target_sil.h" 2 */
/*  69 "../../../include/sil.h" 2 */
/*  82 "../../../include/sil.h" */
extern void sil_dly_nse(ulong_t dlytim) ;
/*  108 "../../../include/sil.h" */
static inline uint8_t
sil_reb_mem(const uint8_t *mem)
{
 uint8_t data;

 data = *((const volatile uint8_t *) mem);
 return(data);
}

static inline void
sil_wrb_mem(uint8_t *mem, uint8_t data)
{
 *((volatile uint8_t *) mem) = data;
}







static inline uint16_t
sil_reh_mem(const uint16_t *mem)
{
 uint16_t data;

 data = *((const volatile uint16_t *) mem);
 return(data);
}

static inline void
sil_wrh_mem(uint16_t *mem, uint16_t data)
{
 *((volatile uint16_t *) mem) = data;
}
/*  178 "../../../include/sil.h" */
static inline uint16_t
sil_reh_bem(const uint16_t *mem)
{
 uint16_t data;

 data = *((const volatile uint16_t *) mem);
 return(((((data) & 0xffU) << 8) | (((data) >> 8) & 0xffU)));
}




static inline void
sil_wrh_bem(uint16_t *mem, uint16_t data)
{
 *((volatile uint16_t *) mem) = ((((data) & 0xffU) << 8) | (((data) >> 8) & 0xffU));
}
/*  208 "../../../include/sil.h" */
static inline uint32_t
sil_rew_mem(const uint32_t *mem)
{
 uint32_t data;

 data = *((const volatile uint32_t *) mem);
 return(data);
}

static inline void
sil_wrw_mem(uint32_t *mem, uint32_t data)
{
 *((volatile uint32_t *) mem) = data;
}
/*  256 "../../../include/sil.h" */
static inline uint32_t
sil_rew_bem(const uint32_t *mem)
{
 uint32_t data;

 data = *((const volatile uint32_t *) mem);
 return(((((data) & 0xffU) << 24) | (((data) & 0xff00U) << 8) | (((data) >> 8) & 0xff00U) | (((data) >> 24) & 0xffU)));
}




static inline void
sil_wrw_bem(uint32_t *mem, uint32_t data)
{
 *((volatile uint32_t *) mem) = ((((data) & 0xffU) << 24) | (((data) & 0xff00U) << 8) | (((data) >> 8) & 0xff00U) | (((data) >> 24) & 0xffU));
}
/*  285 "../../../include/sil.h" */
static inline void
sil_swrb_mem(uint8_t *mem, uint8_t data)
{
 sil_wrb_mem(mem, data);
 __asm__ volatile("dsb":::"memory");
}







static inline void
sil_swrh_mem(uint16_t *mem, uint16_t data)
{
 sil_wrh_mem(mem, data);
 __asm__ volatile("dsb":::"memory");
}

static inline void
sil_swrh_lem(uint16_t *mem, uint16_t data)
{
 sil_wrh_mem(mem, data);
 __asm__ volatile("dsb":::"memory");
}

static inline void
sil_swrh_bem(uint16_t *mem, uint16_t data)
{
 sil_wrh_bem(mem, data);
 __asm__ volatile("dsb":::"memory");
}





static inline void
sil_swrw_mem(uint32_t *mem, uint32_t data)
{
 sil_wrw_mem(mem, data);
 __asm__ volatile("dsb":::"memory");
}

static inline void
sil_swrw_lem(uint32_t *mem, uint32_t data)
{
 sil_wrw_mem(mem, data);
 __asm__ volatile("dsb":::"memory");
}

static inline void
sil_swrw_bem(uint32_t *mem, uint32_t data)
{
 sil_wrw_bem(mem, data);
 __asm__ volatile("dsb":::"memory");
}
/*  109 "../../../arch/arm_gcc/zynq7000/tXUartPs.h" 2 */
/*  40 "./gen/tmp_C_src.c" 2 */
