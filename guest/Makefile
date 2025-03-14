ARCH16 := i8086
ARCH64 := i386:x86-64

bits = $(word 1, $(subst -, ,$(1)))

guest: guest16 guest64 guest64-msr
disasm: disasm16 disasm64 disasm64-msr

guest%: guest.ld guest%.S
	$(CC) $(CFLAGS) -m$(call bits, $*) -o $@ -nostdlib -ffreestanding -Wpedantic -Wall -Wextra -Werror -T guest.ld guest$*.S

disasm%: guest%
	objdump -D -b binary -m $(ARCH$(call bits, $*)) -M intel $^

clean:
	$(RM) guest16 guest64 guest64-msr
