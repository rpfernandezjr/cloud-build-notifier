# Shell
SHELL = bash

# Colors
BLUE = \e[1;34m
GREEN = \e[0;32m
GREEN_BOLD = \e[1;32m
RED = \e[0;31m
YELLOW = \e[1;33m
LIGHT_CYAN = \e[1;36m
NC = \e[0m

# Multiple Targets
all: build success
install: move-binary success

# Targets
build:
	@echo
	@echo -e '${BLUE}building'
	@echo -e        '--------$(NC)'
	@cargo build --release

clean:
	@echo
	@echo -e '${RED}clean'
	@echo -e       '-----$(NC)'
	@cargo clean

move-binary:
	@echo
	@echo -e '${BLUE}Copying Binary'
	@echo -e        '-------$(NC)'
	@cp target/release/cloud-build-notifier /usr/local/bin/cloud-build-notifier

success:
	@echo
	@echo -e '$(GREEN_BOLD)ALL TARGETS COMPLETED$(NC)'
	@echo
