js/figure.min.js: js/figure.js
	uglifyjs js/figure.js --compress 'pure_funcs="F2,F3,F4,F5,F6,F7,F8,F9,A2,A3,A4,A5,A6,A7,A8,A9",pure_getters,keep_fargs=false,unsafe_comps,unsafe' | uglifyjs --mangle --output=js/figure.min.js

js/figure.js: src/Figure.elm
	elm make src/Figure.elm --optimize --output=js/figure.js
