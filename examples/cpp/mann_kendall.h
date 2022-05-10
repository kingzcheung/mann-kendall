#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * # Safety
 * 曼－肯德尔趋势检验
 *
 */
double mann_kendall(float *ptr, size_t len, double alpha);
