MEMORY {
    mem (A) : ORIGIN = 0x70000000, LENGTH = 0x400000
}

ENTRY(_start);

SECTIONS {
    . = 0x70000000;
    foo 0x70000000 : { *(foo) } >mem
    . = 0x70001000;
    bar 0x70001000 : { *(bar) } >mem
}
