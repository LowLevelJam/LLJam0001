#include "crc8.h"

// Adapted from https://www.devcoons.com/crc8/
uint8_t crc8(const uint8_t* data, size_t length) {
  uint8_t crc = 0x00;
  uint8_t byte;
  uint8_t sum;

  for(size_t i = 0; i < length; i++) {
     byte = *data;
     for (uint8_t j = 8; j > 0; j--) {
        sum = (crc ^ byte) & 0x01;
        crc >>= 1;
        if (sum) {
          crc ^= 0x8C;
        }
        byte >>= 1;
     }
     data++;
  }

  return crc;
}
