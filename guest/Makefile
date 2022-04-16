guest: guest16 guest64
disasm: disasm16 disasm64

guest16: guest.ld guest16.S
	$(CC) $(CFLAGS) -m16 -o $@ -nostdlib -ffreestanding -Wpedantic -Wall -Wextra -Werror -T guest.ld guest16.S

guest64: guest.ld guest64.S
	$(CC) $(CFLAGS) -m64 -o $@ -nostdlib -ffreestanding -Wpedantic -Wall -Wextra -Werror -T guest.ld guest64.S

disasm16: guest16
	objdump -D -b binary -m i8086 -M intel $^

disasm64: guest64
	objdump -D -b binary -m i386:x86-64 -M intel $^

clean:
	$(RM) guest16 guest64
