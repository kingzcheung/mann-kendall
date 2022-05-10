
## Usage/Examples

### install

```yaml
[dependencies]
mann_kendall = "0.1"

```

### build
本地编译

```shell
cargo build
```

### rust usage
```rust

use distrs::Normal;

fn main() {
    let x = vec![202.175842,222.266602,250.772827,300.741455,350.643982,369.749268,400.998474,479.279663,486.617310,487.517456,491.321625];
    let res = mann_kendall::test(x, 0.05);

    println!("z:: {}",res.0);
    println!("h:: {}",res.1);
}
 
```

### cpp usage

详见 [examples](./examples/cpp) 

```cpp

extern "C"
{
#include "mann_kendall.h"
}

void test_mann_kendall(void)
{
	float ptr[] = {404.175842,421.266602,439.772827,453.741455,458.643982,469.749268,475.998474,479.279663,486.617310,487.517456,491.321625};
	Trend k = mann_kendall_test(ptr, sizeof(ptr) / sizeof(float), 0.05);
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

```