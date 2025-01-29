#ifndef RUSTTECS_H
#define RUSTTECS_H
#include <kernel.h>

#define DEFAULT_PRIORITY    8   /* タスクの優先度 */
#define MID_PRIORITY    7   /* タスクの優先度 */
#define STACK_SIZE        4096  /* タスクのスタックサイズ */

#ifndef   TOPPERS_MACRO_ONLY

extern void rust_start1(intptr_t exinf);
extern void rust_start2(intptr_t exinf);

#endif   /* TOPPERS_MACRO_ONLY */
#endif   /* STUDY_H */