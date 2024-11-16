LDFLAGS=
SFLAGS=-f elf64 -g

all: hex-maze

hex-maze: src/main.o src/term.o
	$(CC) $(LDFLAGS) -o $@ $^

%.o: %.s
	nasm $(SFLAGS) -o $@ $<

clean:
	rm -f **/*.o hex-maze

.PHONY: clean
