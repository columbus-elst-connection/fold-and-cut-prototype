.PHONY: all

SRC_DIR=src
JS_DIR=js
TARGET=${JS_DIR}/Fold.min.js

all: ${TARGET}
	@echo "building the client"

${JS_DIR}/%.min.js: ${JS_DIR}/%.js
	uglifyjs $< --compress 'pure_funcs="F2,F3,F4,F5,F6,F7,F8,F9,A2,A3,A4,A5,A6,A7,A8,A9",pure_getters,keep_fargs=false,unsafe_comps,unsafe' | uglifyjs --mangle --output=$@

${JS_DIR}/%.js: ${SRC_DIR}/%.elm ${SRC_DIR}/*.elm
	elm make $< --optimize --output=$@
