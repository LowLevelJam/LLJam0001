#include "salsa20.h"

void salsa_init(salsa_state_t* state) {
  state->block.row0[0] = 0x61707865; // "expa"
  state->block.row0[1] = state->key[0];
  state->block.row0[2] = state->key[1];
  state->block.row0[3] = state->key[2];

  state->block.row1[0] = state->key[3];
  state->block.row1[1] = 0x3320646e; // "nd 3"
  state->block.row1[2] = state->nonce & 0xffffffffL;
  state->block.row1[3] = state->nonce >> 32;

  state->block.row2[0] = state->blockNumber & 0xffffffffL;
  state->block.row2[1] = state->blockNumber >> 32;
  state->block.row2[2] = 0x79622d32; // "2-by"
  state->block.row2[3] = state->key[4];

  state->block.row3[0] = state->key[5];
  state->block.row3[1] = state->key[6];
  state->block.row3[2] = state->key[7];
  state->block.row3[3] = 0x6b206574; // "te k"
}

uint32_t salsa_rotate_uint32(uint32_t x, uint32_t rot) {
  const uint32_t rotMod = rot % 32;
  return (x << rotMod) | (x >> (32 - rotMod));
}

void salsa_QR(uint32_t* a, uint32_t* b, uint32_t* c, uint32_t* d) {
  *b ^= salsa_rotate_uint32(*a + *d, 7L);
  *c ^= salsa_rotate_uint32(*b + *a, 9L);
  *d ^= salsa_rotate_uint32(*c + *b, 13L);
  *a ^= salsa_rotate_uint32(*d + *c, 18L);
}

void salsa_double_round(salsa_block_t* block) {
  uint32_t* bp = (uint32_t*)block;

  salsa_block_t block2 = {0};
  memcpy(&block2, block, 64);
  uint32_t* bp2 = (uint32_t*)&block2;

  salsa_block_t x = {0};
  memcpy(&x, block, 64);
  uint32_t* xp = (uint32_t*)&x;

  for (size_t i = 0; i < SALSA_ROUNDS; i += 2) {
    // Odd round transforms columns
    salsa_QR(&bp[0],  &bp[4],  &bp[8],  &bp[12]);
    salsa_QR(&bp[5],  &bp[9],  &bp[13], &bp[1]);
    salsa_QR(&bp[10], &bp[14], &bp[2],  &bp[6]);
    salsa_QR(&bp[15], &bp[3],  &bp[7],  &bp[11]);

    // Even round transforms rows
    salsa_QR(&bp[0],  &bp[1],  &bp[2],  &bp[3]);
    salsa_QR(&bp[5],  &bp[6],  &bp[7],  &bp[4]);
    salsa_QR(&bp[10], &bp[11], &bp[8],  &bp[9]);
    salsa_QR(&bp[15], &bp[12], &bp[13], &bp[14]);
  }

  for (size_t i = 0; i < 16; i++) {
    bp[i] += bp2[i];
  }
}

void salsa_encrypt(salsa_state_t* state, const uint32_t dataIn[SALSA_DATA_WORDS], uint32_t cipherOut[SALSA_DATA_WORDS]) {
  salsa_init(state);
  state->blockNumber++;

  salsa_double_round(&state->block);

  uint32_t* blockPtr = (uint32_t*)(&state->block);

  for (size_t i = 0; i < SALSA_DATA_WORDS; i++) {
    cipherOut[i] = dataIn[i] ^ blockPtr[i];
  }
}

uint32_t salsa_little_endian(uint32_t in) {
  return (
      ((in & 0xff000000) >> 24)
    | ((in & 0x00ff0000) >> 8)
    | ((in & 0x0000ff00) << 8)
    | ((in & 0x000000ff) << 24)
  );
}
