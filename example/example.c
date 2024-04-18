#include "huc.h"

#define OVL_BOOT 0
#define OVL_PROG 1
#define OVL_VIDEODATA 2

void main() {
  set_xres(320);
  set_color(0, 0);
  set_color(1, 511);

  // cd_loadvram(int ovl_idx, int sector_offset, int vram_addr, int bytes)
  cd_loadvram(OVL_VIDEODATA, 0, 0, 0x200);

  for (;;) {
    vsync();
  }
}
