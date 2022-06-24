#include "salsa20.h"
#include "uart.h"
#include "crc8.h"

// Communication protocol constants
#define SEND_KEY_REQ        (0x13)
#define SEND_NONCE_REQ      (0x14)
#define ENC_REQ             (0x15)
#define SEND_CIPHER_REQ     (0x15)
#define ACK                 (0xB1)
#define NACK                (0xC2)

// Internal salsa state
static salsa_state_t salsa = {0};
static uint32_t dataIn[SALSA_DATA_WORDS];
static uint32_t dataOut[SALSA_DATA_WORDS];
static size_t dataInIndex = 0;
static size_t dataOutIndex = 0;

// Function prototypes
static void read_key(salsa_state_t* state);
static void read_nonce(salsa_state_t* state);
static void wait_for_request(uint8_t reqType, bool sendAck);
static void receive_packet();
static void send_packet();
static void setup_salsa();
static void error_handler();

int main() {
  if (!setup_uart()) {
    error_handler();
  }

  setup_salsa();

  uint8_t crcByte;
  while (1) {
    wait_for_request(ENC_REQ, true);

    // Receive 64 bytes over 4 16-byte packets
    // After each packet an CRC byte is sent and checked
    for (size_t i = 0; i < 4; i++) {
      receive_packet();
    }

    // When we have 64 bytes, we can encrypt
    salsa_encrypt(&salsa, dataIn, dataOut);

    // Send the encrypted bytes
    uart_send_u8(SEND_CIPHER_REQ);
    if (uart_receive_u8() != ACK) {
      // Something went wrong, wait for new encryption request
      error_handler();
      // continue;
    }

    // Send the cipher back over 4 16-byte packets, along with a CRC byte
    for (size_t i = 0; i < 4; i++) {
      send_packet();
    }

    // Wait for the next request
  }

  // Should never get here
  return 0;
}

// Protocol functions
static void send_packet() {
  uint8_t crcByte;
  while (1) {
    // Send out 16 bytes of data
    for (size_t i = 0; i < 4; i++) {
      uart_send_u32le(dataOut[(dataOutIndex++) + i]);
    }

    // Calculate and send the CRC
    crcByte = crc8((uint8_t*)(&dataOut[dataOutIndex - 4]), 16);
    uart_send_u8(crcByte);

    // If something went wrong in trasmission, start this packet again
    if (uart_receive_u8() != ACK) {
      // Move the index back to the start of the buffer in order to transfer again
      dataOutIndex = 0;
      continue;
    }

    // If all is well, we're done
    return;
  }
}

static void receive_packet() {
  uint8_t crcByte;
  while (1) {
    // Get 16 bytes of data
    for (size_t i = 0; i < 4; i++) {
      dataIn[(dataInIndex++) + i] = uart_receive_u32le();
    }

    // Get and check the CRC
    crcByte = uart_receive_u8();
    if (crc8((uint8_t*)(&dataIn[dataInIndex - 4]), 16) != crcByte) {
      uart_send_u8(NACK);
      dataInIndex = 0;
      continue;
    }

    // If all is well, ACK and return
    uart_send_u8(ACK);
    return;
  }
}

static void read_key(salsa_state_t* state) {
  for (size_t i = 0; i < SALSA_KEY_WORDS; i++) {
    state->key[i] = uart_receive_u32le();
  }
}

static void read_nonce(salsa_state_t* state) {
  for (size_t i = 0; i < SALSA_NONCE_WORDS; i++) {
    state->nonce |= uart_receive_u32le() << (i * 32);
  }
}

static void wait_for_request(uint8_t reqType, bool sendAck) {
  uint8_t byte;
  while (true) {
    byte = uart_receive_u8();
    if (byte == reqType) {
      if (sendAck) {
        uart_send_u8(ACK);
      }
      return;
    }
  }
}

static void setup_salsa() {
  wait_for_request(SEND_KEY_REQ, true);
  read_key(&salsa);
  uart_send_u8(ACK);

  wait_for_request(SEND_NONCE_REQ, true);
  read_nonce(&salsa);
  uart_send_u8(ACK);
}

// Error handler
static void error_handler() {
  while (1) {
    __breakpoint();
  }
}
