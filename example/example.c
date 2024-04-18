#include "huc.h"

void main() {
  set_xres(328);
  set_color(0, 0);
  set_color(1, 511);

  put_string("Example", 0, 0);

  for (;;) {
    vsync();
  }
}
