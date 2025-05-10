# SPDX-License-Identifier: MIT
#
# Copyright (c) 2021, Johannes Stoelp <dev@memzero.de>

OUT ?= /dev/stdout

gen: kvm
	./kvm > $(OUT)

kvm: kvm.c
	$(CC) $(CFLAGS) -Wpedantic -Wall -Wextra -Werror -o $@ $^

clean:
	$(RM) kvm
