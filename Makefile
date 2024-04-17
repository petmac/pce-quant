PCE_QUANT := target/release/pce-quant

OUTPUT_PNG := images/output.png

HUC_DIR := external/huc
HUC := $(HUC_DIR)/bin/huc

MACHINE := $(shell uname -m)
ifeq ($(filter arm%,$(MACHINE)),)
	MESEN_PLATFORM := osx-x64
else
	MESEN_PLATFORM := osx-arm64
endif
MESEN_DIR := external/Mesen2
MESEN := $(MESEN_DIR)/bin/$(MESEN_PLATFORM)/Release/$(MESEN_PLATFORM)/publish/Mesen

.PHONY: all
all: $(OUTPUT_PNG) $(HUC) $(MESEN)
	$(MESEN) $(shell pwd)/$(HUC_DIR)/examples/huc/overlay/overlay.cue

.PHONY: clean
clean:
	cargo clean
	$(MAKE) --directory $(HUC_DIR) clean
	$(MAKE) --directory $(MESEN_DIR) clean

$(OUTPUT_PNG): images/320x256/ff7_1.png $(PCE_QUANT)
	$(PCE_QUANT) $< png $@

$(PCE_QUANT): cargo-build

.PHONY: cargo-build
cargo-build:
	cargo build --release

$(HUC):
	$(MAKE) --directory $(HUC_DIR)

$(MESEN):
	$(MAKE) --directory $(MESEN_DIR)
