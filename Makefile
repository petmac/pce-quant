HUC_DIR := external/huc
HUC := $(HUC_DIR)/bin/huc

.PHONY: all
all: $(HUC)

.PHONY: clean
clean:
	$(MAKE) --directory $(HUC_DIR) clean

$(HUC):
	$(MAKE) --directory $(HUC_DIR)
