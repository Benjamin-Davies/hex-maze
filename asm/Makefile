LDFLAGS=
SFLAGS=-f elf64 -g

all: hex-maze

hex-maze: src/main.o src/generator.o src/grid.o src/hex.o src/maze.o src/terminal.o src/vec.o
	$(CC) $(LDFLAGS) -o $@ $^

%.o: %.s
	nasm $(SFLAGS) -o $@ $<

clean:
	rm -f **/*.o hex-maze

.PHONY: clean
