PCE_QUANT := target/release/pce-quant

EXAMPLE_PNG := temp/example.png
EXAMPLE_ISO := temp/example.iso

HUC_DIR := external/huc
HUC := $(HUC_DIR)/bin/hucc
ISOLINK := $(HUC_DIR)/bin/isolink

MACHINE := $(shell uname -m)
ifeq ($(filter arm%,$(MACHINE)),)
	MESEN_PLATFORM := osx-x64
else
	MESEN_PLATFORM := osx-arm64
endif
MESEN_DIR := external/Mesen2
MESEN := $(MESEN_DIR)/bin/$(MESEN_PLATFORM)/Release/$(MESEN_PLATFORM)/publish/Mesen

.PHONY: all
all: $(EXAMPLE_PNG) $(EXAMPLE_ISO) $(MESEN)
	$(MESEN) $(shell pwd)/example/example.cue

.PHONY: clean
clean:
	rm -f $(EXAMPLE_PNG)
	rm -rf temp/
	cargo clean
	$(MAKE) --directory $(HUC_DIR) clean
	$(MAKE) --directory $(MESEN_DIR) clean

EXAMPLE_OVERLAYS := example/example.ovl temp/example.pal temp/example.vram

$(EXAMPLE_ISO): $(EXAMPLE_OVERLAYS) $(ISOLINK)
	mkdir -p $(dir $@)
	$(ISOLINK) $@ $(EXAMPLE_OVERLAYS)

export PCE_INCLUDE := $(HUC_DIR)/include/hucc
export PCE_PCEAS := $(HUC_DIR)/bin/pceas

example/example.ovl: example/example.c $(HUC)
	$(HUC) -cd -v -fno-recursive -msmall -over $<

$(EXAMPLE_PNG) temp/example.vram temp/example.pal: example/images/320x256/ff7_1.png $(PCE_QUANT)
	mkdir -p $(dir $@)
	$(PCE_QUANT) $< --png $(EXAMPLE_PNG) --vram temp/example.vram --palettes temp/example.pal

$(PCE_QUANT): cargo-build

.PHONY: cargo-build
cargo-build:
	cargo build --release

$(HUC) $(ISOLINK):
	$(MAKE) --directory $(HUC_DIR)

$(MESEN):
	$(MAKE) --directory $(MESEN_DIR)
