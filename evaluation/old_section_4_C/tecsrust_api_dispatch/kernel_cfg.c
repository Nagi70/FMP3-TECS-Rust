/* kernel_cfg.c */
#include "kernel/kernel_int.h"
#include "kernel_cfg.h"

#if !(TKERNEL_PRID == 0x0008U && (TKERNEL_PRVER & 0xf000U) == 0x3000U)
#error The kernel does not match this configuration file.
#endif

/*
 *  Include Directives
 */

#include "target_timer.h"
#if TNUM_PRCID >= 2
#endif
#if TNUM_PRCID >= 3
#endif
#if TNUM_PRCID >= 4
#endif
#ifdef USE_MPCORE_WDG_OVRTIMER
#ifdef TOPPERS_SUPPORT_OVRHDR
#if TNUM_PRCID >= 2
#endif
#if TNUM_PRCID >= 3
#endif
#if TNUM_PRCID >= 4
#endif
#endif
#endif
#include "target_ipi.h"
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER
#endif
#if TNUM_PRCID >= 2
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER
#endif
#endif
#if TNUM_PRCID >= 3
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER
#endif
#endif
#if TNUM_PRCID >= 4
#ifndef USE_BYPASS_IPI_DISPATCH_HANDER
#endif
#endif
#if TNUM_PRCID >= 2
#endif
#if TNUM_PRCID >= 3
#endif
#if TNUM_PRCID >= 4
#endif
#if TNUM_PRCID >= 2
#endif
#if TNUM_PRCID >= 3
#endif
#if TNUM_PRCID >= 4
#endif
#include "tTask_tecsgen.h"
#include "tISR_tecsgen.h"
#include "tInitializeRoutine_tecsgen.h"
#include "tTerminateRoutine_tecsgen.h"
#include "rust_tecs.h"

/*
 *  Processor Management Functions
 */

bool_t	_kernel_kerflg_table[TNUM_PRCID];

static PCB _kernel_pcb_prc1 __attribute__((section(".kernel_data_CLS_ALL_PRC1"),nocommon));
static PCB _kernel_pcb_prc2 __attribute__((section(".kernel_data_CLS_ALL_PRC2"),nocommon));

PCB	*const _kernel_p_pcb_table[TNUM_PRCID] = {
	&_kernel_pcb_prc1,
	&_kernel_pcb_prc2
};

/*
 *  Task Management Functions
 */

const ID _kernel_tmax_tskid = (TMIN_TSKID + TNUM_TSKID - 1);

static STK_T _kernel_stack_TSKID_tTask_rProcessor1Migratable_LogTask_Task[COUNT_STK_T(4096)] __attribute__((section(".stack_CLS_ALL_PRC1"),nocommon));
static STK_T _kernel_stack_TSKID_1_1[COUNT_STK_T(2048)] __attribute__((section(".stack_CLS_PRC1"),nocommon));
static STK_T _kernel_stack_TSKID_MIG[COUNT_STK_T(2048)] __attribute__((section(".stack_CLS_ALL_PRC1"),nocommon));
static STK_T _kernel_stack_TSKID_2_1[COUNT_STK_T(2048)] __attribute__((section(".stack_CLS_PRC2"),nocommon));
static STK_T _kernel_stack_TSKID_2_2[COUNT_STK_T(2048)] __attribute__((section(".stack_CLS_PRC2"),nocommon));
const TINIB _kernel_tinib_table[TNUM_TSKID] = {
	{ (TA_ACT), (EXINF)((intptr_t)&tTask_INIB_tab[0]), (TASK)(tTask_start), INT_PRIORITY(3), ROUND_STK_T(4096), _kernel_stack_TSKID_tTask_rProcessor1Migratable_LogTask_Task, 1, 0x3 },
	{ (TA_ACT), (EXINF)(0), (TASK)(tecs_rust_start_r_processor1_symmetric__task1_1), INT_PRIORITY(4), ROUND_STK_T(2048), _kernel_stack_TSKID_1_1, 1, 0x1 },
	{ (TA_NULL), (EXINF)(0), (TASK)(tecs_rust_start_r_processor_all_mig__taskmig), INT_PRIORITY(6), ROUND_STK_T(2048), _kernel_stack_TSKID_MIG, 1, 0x3 },
	{ (TA_ACT), (EXINF)(0), (TASK)(tecs_rust_start_r_processor2_symmetric__task2_1), INT_PRIORITY(7), ROUND_STK_T(2048), _kernel_stack_TSKID_2_1, 2, 0x2 },
	{ (TA_ACT), (EXINF)(0), (TASK)(tecs_rust_start_r_processor2_symmetric__task2_2), INT_PRIORITY(10), ROUND_STK_T(2048), _kernel_stack_TSKID_2_2, 2, 0x2 }
};

static TCB _kernel_tcb_TSKID_tTask_rProcessor1Migratable_LogTask_Task __attribute__((section(".kernel_data_CLS_ALL_PRC1"),nocommon));
static TCB _kernel_tcb_TSKID_1_1 __attribute__((section(".kernel_data_CLS_PRC1"),nocommon));
static TCB _kernel_tcb_TSKID_MIG __attribute__((section(".kernel_data_CLS_ALL_PRC1"),nocommon));
static TCB _kernel_tcb_TSKID_2_1 __attribute__((section(".kernel_data_CLS_PRC2"),nocommon));
static TCB _kernel_tcb_TSKID_2_2 __attribute__((section(".kernel_data_CLS_PRC2"),nocommon));

TCB	*const _kernel_p_tcb_table[TNUM_TSKID] = {
	&_kernel_tcb_TSKID_tTask_rProcessor1Migratable_LogTask_Task,
	&_kernel_tcb_TSKID_1_1,
	&_kernel_tcb_TSKID_MIG,
	&_kernel_tcb_TSKID_2_1,
	&_kernel_tcb_TSKID_2_2
};

const ID _kernel_torder_table[TNUM_TSKID] = { 
	TSKID_tTask_rProcessor1Migratable_LogTask_Task, TSKID_1_1, TSKID_MIG, TSKID_2_1, TSKID_2_2
};

const uint16_t _kernel_subprio_primap = 0U;

/*
 *  Semaphore Functions
 */

const ID _kernel_tmax_semid = (TMIN_SEMID + TNUM_SEMID - 1);

const SEMINIB _kernel_seminib_table[TNUM_SEMID] = {
	{ (TA_TPRI), (0), (1) },
	{ (TA_TPRI), (1), (1) },
	{ (TA_NULL), (1), (1) }
};

static SEMCB _kernel_semcb_SEMID_tSemaphore_rProcessor1Migratable_SerialPort1_ReceiveSemaphore __attribute__((section(".kernel_data_CLS_ALL_PRC1"),nocommon));
static SEMCB _kernel_semcb_SEMID_tSemaphore_rProcessor1Migratable_SerialPort1_SendSemaphore __attribute__((section(".kernel_data_CLS_ALL_PRC1"),nocommon));
static SEMCB _kernel_semcb_SEMID_1 __attribute__((section(".kernel_data_CLS_ALL_PRC1"),nocommon));

SEMCB	*const _kernel_p_semcb_table[TNUM_SEMID] = {
	&_kernel_semcb_SEMID_tSemaphore_rProcessor1Migratable_SerialPort1_ReceiveSemaphore,
	&_kernel_semcb_SEMID_tSemaphore_rProcessor1Migratable_SerialPort1_SendSemaphore,
	&_kernel_semcb_SEMID_1
};

/*
 *  Eventflag Functions
 */

const ID _kernel_tmax_flgid = (TMIN_FLGID + TNUM_FLGID - 1);

TOPPERS_EMPTY_LABEL(const FLGINIB, _kernel_flginib_table);
TOPPERS_EMPTY_LABEL(FLGCB *const, _kernel_p_flgcb_table);

/*
 *  Dataqueue Functions
 */

const ID _kernel_tmax_dtqid = (TMIN_DTQID + TNUM_DTQID - 1);

TOPPERS_EMPTY_LABEL(const DTQINIB, _kernel_dtqinib_table);
TOPPERS_EMPTY_LABEL(DTQCB *const, _kernel_p_dtqcb_table);

/*
 *  Priority Dataqueue Functions
 */

const ID _kernel_tmax_pdqid = (TMIN_PDQID + TNUM_PDQID - 1);

TOPPERS_EMPTY_LABEL(const PDQINIB, _kernel_pdqinib_table);
TOPPERS_EMPTY_LABEL(PDQCB *const, _kernel_p_pdqcb_table);

/*
 *  Mutex Functions
 */

const ID _kernel_tmax_mtxid = (TMIN_MTXID + TNUM_MTXID - 1);

TOPPERS_EMPTY_LABEL(const MTXINIB, _kernel_mtxinib_table);
TOPPERS_EMPTY_LABEL(MTXCB *const, _kernel_p_mtxcb_table);

/*
 *  SpinLock Functions
 */

const ID _kernel_tmax_spnid = (TMIN_SPNID + TNUM_SPNID - 1);

TOPPERS_EMPTY_LABEL(const SPNINIB, _kernel_spninib_table);
/*
 *  Fixed-sized Memorypool Functions
 */

const ID _kernel_tmax_mpfid = (TMIN_MPFID + TNUM_MPFID - 1);

TOPPERS_EMPTY_LABEL(const MPFINIB, _kernel_mpfinib_table);
TOPPERS_EMPTY_LABEL(MPFCB *const, _kernel_p_mpfcb_table);

/*
 *  Cyclic Notification Functions
 */

const ID _kernel_tmax_cycid = (TMIN_CYCID + TNUM_CYCID - 1);

TOPPERS_EMPTY_LABEL(const CYCINIB, _kernel_cycinib_table);
TOPPERS_EMPTY_LABEL(CYCCB *const, _kernel_p_cyccb_table);

/*
 *  Alarm Notification Functions
 */

const ID _kernel_tmax_almid = (TMIN_ALMID + TNUM_ALMID - 1);

TOPPERS_EMPTY_LABEL(const ALMINIB, _kernel_alminib_table);
TOPPERS_EMPTY_LABEL(ALMCB *const, _kernel_p_almcb_table);

/*
 *  Interrupt Management Functions
 */

#ifndef LOG_ISR_ENTER
#define LOG_ISR_ENTER(isrid)
#endif /* LOG_ISR_ENTER */

#ifndef LOG_ISR_LEAVE
#define LOG_ISR_LEAVE(isrid)
#endif /* LOG_ISR_LEAVE */

void
_kernel_inthdr_82(void)
{
	LOG_ISR_ENTER(ISRID_tISR_rProcessor1Migratable_SIOPortTarget1_ISRInstance);
	((ISR)(tISR_start))((EXINF)(&tISR_INIB_tab[0]));
	LOG_ISR_LEAVE(ISRID_tISR_rProcessor1Migratable_SIOPortTarget1_ISRInstance);
}

#define TNUM_CFG_INTNO	9
const uint_t _kernel_tnum_cfg_intno = TNUM_CFG_INTNO;

const INTINIB _kernel_intinib_table[TNUM_CFG_INTNO] = {
	{ (INTNO_TIMER_PRC1), (TA_ENAINT|INTATR_TIMER), (INTPRI_TIMER), 1, 0x1U },
	{ (INTNO_TIMER_PRC2), (TA_ENAINT|INTATR_TIMER), (INTPRI_TIMER), 2, 0x2U },
	{ (INTNO_IPI_DISPATCH_PRC1), (TA_ENAINT), (INTPRI_IPI_DISPATCH_PRC1), 1, 0x1U },
	{ (INTNO_IPI_DISPATCH_PRC2), (TA_ENAINT), (INTPRI_IPI_DISPATCH_PRC2), 2, 0x2U },
	{ (INTNO_IPI_EXT_KER_PRC1), (TA_ENAINT), (INTPRI_IPI_EXT_KER_PRC1), 1, 0x1U },
	{ (INTNO_IPI_EXT_KER_PRC2), (TA_ENAINT), (INTPRI_IPI_EXT_KER_PRC2), 2, 0x2U },
	{ (INTNO_IPI_SET_HRT_EVT_PRC1), (TA_ENAINT), (INTPRI_IPI_SET_HRT_EVT_PRC1), 1, 0x1U },
	{ (INTNO_IPI_SET_HRT_EVT_PRC2), (TA_ENAINT), (INTPRI_IPI_SET_HRT_EVT_PRC2), 2, 0x2U },
	{ (INTNO_SIO), (TA_NULL), (INTPRI_SIO), 1, 0x1U }
};

/*
 *  CPU Exception Management Functions
 */

/*
 *  Stack Area for Non-task Context
 */

static STK_T _kernel_istack_prc1[COUNT_STK_T(DEFAULT_ISTKSZ)] __attribute__((section(".stack_CLS_ALL_PRC1"),nocommon));
static STK_T _kernel_istack_prc2[COUNT_STK_T(DEFAULT_ISTKSZ)] __attribute__((section(".stack_CLS_ALL_PRC2"),nocommon));

const size_t _kernel_istksz_table[TNUM_PRCID] = {
	ROUND_STK_T(DEFAULT_ISTKSZ),
	ROUND_STK_T(DEFAULT_ISTKSZ)
};

STK_T *const _kernel_istk_table[TNUM_PRCID] = {
	_kernel_istack_prc1,
	_kernel_istack_prc2
};

#ifdef TOPPERS_ISTKPT
STK_T *const _kernel_istkpt_table[TNUM_PRCID] = {
	TOPPERS_ISTKPT(_kernel_istack_prc1, ROUND_STK_T(DEFAULT_ISTKSZ)),
	TOPPERS_ISTKPT(_kernel_istack_prc2, ROUND_STK_T(DEFAULT_ISTKSZ))
};
#endif /* TOPPERS_ISTKPT */

/*
 *  Stack Area for Idle
 */

static STK_T _kernel_idstack_prc1[COUNT_STK_T(DEFAULT_IDSTKSZ)] __attribute__((section(".stack_CLS_ALL_PRC1"),nocommon));
static STK_T _kernel_idstack_prc2[COUNT_STK_T(DEFAULT_IDSTKSZ)] __attribute__((section(".stack_CLS_ALL_PRC2"),nocommon));

#ifndef TOPPERS_ISTKPT
STK_T *const _kernel_idstk_table[TNUM_PRCID] = {
	_kernel_idstack_prc1,
	_kernel_idstack_prc2
};
#endif /* TOPPERS_ISTKPT */

#ifdef TOPPERS_ISTKPT
STK_T *const _kernel_idstkpt_table[TNUM_PRCID] = {
	TOPPERS_ISTKPT(_kernel_idstack_prc1, ROUND_STK_T(DEFAULT_IDSTKSZ)),
	TOPPERS_ISTKPT(_kernel_idstack_prc2, ROUND_STK_T(DEFAULT_IDSTKSZ))
};
#endif /* TOPPERS_ISTKPT */

/*
 *  Time Event Management
 */

static TMEVTN	_kernel_tmevt_heap_prc1[1 + TNUM_TSKID + TNUM_CYCID + TNUM_ALMID] __attribute__((section(".kernel_data_CLS_ALL_PRC1"),nocommon));
static TMEVTN	_kernel_tmevt_heap_prc2[1 + TNUM_TSKID + TNUM_CYCID + TNUM_ALMID] __attribute__((section(".kernel_data_CLS_ALL_PRC2"),nocommon));

TMEVTN	*const _kernel_p_tmevt_heap_table[TNUM_PRCID] = {
	_kernel_tmevt_heap_prc1,
	_kernel_tmevt_heap_prc2
};

TEVTCB _kernel_tevtcb_prc1;
TEVTCB _kernel_tevtcb_prc2;

TEVTCB	*const _kernel_p_tevtcb_table[TNUM_PRCID] = {
	&_kernel_tevtcb_prc1,
	&_kernel_tevtcb_prc2
};

/*
 *  Module Initialization Function
 */

void
_kernel_initialize_object(PCB *p_my_pcb)
{
	_kernel_initialize_task(p_my_pcb);
	_kernel_initialize_semaphore(p_my_pcb);
	_kernel_initialize_interrupt(p_my_pcb);
	_kernel_initialize_exception(p_my_pcb);
}

/*
 *  Initialization Routine
 */

const INIRTNB _kernel_inirtnb_table_prc1[2] = {
	{ (INIRTN)(_kernel_target_hrt_initialize), (EXINF)(0) },
	{ (INIRTN)(tInitializeRoutine_start), (EXINF)(((tInitializeRoutine_IDX)0)) }
};

const INIRTNB _kernel_inirtnb_table_prc2[1] = {
	{ (INIRTN)(_kernel_target_hrt_initialize), (EXINF)(0) }
};

const INIRTNBB _kernel_inirtnbb_table[TNUM_PRCID + 1] = {
	{ 0, NULL },
	{ 2, _kernel_inirtnb_table_prc1 },
	{ 1, _kernel_inirtnb_table_prc2 }
};

/*
 *  Termination Routine
 */

const TERRTNB _kernel_terrtnb_table_prc1[3] = {
	{ (TERRTN)(tTerminateRoutine_start), (EXINF)(&tTerminateRoutine_INIB_tab[1]) },
	{ (TERRTN)(tTerminateRoutine_start), (EXINF)(&tTerminateRoutine_INIB_tab[0]) },
	{ (TERRTN)(_kernel_target_hrt_terminate), (EXINF)(0) }
};

const TERRTNB _kernel_terrtnb_table_prc2[1] = {
	{ (TERRTN)(_kernel_target_hrt_terminate), (EXINF)(0) }
};

const TERRTNBB _kernel_terrtnbb_table[TNUM_PRCID + 1] = {
	{ 0, NULL },
	{ 3, _kernel_terrtnb_table_prc1 },
	{ 1, _kernel_terrtnb_table_prc2 }
};

/*
 *  Interrupt Handler Table
 */

const FP _kernel_inh_table_prc1[96] = {
	/* 0x10000 */ (FP)(_kernel_default_int_handler),
	/* 0x10001 */ (FP)(_kernel_ext_ker_handler),
	/* 0x10002 */ (FP)(_kernel_set_hrt_event_handler),
	/* 0x10003 */ (FP)(_kernel_default_int_handler),
	/* 0x10004 */ (FP)(_kernel_default_int_handler),
	/* 0x10005 */ (FP)(_kernel_default_int_handler),
	/* 0x10006 */ (FP)(_kernel_default_int_handler),
	/* 0x10007 */ (FP)(_kernel_default_int_handler),
	/* 0x10008 */ (FP)(_kernel_default_int_handler),
	/* 0x10009 */ (FP)(_kernel_default_int_handler),
	/* 0x1000a */ (FP)(_kernel_default_int_handler),
	/* 0x1000b */ (FP)(_kernel_default_int_handler),
	/* 0x1000c */ (FP)(_kernel_default_int_handler),
	/* 0x1000d */ (FP)(_kernel_default_int_handler),
	/* 0x1000e */ (FP)(_kernel_default_int_handler),
	/* 0x1000f */ (FP)(_kernel_default_int_handler),
	/* 0x10010 */ (FP)(_kernel_default_int_handler),
	/* 0x10011 */ (FP)(_kernel_default_int_handler),
	/* 0x10012 */ (FP)(_kernel_default_int_handler),
	/* 0x10013 */ (FP)(_kernel_default_int_handler),
	/* 0x10014 */ (FP)(_kernel_default_int_handler),
	/* 0x10015 */ (FP)(_kernel_default_int_handler),
	/* 0x10016 */ (FP)(_kernel_default_int_handler),
	/* 0x10017 */ (FP)(_kernel_default_int_handler),
	/* 0x10018 */ (FP)(_kernel_default_int_handler),
	/* 0x10019 */ (FP)(_kernel_default_int_handler),
	/* 0x1001a */ (FP)(_kernel_default_int_handler),
	/* 0x1001b */ (FP)(_kernel_target_hrt_handler),
	/* 0x1001c */ (FP)(_kernel_default_int_handler),
	/* 0x1001d */ (FP)(_kernel_default_int_handler),
	/* 0x1001e */ (FP)(_kernel_default_int_handler),
	/* 0x1001f */ (FP)(_kernel_default_int_handler),
	/* 0x10020 */ (FP)(_kernel_default_int_handler),
	/* 0x10021 */ (FP)(_kernel_default_int_handler),
	/* 0x10022 */ (FP)(_kernel_default_int_handler),
	/* 0x10023 */ (FP)(_kernel_default_int_handler),
	/* 0x10024 */ (FP)(_kernel_default_int_handler),
	/* 0x10025 */ (FP)(_kernel_default_int_handler),
	/* 0x10026 */ (FP)(_kernel_default_int_handler),
	/* 0x10027 */ (FP)(_kernel_default_int_handler),
	/* 0x10028 */ (FP)(_kernel_default_int_handler),
	/* 0x10029 */ (FP)(_kernel_default_int_handler),
	/* 0x1002a */ (FP)(_kernel_default_int_handler),
	/* 0x1002b */ (FP)(_kernel_default_int_handler),
	/* 0x1002c */ (FP)(_kernel_default_int_handler),
	/* 0x1002d */ (FP)(_kernel_default_int_handler),
	/* 0x1002e */ (FP)(_kernel_default_int_handler),
	/* 0x1002f */ (FP)(_kernel_default_int_handler),
	/* 0x10030 */ (FP)(_kernel_default_int_handler),
	/* 0x10031 */ (FP)(_kernel_default_int_handler),
	/* 0x10032 */ (FP)(_kernel_default_int_handler),
	/* 0x10033 */ (FP)(_kernel_default_int_handler),
	/* 0x10034 */ (FP)(_kernel_default_int_handler),
	/* 0x10035 */ (FP)(_kernel_default_int_handler),
	/* 0x10036 */ (FP)(_kernel_default_int_handler),
	/* 0x10037 */ (FP)(_kernel_default_int_handler),
	/* 0x10038 */ (FP)(_kernel_default_int_handler),
	/* 0x10039 */ (FP)(_kernel_default_int_handler),
	/* 0x1003a */ (FP)(_kernel_default_int_handler),
	/* 0x1003b */ (FP)(_kernel_default_int_handler),
	/* 0x1003c */ (FP)(_kernel_default_int_handler),
	/* 0x1003d */ (FP)(_kernel_default_int_handler),
	/* 0x1003e */ (FP)(_kernel_default_int_handler),
	/* 0x1003f */ (FP)(_kernel_default_int_handler),
	/* 0x10040 */ (FP)(_kernel_default_int_handler),
	/* 0x10041 */ (FP)(_kernel_default_int_handler),
	/* 0x10042 */ (FP)(_kernel_default_int_handler),
	/* 0x10043 */ (FP)(_kernel_default_int_handler),
	/* 0x10044 */ (FP)(_kernel_default_int_handler),
	/* 0x10045 */ (FP)(_kernel_default_int_handler),
	/* 0x10046 */ (FP)(_kernel_default_int_handler),
	/* 0x10047 */ (FP)(_kernel_default_int_handler),
	/* 0x10048 */ (FP)(_kernel_default_int_handler),
	/* 0x10049 */ (FP)(_kernel_default_int_handler),
	/* 0x1004a */ (FP)(_kernel_default_int_handler),
	/* 0x1004b */ (FP)(_kernel_default_int_handler),
	/* 0x1004c */ (FP)(_kernel_default_int_handler),
	/* 0x1004d */ (FP)(_kernel_default_int_handler),
	/* 0x1004e */ (FP)(_kernel_default_int_handler),
	/* 0x1004f */ (FP)(_kernel_default_int_handler),
	/* 0x10050 */ (FP)(_kernel_default_int_handler),
	/* 0x10051 */ (FP)(_kernel_default_int_handler),
	/* 0x10052 */ (FP)(_kernel_inthdr_82),
	/* 0x10053 */ (FP)(_kernel_default_int_handler),
	/* 0x10054 */ (FP)(_kernel_default_int_handler),
	/* 0x10055 */ (FP)(_kernel_default_int_handler),
	/* 0x10056 */ (FP)(_kernel_default_int_handler),
	/* 0x10057 */ (FP)(_kernel_default_int_handler),
	/* 0x10058 */ (FP)(_kernel_default_int_handler),
	/* 0x10059 */ (FP)(_kernel_default_int_handler),
	/* 0x1005a */ (FP)(_kernel_default_int_handler),
	/* 0x1005b */ (FP)(_kernel_default_int_handler),
	/* 0x1005c */ (FP)(_kernel_default_int_handler),
	/* 0x1005d */ (FP)(_kernel_default_int_handler),
	/* 0x1005e */ (FP)(_kernel_default_int_handler),
	/* 0x1005f */ (FP)(_kernel_default_int_handler)
};

const FP _kernel_inh_table_prc2[96] = {
	/* 0x20000 */ (FP)(_kernel_default_int_handler),
	/* 0x20001 */ (FP)(_kernel_ext_ker_handler),
	/* 0x20002 */ (FP)(_kernel_set_hrt_event_handler),
	/* 0x20003 */ (FP)(_kernel_default_int_handler),
	/* 0x20004 */ (FP)(_kernel_default_int_handler),
	/* 0x20005 */ (FP)(_kernel_default_int_handler),
	/* 0x20006 */ (FP)(_kernel_default_int_handler),
	/* 0x20007 */ (FP)(_kernel_default_int_handler),
	/* 0x20008 */ (FP)(_kernel_default_int_handler),
	/* 0x20009 */ (FP)(_kernel_default_int_handler),
	/* 0x2000a */ (FP)(_kernel_default_int_handler),
	/* 0x2000b */ (FP)(_kernel_default_int_handler),
	/* 0x2000c */ (FP)(_kernel_default_int_handler),
	/* 0x2000d */ (FP)(_kernel_default_int_handler),
	/* 0x2000e */ (FP)(_kernel_default_int_handler),
	/* 0x2000f */ (FP)(_kernel_default_int_handler),
	/* 0x20010 */ (FP)(_kernel_default_int_handler),
	/* 0x20011 */ (FP)(_kernel_default_int_handler),
	/* 0x20012 */ (FP)(_kernel_default_int_handler),
	/* 0x20013 */ (FP)(_kernel_default_int_handler),
	/* 0x20014 */ (FP)(_kernel_default_int_handler),
	/* 0x20015 */ (FP)(_kernel_default_int_handler),
	/* 0x20016 */ (FP)(_kernel_default_int_handler),
	/* 0x20017 */ (FP)(_kernel_default_int_handler),
	/* 0x20018 */ (FP)(_kernel_default_int_handler),
	/* 0x20019 */ (FP)(_kernel_default_int_handler),
	/* 0x2001a */ (FP)(_kernel_default_int_handler),
	/* 0x2001b */ (FP)(_kernel_target_hrt_handler),
	/* 0x2001c */ (FP)(_kernel_default_int_handler),
	/* 0x2001d */ (FP)(_kernel_default_int_handler),
	/* 0x2001e */ (FP)(_kernel_default_int_handler),
	/* 0x2001f */ (FP)(_kernel_default_int_handler),
	/* 0x20020 */ (FP)(_kernel_default_int_handler),
	/* 0x20021 */ (FP)(_kernel_default_int_handler),
	/* 0x20022 */ (FP)(_kernel_default_int_handler),
	/* 0x20023 */ (FP)(_kernel_default_int_handler),
	/* 0x20024 */ (FP)(_kernel_default_int_handler),
	/* 0x20025 */ (FP)(_kernel_default_int_handler),
	/* 0x20026 */ (FP)(_kernel_default_int_handler),
	/* 0x20027 */ (FP)(_kernel_default_int_handler),
	/* 0x20028 */ (FP)(_kernel_default_int_handler),
	/* 0x20029 */ (FP)(_kernel_default_int_handler),
	/* 0x2002a */ (FP)(_kernel_default_int_handler),
	/* 0x2002b */ (FP)(_kernel_default_int_handler),
	/* 0x2002c */ (FP)(_kernel_default_int_handler),
	/* 0x2002d */ (FP)(_kernel_default_int_handler),
	/* 0x2002e */ (FP)(_kernel_default_int_handler),
	/* 0x2002f */ (FP)(_kernel_default_int_handler),
	/* 0x20030 */ (FP)(_kernel_default_int_handler),
	/* 0x20031 */ (FP)(_kernel_default_int_handler),
	/* 0x20032 */ (FP)(_kernel_default_int_handler),
	/* 0x20033 */ (FP)(_kernel_default_int_handler),
	/* 0x20034 */ (FP)(_kernel_default_int_handler),
	/* 0x20035 */ (FP)(_kernel_default_int_handler),
	/* 0x20036 */ (FP)(_kernel_default_int_handler),
	/* 0x20037 */ (FP)(_kernel_default_int_handler),
	/* 0x20038 */ (FP)(_kernel_default_int_handler),
	/* 0x20039 */ (FP)(_kernel_default_int_handler),
	/* 0x2003a */ (FP)(_kernel_default_int_handler),
	/* 0x2003b */ (FP)(_kernel_default_int_handler),
	/* 0x2003c */ (FP)(_kernel_default_int_handler),
	/* 0x2003d */ (FP)(_kernel_default_int_handler),
	/* 0x2003e */ (FP)(_kernel_default_int_handler),
	/* 0x2003f */ (FP)(_kernel_default_int_handler),
	/* 0x20040 */ (FP)(_kernel_default_int_handler),
	/* 0x20041 */ (FP)(_kernel_default_int_handler),
	/* 0x20042 */ (FP)(_kernel_default_int_handler),
	/* 0x20043 */ (FP)(_kernel_default_int_handler),
	/* 0x20044 */ (FP)(_kernel_default_int_handler),
	/* 0x20045 */ (FP)(_kernel_default_int_handler),
	/* 0x20046 */ (FP)(_kernel_default_int_handler),
	/* 0x20047 */ (FP)(_kernel_default_int_handler),
	/* 0x20048 */ (FP)(_kernel_default_int_handler),
	/* 0x20049 */ (FP)(_kernel_default_int_handler),
	/* 0x2004a */ (FP)(_kernel_default_int_handler),
	/* 0x2004b */ (FP)(_kernel_default_int_handler),
	/* 0x2004c */ (FP)(_kernel_default_int_handler),
	/* 0x2004d */ (FP)(_kernel_default_int_handler),
	/* 0x2004e */ (FP)(_kernel_default_int_handler),
	/* 0x2004f */ (FP)(_kernel_default_int_handler),
	/* 0x20050 */ (FP)(_kernel_default_int_handler),
	/* 0x20051 */ (FP)(_kernel_default_int_handler),
	/* 0x20052 */ (FP)(_kernel_inthdr_82),
	/* 0x20053 */ (FP)(_kernel_default_int_handler),
	/* 0x20054 */ (FP)(_kernel_default_int_handler),
	/* 0x20055 */ (FP)(_kernel_default_int_handler),
	/* 0x20056 */ (FP)(_kernel_default_int_handler),
	/* 0x20057 */ (FP)(_kernel_default_int_handler),
	/* 0x20058 */ (FP)(_kernel_default_int_handler),
	/* 0x20059 */ (FP)(_kernel_default_int_handler),
	/* 0x2005a */ (FP)(_kernel_default_int_handler),
	/* 0x2005b */ (FP)(_kernel_default_int_handler),
	/* 0x2005c */ (FP)(_kernel_default_int_handler),
	/* 0x2005d */ (FP)(_kernel_default_int_handler),
	/* 0x2005e */ (FP)(_kernel_default_int_handler),
	/* 0x2005f */ (FP)(_kernel_default_int_handler)
};

const FP* const _kernel_p_inh_table[TNUM_PRCID] = {
	_kernel_inh_table_prc1,
	_kernel_inh_table_prc2
};

/*
 *  Interrupt Configuration Table
 */

const uint8_t _kernel_intcfg_table_prc1[96] = {
	/* 0x10000 */ 1U,
	/* 0x10001 */ 1U,
	/* 0x10002 */ 1U,
	/* 0x10003 */ 0U,
	/* 0x10004 */ 0U,
	/* 0x10005 */ 0U,
	/* 0x10006 */ 0U,
	/* 0x10007 */ 0U,
	/* 0x10008 */ 0U,
	/* 0x10009 */ 0U,
	/* 0x1000a */ 0U,
	/* 0x1000b */ 0U,
	/* 0x1000c */ 0U,
	/* 0x1000d */ 0U,
	/* 0x1000e */ 0U,
	/* 0x1000f */ 0U,
	/* 0x10010 */ 0U,
	/* 0x10011 */ 0U,
	/* 0x10012 */ 0U,
	/* 0x10013 */ 0U,
	/* 0x10014 */ 0U,
	/* 0x10015 */ 0U,
	/* 0x10016 */ 0U,
	/* 0x10017 */ 0U,
	/* 0x10018 */ 0U,
	/* 0x10019 */ 0U,
	/* 0x1001a */ 0U,
	/* 0x1001b */ 1U,
	/* 0x1001c */ 0U,
	/* 0x1001d */ 0U,
	/* 0x1001e */ 0U,
	/* 0x1001f */ 0U,
	/* 0x00020 */ 0U,
	/* 0x00021 */ 0U,
	/* 0x00022 */ 0U,
	/* 0x00023 */ 0U,
	/* 0x00024 */ 0U,
	/* 0x00025 */ 0U,
	/* 0x00026 */ 0U,
	/* 0x00027 */ 0U,
	/* 0x00028 */ 0U,
	/* 0x00029 */ 0U,
	/* 0x0002a */ 0U,
	/* 0x0002b */ 0U,
	/* 0x0002c */ 0U,
	/* 0x0002d */ 0U,
	/* 0x0002e */ 0U,
	/* 0x0002f */ 0U,
	/* 0x00030 */ 0U,
	/* 0x00031 */ 0U,
	/* 0x00032 */ 0U,
	/* 0x00033 */ 0U,
	/* 0x00034 */ 0U,
	/* 0x00035 */ 0U,
	/* 0x00036 */ 0U,
	/* 0x00037 */ 0U,
	/* 0x00038 */ 0U,
	/* 0x00039 */ 0U,
	/* 0x0003a */ 0U,
	/* 0x0003b */ 0U,
	/* 0x0003c */ 0U,
	/* 0x0003d */ 0U,
	/* 0x0003e */ 0U,
	/* 0x0003f */ 0U,
	/* 0x00040 */ 0U,
	/* 0x00041 */ 0U,
	/* 0x00042 */ 0U,
	/* 0x00043 */ 0U,
	/* 0x00044 */ 0U,
	/* 0x00045 */ 0U,
	/* 0x00046 */ 0U,
	/* 0x00047 */ 0U,
	/* 0x00048 */ 0U,
	/* 0x00049 */ 0U,
	/* 0x0004a */ 0U,
	/* 0x0004b */ 0U,
	/* 0x0004c */ 0U,
	/* 0x0004d */ 0U,
	/* 0x0004e */ 0U,
	/* 0x0004f */ 0U,
	/* 0x00050 */ 0U,
	/* 0x00051 */ 0U,
	/* 0x00052 */ 1U,
	/* 0x00053 */ 0U,
	/* 0x00054 */ 0U,
	/* 0x00055 */ 0U,
	/* 0x00056 */ 0U,
	/* 0x00057 */ 0U,
	/* 0x00058 */ 0U,
	/* 0x00059 */ 0U,
	/* 0x0005a */ 0U,
	/* 0x0005b */ 0U,
	/* 0x0005c */ 0U,
	/* 0x0005d */ 0U,
	/* 0x0005e */ 0U,
	/* 0x0005f */ 0U
};

const uint8_t _kernel_intcfg_table_prc2[96] = {
	/* 0x20000 */ 1U,
	/* 0x20001 */ 1U,
	/* 0x20002 */ 1U,
	/* 0x20003 */ 0U,
	/* 0x20004 */ 0U,
	/* 0x20005 */ 0U,
	/* 0x20006 */ 0U,
	/* 0x20007 */ 0U,
	/* 0x20008 */ 0U,
	/* 0x20009 */ 0U,
	/* 0x2000a */ 0U,
	/* 0x2000b */ 0U,
	/* 0x2000c */ 0U,
	/* 0x2000d */ 0U,
	/* 0x2000e */ 0U,
	/* 0x2000f */ 0U,
	/* 0x20010 */ 0U,
	/* 0x20011 */ 0U,
	/* 0x20012 */ 0U,
	/* 0x20013 */ 0U,
	/* 0x20014 */ 0U,
	/* 0x20015 */ 0U,
	/* 0x20016 */ 0U,
	/* 0x20017 */ 0U,
	/* 0x20018 */ 0U,
	/* 0x20019 */ 0U,
	/* 0x2001a */ 0U,
	/* 0x2001b */ 1U,
	/* 0x2001c */ 0U,
	/* 0x2001d */ 0U,
	/* 0x2001e */ 0U,
	/* 0x2001f */ 0U,
	/* 0x00020 */ 0U,
	/* 0x00021 */ 0U,
	/* 0x00022 */ 0U,
	/* 0x00023 */ 0U,
	/* 0x00024 */ 0U,
	/* 0x00025 */ 0U,
	/* 0x00026 */ 0U,
	/* 0x00027 */ 0U,
	/* 0x00028 */ 0U,
	/* 0x00029 */ 0U,
	/* 0x0002a */ 0U,
	/* 0x0002b */ 0U,
	/* 0x0002c */ 0U,
	/* 0x0002d */ 0U,
	/* 0x0002e */ 0U,
	/* 0x0002f */ 0U,
	/* 0x00030 */ 0U,
	/* 0x00031 */ 0U,
	/* 0x00032 */ 0U,
	/* 0x00033 */ 0U,
	/* 0x00034 */ 0U,
	/* 0x00035 */ 0U,
	/* 0x00036 */ 0U,
	/* 0x00037 */ 0U,
	/* 0x00038 */ 0U,
	/* 0x00039 */ 0U,
	/* 0x0003a */ 0U,
	/* 0x0003b */ 0U,
	/* 0x0003c */ 0U,
	/* 0x0003d */ 0U,
	/* 0x0003e */ 0U,
	/* 0x0003f */ 0U,
	/* 0x00040 */ 0U,
	/* 0x00041 */ 0U,
	/* 0x00042 */ 0U,
	/* 0x00043 */ 0U,
	/* 0x00044 */ 0U,
	/* 0x00045 */ 0U,
	/* 0x00046 */ 0U,
	/* 0x00047 */ 0U,
	/* 0x00048 */ 0U,
	/* 0x00049 */ 0U,
	/* 0x0004a */ 0U,
	/* 0x0004b */ 0U,
	/* 0x0004c */ 0U,
	/* 0x0004d */ 0U,
	/* 0x0004e */ 0U,
	/* 0x0004f */ 0U,
	/* 0x00050 */ 0U,
	/* 0x00051 */ 0U,
	/* 0x00052 */ 1U,
	/* 0x00053 */ 0U,
	/* 0x00054 */ 0U,
	/* 0x00055 */ 0U,
	/* 0x00056 */ 0U,
	/* 0x00057 */ 0U,
	/* 0x00058 */ 0U,
	/* 0x00059 */ 0U,
	/* 0x0005a */ 0U,
	/* 0x0005b */ 0U,
	/* 0x0005c */ 0U,
	/* 0x0005d */ 0U,
	/* 0x0005e */ 0U,
	/* 0x0005f */ 0U
};

const uint8_t* const _kernel_p_intcfg_table[TNUM_PRCID] = {
	_kernel_intcfg_table_prc1,
	_kernel_intcfg_table_prc2
};

/*
 *  CPU Exception Handler Table
 */

const FP _kernel_exc_table_prc1[7] = {
	/* 0x10000 */ (FP)(_kernel_default_exc_handler),
	/* 0x10001 */ (FP)(_kernel_default_exc_handler),
	/* 0x10002 */ (FP)(_kernel_default_exc_handler),
	/* 0x10003 */ (FP)(_kernel_default_exc_handler),
	/* 0x10004 */ (FP)(_kernel_default_exc_handler),
	/* 0x10005 */ (FP)(_kernel_default_exc_handler),
	/* 0x10006 */ (FP)(_kernel_default_exc_handler)
};

const FP _kernel_exc_table_prc2[7] = {
	/* 0x20000 */ (FP)(_kernel_default_exc_handler),
	/* 0x20001 */ (FP)(_kernel_default_exc_handler),
	/* 0x20002 */ (FP)(_kernel_default_exc_handler),
	/* 0x20003 */ (FP)(_kernel_default_exc_handler),
	/* 0x20004 */ (FP)(_kernel_default_exc_handler),
	/* 0x20005 */ (FP)(_kernel_default_exc_handler),
	/* 0x20006 */ (FP)(_kernel_default_exc_handler)
};

const FP* const _kernel_p_exc_table[TNUM_PRCID] = {
	_kernel_exc_table_prc1,
	_kernel_exc_table_prc2
};

