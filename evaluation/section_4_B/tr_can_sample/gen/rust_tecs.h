#ifndef RUST_TECS_H
#define RUST_TECS_H
#include <kernel.h>

extern void tecs_rust_start_r_processor1_symmetric_can_task(intptr_t exinf);
extern void tecs_rust_start_r_processor2_symmetric_loop_task(intptr_t exinf);

#endif
