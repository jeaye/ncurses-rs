# Sources
LIB_SRC = src/ncurses.rs
LIB_DEPS = $(shell head -n1 target/.ncurses.deps 2> /dev/null)
EXAMPLES_SRC = $(wildcard examples/*.rs)

# Objects
LIB = target/$(shell rustc --print-file-name ${LIB_SRC})
EXAMPLES_BIN = $(EXAMPLES_SRC:examples/%.rs=bin/%)

# CFG Directive Options
CFG_OPT ?= -O

SED_OPT=-i

ifeq ($(shell uname),Darwin)
	SED_OPT=-i ''
endif

all: ${LIB} ${EXAMPLES_BIN}

lib: ${LIB}

link-ncursesw: CFG_OPT = --cfg ncursesw
link-ncursesw: all

${LIB}: ${LIB_DEPS}
	@mkdir -p target
	rustc ${CFG_OPT} --out-dir target ${LIB_SRC}
	@rustc --no-trans --dep-info target/.ncurses.deps ${LIB_SRC}
	@sed ${SED_OPT} 's/.*: //' target/.ncurses.deps

${EXAMPLES_BIN}: bin/%: examples/%.rs ${LIB}
	@mkdir -p bin
	rustc --out-dir bin -L target $<

clean:
	rm -rf target bin

.PHONY: all clean link-ncursesw
