#include <stdio.h>
#include "heater.h"

//#define WASM_EXPORT(name) __attribute__((export_name(name)))
//#define WASM_IMPORT(name) __attribute__((import_name(name)))


int main() {
	gpu_command(99);
	printf("hi");
	return 0;
}
