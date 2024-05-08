#include "stdio.h"
#include "malloc.h"
#include "pico/stdlib.h"

volatile uint32_t* AIRCR_REGISTER = (uint32_t*)(PPB_BASE + 0x0ED0C);
