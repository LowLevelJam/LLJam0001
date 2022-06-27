#ifndef SALSA20_H
#define SALSA20_H

#include "common.h"

#define SALSA_ROUNDS            20
#define SALSA_INIT_STREAM_POS_0 0
#define SALSA_INIT_STREAM_POS_1 0
#define SALSA_KEY_WORDS         8
#define SALSA_NONCE_WORDS       2
#define SALSA_DATA_WORDS        16

typedef struct salsa_block_t {
  uint32_t row0[4];
  uint32_t row1[4];
  uint32_t row2[4];
  uint32_t row3[4];
} salsa_block_t;

typedef struct salsa_state_t {
  uint32_t key[SALSA_KEY_WORDS];
  uint64_t blockNumber;
  uint64_t nonce;
  salsa_block_t block;
} salsa_state_t;

void salsa_init(salsa_state_t* state);
void salsa_double_round(salsa_block_t* block);
void salsa_encrypt(salsa_state_t* state, const uint32_t dataIn[SALSA_DATA_WORDS], uint32_t cipherOut[SALSA_DATA_WORDS]);

void salsa_QR(uint32_t* a, uint32_t* b, uint32_t* c, uint32_t* d);
uint32_t salsa_rotate_uint32(uint32_t x, uint32_t rot);

uint32_t salsa_little_endian(uint32_t in);

#endif // SALSA20_H
