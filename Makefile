BUILD_DIR = build

# ------------------------------------------------------------------------------

MCUS = bcm2835 bcm2837 bcm2711

.PHONY: all svds pacs clean

all: svds pacs

svds: $(MCUS:%=$(BUILD_DIR)/svds/%.svd)

$(BUILD_DIR)/svds/%.svd: broadcom-peripherals/svd/gen/%_lpa.svd svd-patches/%.svd.patch | $(BUILD_DIR)/svds
	cp $< $@
	patch $@ svd-patches/$*.svd.patch

$(BUILD_DIR)/svds:
	@mkdir -p $@

pacs: $(MCUS:%=$(BUILD_DIR)/pacs/%-lpa)

$(BUILD_DIR)/pacs/%-lpa: $(BUILD_DIR)/svds/%.svd $(wildcard pac-template/*.jinja) pac-metadata/%.toml gen-pac.sh
	BUILD_DIR=$(BUILD_DIR) ./gen-pac.sh $*

clean:
	$(RM) -r $(BUILD_DIR)
