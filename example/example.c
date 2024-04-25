#include <pce-cd.h>

uint16_t ticks = 0;

int main(void) {
  while (1) {
    pce_cdb_wait_vblank();
    ++ticks;
  }
}
