/* kernel_cfg.h */
#ifndef TOPPERS_KERNEL_CFG_H
#define TOPPERS_KERNEL_CFG_H

#define TNUM_TSKID	3
#define TSKID_tTask_rProcessor1Migratable_LogTask_Task	1
#define TSKID_UART	2
#define TSKID_LOOP	3

#define TNUM_SEMID	2
#define SEMID_tSemaphore_rProcessor1Migratable_SerialPort1_ReceiveSemaphore	1
#define SEMID_tSemaphore_rProcessor1Migratable_SerialPort1_SendSemaphore	2

#define TNUM_FLGID	0

#define TNUM_DTQID	1
#define DTQID_UART	1

#define TNUM_PDQID	0

#define TNUM_MTXID	0

#define TNUM_SPNID	0

#define TNUM_MPFID	0

#define TNUM_CYCID	0

#define TNUM_ALMID	0

#define TNUM_ISRID	2
#define ISRID_tISR_rProcessor1Migratable_SIOPortTarget1_ISRInstance	1
#define ISRID_PRC2	2

#define TNUM_INIRTN	4

#define TNUM_TERRTN	4

#endif /* TOPPERS_KERNEL_CFG_H */
