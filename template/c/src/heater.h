#pragma once

#include <stdint.h>

#define WASM_EXPORT(name) __attribute__((export_name(name)))
#define WASM_IMPORT(name) __attribute__((import_name(name)))

//GPU interface

//registers
#define DX 0
#define DY 1
#define SX 2
#define SY 3
#define W 4
#define H 5
#define CC 6

//targets
#define TEX 0
#define FRAME 1

//gpu_reg_read(uint32_t reg)
//gpu_reg_write(uint32_t reg, int32_t value)
//gpu_load_texbuf(uint32_t addr, uint32_t color)
//gpu_load_framebuf(uint32_t addr, uint16_t color)
//gpu_read_framebuf(uint32_t addr)
//gpu_run_shader(uint32_t cacheid, uint32_2 target)
//gpu_command(uint32_t cmd)

//Shader Core interface

#define SR 0
#define FR 1

#define SG 2
#define FG 3

#define SB 4
#define FB 5

#define SA 6
#define FA 7

#define X 8
#define Y 9

#define R1 10
#define R2 11
#define R3 12
#define R4 13
#define R5 14
#define R6 15

//sc_load_cache(uint32_t cacheid, uint32_t addr,  uint32_t value)
//sc_read_reg(uint32_t reg)
//sc_write_reg(uint32_t reg, int32_t value)
//sc_clear_cache(uint32_t cacheid)

//Sound Chip interface

//sound_load_pcm(uint32_t soundid, uint32_t addr, int32 value)
//sound_write_reg(uint32_t reg, uint32_t value)
//sound_command(uint32 cmd)

//Disc interface

//disc_read_file(char* file)
//disc_run_program(char* file)

//Controler interface

//controler_buttons() <- returns a WORD in the format below
//0b0000 0000 0  0  000000
//  ^v>< +–×÷ LT RT Unused
