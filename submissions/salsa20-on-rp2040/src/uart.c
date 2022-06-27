#include "uart.h"

bool setup_uart() {
  bool result;

  // Setup uart peripheral
  result = uart_init(uart0, BAUD_RATE) == BAUD_RATE;
  uart_set_format(uart0, 8, 1, 0);

  // Setup gpio
  gpio_set_function(UART_TX_PIN, GPIO_FUNC_UART);
  gpio_set_function(UART_RX_PIN, GPIO_FUNC_UART);

  return result;
}

uint32_t uart_receive_u8() {
  uint8_t byte;
  uart_read_blocking(uart0, &byte, 1);
  return byte;
}

uint32_t uart_receive_u16le() {
  uint bytesReceived = 0;
  uint16_t bytes = 0;

  while (bytesReceived < 2) {
    if (uart_is_readable(uart0)) {
      bytes |= uart_getc(uart0) << (bytesReceived * 8);
      bytesReceived++;
    }
  }

  return bytes;
}


uint32_t uart_receive_u32le() {
  uint bytesReceived = 0;
  uint32_t bytes = 0;

  while (bytesReceived < 4) {
    if (uart_is_readable(uart0)) {
      bytes |= uart_getc(uart0) << (bytesReceived * 8);
      bytesReceived++;
    }
  }

  return bytes;
}

void uart_send_u32le(uint32_t data) {
  uint8_t byte;
  for (size_t i = 0; i < 4; i++) {
    byte = (data & 0xff);
    data >>= 8;
    uart_write_blocking(uart0, &byte, 1);
  }
}

void uart_send_u16le(uint16_t data) {
  uint8_t byte;
  for (size_t i = 0; i < 2; i++) {
    byte = (data & 0xff);
    data >>= 8;
    uart_write_blocking(uart0, &byte, 1);
  }
}

void uart_send_u8(uint8_t data) {
    uart_write_blocking(uart0, &data, 1);
}
