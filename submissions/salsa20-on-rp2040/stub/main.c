#include "salsa20.h"
#include "stdio.h"

static void print_block_le(uint32_t* block) {
  printf("%08x%08x%08x%08x\n", salsa_little_endian(block[0]), salsa_little_endian(block[1]), salsa_little_endian(block[2]), salsa_little_endian(block[3]));
  printf("%08x%08x%08x%08x\n", salsa_little_endian(block[4]), salsa_little_endian(block[5]), salsa_little_endian(block[6]), salsa_little_endian(block[7]));
  printf("%08x%08x%08x%08x\n", salsa_little_endian(block[8]), salsa_little_endian(block[9]), salsa_little_endian(block[10]), salsa_little_endian(block[11]));
  printf("%08x%08x%08x%08x\n", salsa_little_endian(block[12]), salsa_little_endian(block[13]), salsa_little_endian(block[14]), salsa_little_endian(block[15]));
}

uint32_t dataIn[16] = {0};
uint32_t dataOut[16] = {0};

int main() {
  salsa_state_t salsa = {
    .key = {
      0x00000080, 0x00000000, 0x00000000, 0x00000000,
      0x00000000, 0x00000000, 0x00000000, 0x00000000,
    },
    .blockNumber = 0x0000000000000000ULL,
    .nonce = 0x0000000000000000ULL
  };

  salsa_init(&salsa);
  salsa_encrypt(&salsa, dataIn, dataOut);
  print_block_le((uint32_t*)(&salsa.block));

  salsa.blockNumber = 3;
  salsa_init(&salsa);
  salsa_encrypt(&salsa, dataIn, dataOut);
  print_block_le((uint32_t*)(&salsa.block));

  salsa.blockNumber = 4;
  salsa_init(&salsa);
  salsa_encrypt(&salsa, dataIn, dataOut);
  print_block_le((uint32_t*)(&salsa.block));

  salsa.blockNumber = 7;
  salsa_init(&salsa);
  salsa_encrypt(&salsa, dataIn, dataOut);
  print_block_le((uint32_t*)(&salsa.block));

  printf("\nKnown test vector:\n");
  printf("e3be8fdd8beca2e3ea8ef9475b29a6e7\n");
  printf("003951e1097a5c38d23b7a5fad9f6844\n");
  printf("b22c97559e2723c7cbbd3fe4fc8d9a07\n");
  printf("44652a83e72a9c461876af4d7ef1a117\n");
  printf("57be81f47b17d9ae7c4ff15429a73e10\n");
  printf("acf250ed3a90a93c711308a74c6216a9\n");
  printf("ed84cd126da7f28e8abf8bb63517e1ca\n");
  printf("98e712f4fb2e1a6aed9fdc73291faa17\n");
  printf("958211c4ba2ebd5838c635edb81f513a\n");
  printf("91a294e194f1c039aeec657dce40aa7e\n");
  printf("7c0af57cacefa40c9f14b71a4b3456a6\n");
  printf("3e162ec7d8d10b8ffb1810d71001b618\n");
  printf("696afcfd0cddcc83c7e77f11a649d79a\n");
  printf("cdc3354e9635ff137e929933a0bd6f53\n");
  printf("77efa105a3a4266b7c0d089d08f1e855\n");
  printf("cc32b15b93784a36e56a76cc64bc8477\n");

  return 0;
}