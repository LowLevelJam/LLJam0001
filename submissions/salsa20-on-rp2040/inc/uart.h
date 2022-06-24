#ifndef SALSA_UART_H
#define SALSA_UART_H

#include "hardware/uart.h"
#include "common.h"

#define BAUD_RATE 115200

#define UART_TX_PIN 0
#define UART_RX_PIN 1

bool setup_uart(void);

uint32_t uart_receive_u32le();
uint32_t uart_receive_u16le();
uint32_t uart_receive_u8();
void uart_send_u32le(uint32_t data);
void uart_send_u16le(uint16_t data);
void uart_send_u8(uint8_t data);

#endif
