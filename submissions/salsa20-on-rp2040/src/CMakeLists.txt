add_executable(salsa20
  main.c
  salsa20.c
  uart.c
  crc8.c
)

include_directories(AFTER
  ../inc
)

# pull in common dependencies
target_link_libraries(salsa20 pico_stdlib)

# create map/bin/hex file etc.
pico_add_extra_outputs(salsa20)
