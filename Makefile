PLUGIN_NAME = demo-plugin
PLUGIN_BIN  = xi-demo-plugin

# On MacOS, assume that plugins are in the default location (in
#  ~/Library/Application\ Support)
ifeq ($(shell uname -s), Darwin)
	XI_CONFIG_DIR ?= $(HOME)/Library/Application\ Support/XiEditor
endif

# Set variables for Xi config directories if they are not already set
XDG_CONFIG_HOME ?= $(HOME)/.config
XI_CONFIG_DIR ?= $(XDG_CONFIG_HOME)/xi
XI_PLUGIN_DIR ?= $(XI_CONFIG_DIR)/plugins

.PHONY: install
install: manifest.toml $(PLUGIN_BIN)
	mkdir -p $(XI_PLUGIN_DIR)/$(PLUGIN_NAME)/bin
	cp target/release/$(PLUGIN_BIN) $(XI_PLUGIN_DIR)/$(PLUGIN_NAME)/bin
	cp manifest.toml $(XI_PLUGIN_DIR)/$(PLUGIN_NAME)

.PHONY: $(PLUGIN_BIN)
$(PLUGIN_BIN):
	cargo build --release

.PHONY: clean
clean:
	cargo clean
