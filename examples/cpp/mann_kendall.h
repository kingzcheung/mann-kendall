#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Trend {
  double norm_stat;
  bool is_present;
} Trend;

/**
 * # Safety
 */
struct Trend mann_kendall_test(float *ptr, size_t len, double alpha);
