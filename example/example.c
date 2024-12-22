#include "huc.h"

#define OVL_BOOT 0
#define OVL_PROG 1
#define OVL_PALETTE 2
#define OVL_VIDEODATA 3
#define BATM_SIZE (64 * 32 * 2)
#define CHR_SIZE ((320 / 8) * (256 / 8) * 32)
#define VRAM_SIZE (BATM_SIZE + CHR_SIZE)

static int palettes[16 * 16];

void main() {
  set_xres(320);
  scroll(0, 0, 8, 0, 223, 0xc0);
  cd_loaddata(OVL_PALETTE, 0, palettes, sizeof(palettes));
  set_bgpal(0, palettes, 16);
  // cd_loadvram(int ovl_idx, int sector_offset, int vram_addr, int bytes)
  cd_loadvram(OVL_VIDEODATA, 0, 0, VRAM_SIZE);

  for (;;) {
    vsync();
  }
}
