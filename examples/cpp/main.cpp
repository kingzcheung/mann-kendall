
#include <stdio.h>
#include <unistd.h>
#include <fcntl.h>
#include <termios.h>
#include <string.h>
#include <stdarg.h>
#include <stdlib.h>

extern "C" {
	#include "mann_kendall.h"
}

#pragma comment(lib, "../../target/debug/libmann_kendall.dylib")

void test_mann_kendall(void)
{
	float ptr[] = { 462.981506,465.075073,467.101562,471.965942,479.089600,484.824036,487.901184,490.867767,494.470215,495.570557,495.793640,497.071442,497.406555,496.069458,495.469177 };
	double k =mann_kendall( ptr, sizeof(ptr)/ sizeof(float), 0.05);
	printf("k=%lf\n");
}



int main(void)
{
// test_mann_kendall();
printf("%d", 1);
return 0;
}



