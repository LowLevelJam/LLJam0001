# Salsa20 Hardware Encryption Device

## Team

1. Francis Stokes

## Introduction

**Low Level Jam 0001 Entry**

The goal was to create an embedded Salsa20 encryption device using a $4 Raspberry Pi Pico (RP2040).

At boot, the device waits for a uart connection, and a handshake takes place to transfer the 256-bit encryption key, as well as a 64-bit nonce value.

After the key and nonce have been transferred, the device waits for an encryption request. Encryption always takes place on 64-bytes of data at a time, which is split into a 16-byte packets + an additional CRC8 byte. The CRC is checked by the device, and if they do not match (because of line noise, for example), the packet is rejected and expected to be sent again. When all 64-bytes have been transferred, the encrypts the data and sends it back in the same fashion: 16-byte packets + a CRC byte. The receiver checks the data validity and can request a packet to be resent if required. When the transfer is complete, the internal counter is incremented, and the device waits for the next encryption request.

Because the key and nonce are only ever stored in RAM, when the device loses power, the secret information is no longer recoverable. Still, I would not trust this device with any real encryption duties.

The algorithm can be tested locally on a PC by compiling the stub `main.c`, which encrypts using a known test vector.

## Building the local stub

```bash
make -f local.mk
```

## Building the firmware

- Install the RPi Pico SDK following the [instructions on github](https://github.com/raspberrypi/pico-sdk)
- After setting the `PICO_SDK_PATH` env variable correctly, run:

```bash
build.sh
```

- `build/src` will contain various artifacts, including a `bin`, `hex`, `elf`, and `uf2` firmware file that can be *drag-and-dropped* to the pico when it appears as a mass storage device in boot mode
