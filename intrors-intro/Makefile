.POSIX:
.SUFFIXES:
PREFIX = /home/kenneth/.local
VPATH = .

# Single package Makefile script.
.SUFFIXES: .s .o .a .rs .rlib
help:

proj_dir = $(shell pwd)
par_dir = $(shell dirname $(proj_dir))
PKG_CONFIG = pkg-config --with-path=$(PREFIX)/lib/pkgconfig

parent = intrors
pkg = intro
proj = $(parent)_$(pkg)
#outbase = $(shell echo $(proj) | tr '-' '_')

version = 0.1.0
ver_major = $(shell echo $(version) | cut -d'.' -f1)

ifdef DEBUG
CONFIG = debug
else
CONFIG = release
PROFILEFLAGS := $(PROFILEFLAGS) --release
endif

.PHONY: build testCompile run debug valgrind

build: ## compile
	-PKG_CONFIG_PATH=$(PREFIX)/lib/pkgconfig cargo build $(PROFILEFLAGS) \
		-p $(proj)
testCompile: tests/ts_main.rs ## compile test
	-PKG_CONFIG_PATH=$(PREFIX)/lib/pkgconfig cargo test --no-run \
		$(PROFILEFLAGS) -p $(proj)

DEBUGGER = ddd --gdb	# rust-lldb ; ddd --gdb ; rust-gdb
# valgrind tools: memcheck helgrind cachegrind massif lackey
VALGRIND = valgrind --verbose --tool=memcheck

run: ## run main [ARGS=""]
#	export [DY]LD_LIBRARY_PATH=. # ([da|ba|z]sh Linux)
#	setenv [DY]LD_LIBRARY_PATH . # (tcsh FreeBSD)
	-LD_LIBRARY_PATH=$(LD_LIBRARY_PATH):lib cargo run --bin $(parent)-main \
		$(PROFILEFLAGS) -p $(proj) -- $(ARGS)
debug: ## debug main [ARGS=""]
	-LD_LIBRARY_PATH=$(LD_LIBRARY_PATH):lib $(DEBUGGER) cargo -- run \
		--bin $(parent)-main $(PROFILEFLAGS) -p $(proj) -- $(ARGS)
valgrind: ## valgrind main
	-LD_LIBRARY_PATH=$(LD_LIBRARY_PATH):lib $(VALGRIND) cargo -- run \
		--bin $(parent)-main $(PROFILEFLAGS) -p $(proj)

-include .depend ./Makefile-targets.mk
#-include ./Makefile-auxffi.mk
