PCE_QUANT := target/release/pce-quant

EXAMPLE_PNG := temp/example.png
EXAMPLE_ISO := temp/example.iso

HUC_DIR := external/huc
HUC := $(HUC_DIR)/bin/huc
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

EXAMPLE_OVERLAYS := example/example.ovl temp/example.vram

$(EXAMPLE_ISO): $(EXAMPLE_OVERLAYS) $(ISOLINK)
	mkdir -p $(dir $@)
	$(ISOLINK) $@ $(EXAMPLE_OVERLAYS)

export PCE_INCLUDE := $(HUC_DIR)/include/huc
export PCE_PCEAS := $(HUC_DIR)/bin/pceas

example/example.ovl: example/example.c $(HUC)
	$(HUC) -cd -v -fno-recursive -msmall -over $<

$(EXAMPLE_PNG): example/images/320x256/ff7_1.png $(PCE_QUANT)
	$(PCE_QUANT) $< png $@

temp/example.vram: example/images/320x256/ff7_1.png $(PCE_QUANT)
	$(PCE_QUANT) $< vram $@

$(PCE_QUANT): cargo-build

.PHONY: cargo-build
cargo-build:
	cargo build --release

$(HUC) $(ISOLINK):
	$(MAKE) --directory $(HUC_DIR)

$(MESEN):
	$(MAKE) --directory $(MESEN_DIR)
