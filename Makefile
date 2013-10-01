# Copyright © 2013 Free Software Foundation, Inc
# See licensing in LICENSE file
#
# File: Makefile
# Author: Jesse 'Jeaye' Wilkerson
# Description:
# 	Builds ncurses-rs

SRC=src/lib.rs src/ll.rs src/constants.rs

.SILENT:

all: setup ${SRC}
	echo "Building ncurses-rs"
	rustc --out-dir lib src/lib.rs
	echo "Success \o/"

setup:
	mkdir -p lib

