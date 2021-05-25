guest16: guest.ld guest16.S
	$(CC) $(CFLAGS) -m16 -o $@ -nostdlib -ffreestanding -Wpedantic -Wall -Wextra -Werror -T guest.ld guest16.S

disasm: guest16
	objdump -D -b binary -m i8086 -M intel $^

clean:
	$(RM) guest16
