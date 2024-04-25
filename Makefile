PCE_QUANT := target/release/pce-quant

EXAMPLE_PNG := temp/example.png
EXAMPLE_PCE := temp/example.pce

LLVM_MOS_URL := https://github.com/llvm-mos/llvm-mos-sdk/releases/latest/download/llvm-mos-macos.tar.xz
LLVM_MOS_ARCHIVE := temp/llvm-mos-macos.tar.xz
LLVM_MOS_BIN_DIR := temp/llvm-mos/bin
LLVM_MOS_CC := $(LLVM_MOS_BIN_DIR)/mos-pce-clang

MACHINE := $(shell uname -m)
ifeq ($(filter arm%,$(MACHINE)),)
	MESEN_PLATFORM := osx-x64
else
	MESEN_PLATFORM := osx-arm64
endif
MESEN_DIR := external/Mesen2
MESEN := $(MESEN_DIR)/bin/$(MESEN_PLATFORM)/Release/$(MESEN_PLATFORM)/publish/Mesen

.PHONY: all
all: $(EXAMPLE_PNG) $(EXAMPLE_PCE) $(MESEN)
	$(MESEN) $(shell pwd)/$(EXAMPLE_PCE)

.PHONY: clean
clean:
	rm -f $(EXAMPLE_PNG)
	rm -rf temp/
	cargo clean
	$(MAKE) --directory $(MESEN_DIR) clean

$(EXAMPLE_PCE): example/example.c $(LLVM_MOS_CC)
	$(LLVM_MOS_CC) -Os $< -o $@

$(EXAMPLE_PNG): example/images/320x256/ff7_1.png $(PCE_QUANT)
	mkdir -p $(dir $@)
	$(PCE_QUANT) $< png $@

temp/example.vram: example/images/320x256/ff7_1.png $(PCE_QUANT)
	mkdir -p $(dir $@)
	$(PCE_QUANT) $< vram $@

$(PCE_QUANT): cargo-build

.PHONY: cargo-build
cargo-build:
	cargo build --release

$(LLVM_MOS_CC): $(LLVM_MOS_ARCHIVE)
	tar --extract --gunzip --file $< --directory temp/ -m

$(LLVM_MOS_ARCHIVE):
	curl -L -o $@ $(LLVM_MOS_URL)

$(MESEN):
	$(MAKE) --directory $(MESEN_DIR)
