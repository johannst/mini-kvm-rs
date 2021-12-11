ARCH16 := i8086
ARCH64 := i386:x86-64

guest: guest16 guest64
disasm: disasm16 disasm64

guest%: guest.ld guest%.S
	$(CC) $(CFLAGS) -m$* -o $@ -nostdlib -ffreestanding -Wpedantic -Wall -Wextra -Werror -T guest.ld guest$*.S

disasm%: guest%
	objdump -D -b binary -m $(ARCH$*) -M intel $^

clean:
	$(RM) guest16 guest64
