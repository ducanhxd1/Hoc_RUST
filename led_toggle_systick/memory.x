/* Memories definition for stm32f103c8t6 */
MEMORY
{
    RAM    (xrw)     : ORIGIN = 0x20000000,   LENGTH = 20K
    FLASH    (rx)    : ORIGIN = 0x8000000,   LENGTH = 64K
}

_start_of_stack = ORIGIN(RAM) + LENGTH(RAM); 
