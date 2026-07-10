#ifndef BLACKHEAD_RUNTIME_H
#define BLACKHEAD_RUNTIME_H

#include <stdint.h>
#define BH_RUNTIME_ABI_VERSION 0u

#ifdef __cplusplus
extern "C" {
#endif
uint32_t bh_runtime_abi_version(void);
#ifdef __cplusplus
}
#endif

#endif
