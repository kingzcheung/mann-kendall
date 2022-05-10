
#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>
#include <string.h>
#include <stdarg.h>
#include <stdlib.h>

extern "C"
{
#include "mann_kendall.h"
}

void test_mann_kendall(void)
{
	float ptr[] = {404.175842,421.266602,439.772827,453.741455,458.643982,469.749268,475.998474,479.279663,486.617310,487.517456,491.321625};
	Trend k = mann_kendall_a(ptr, sizeof(ptr) / sizeof(float), 0.05);
	printf("norm_stat=%lf\n",k.norm_stat);
	if (k.is_present) {
		printf("is_present=%s\n","true");
	}else {
		printf("is_present=%s\n","false");
	}
}

int main(void)
{
	test_mann_kendall();
	return 0;
}
