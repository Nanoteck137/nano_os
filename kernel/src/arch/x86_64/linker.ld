ENTRY(boot_entry)

SECTIONS {
    . = 1M;

    .boot :
    {
        *(.multiboot_header)
    }

    .text :
    {
        *(.text .text.*)
    }

    .rodata : 
    {
        *(.rodata .rodata.*)
    }

    .data.rel.ro : 
    {
        *(.data.rel.ro.local*) *(.data.rel.ro .data.rel.ro.*)
    }
}
