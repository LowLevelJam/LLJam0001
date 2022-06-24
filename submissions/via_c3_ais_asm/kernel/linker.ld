/* Minimal linker script */

SECTIONS {

    . = 0x400000;

    .rodata.keep : {
        KEEP( *(.rodata.keep) )
        . = ALIGN(16);
    }

    .rodata : {
        /* The .rodata.keep section is used for storing the multiboot header
        /* KEEP( *(.rodata.keep) ) */
        *(.rodata .rodata.*)
        . = ALIGN(4);
    }

    /* When using PIC, GOT is used to access symbols from other compalation units.
    * The linker fills the GOT, so it can be read-only
    * We place these after .rodata so we can MMU this range RO */
    .got.plt : {
        PROVIDE(__got_plt = .);
        *(.got.plt)
    }
    .got : {
        PROVIDE(__got = .);
        *(.got)
        . = ALIGN(16);
    }

    .text : {
        *(.text .text.*)
    }

    .data : {
        *(.data .data.*)
        . = ALIGN(16);
    }

    .bss : {
        *(.bss .bss.*)
    }

    . = 0x480000;
    .payload :  {
        KEEP( *(.payload) )
    }
}