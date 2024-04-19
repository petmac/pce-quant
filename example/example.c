#include "huc.h"

#define OVL_BOOT 0
#define OVL_PROG 1
#define OVL_VIDEODATA 2
#define BATM_SIZE (64 * 32 * 2)
#define CHR_SIZE ((320 / 8) * (256 / 8) * 32)
#define VRAM_SIZE (BATM_SIZE + CHR_SIZE)

void main() {
  set_xres(320);
  // cd_loadvram(int ovl_idx, int sector_offset, int vram_addr, int bytes)
  cd_loadvram(OVL_VIDEODATA, 0, 0, VRAM_SIZE);

  for (;;) {
    vsync();
  }
}
