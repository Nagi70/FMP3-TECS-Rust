#ifndef RUST_TECS_H
#define RUST_TECS_H
#include <kernel.h>

extern void tecs_rust_start_r_processor1_symmetric__uart_task(intptr_t exinf);
extern void tecs_rust_start_r_processor2_symmetric__button_task(intptr_t exinf);
extern void tecs_rust_start_r_processor1_symmetric__uart_isr(intptr_t exinf);
extern void tecs_rust_start_r_processor1_symmetric__uart_ini(intptr_t exinf);

#endif
