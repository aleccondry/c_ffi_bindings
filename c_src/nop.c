

void nop(void) {
    for (int i = 0; i < 1000000; i++) {
        /// Perform a NOP operation
        __asm__ __volatile__("nop");
    }
}
