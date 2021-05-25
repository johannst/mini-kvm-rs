OUT ?= /dev/stdout

gen: kvm
	./kvm > $(OUT)

kvm: kvm.c
	$(CC) $(CFLAGS) -Wpedantic -Wall -Wextra -Werror -o $@ $^

clean:
	$(RM) kvm
