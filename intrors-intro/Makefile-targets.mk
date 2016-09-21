# Targets Makefile script.
#----------------------------------------
# Common automatic variables legend (GNU make: make (Linux) gmake (FreeBSD)):
# $* - basename (cur target)  $^ - name(s) (all depns)  $< - name (1st depn)
# $@ - name (cur target)      $% - archive member name  $? - changed depns

distdir = $(proj)-$(version)

.PHONY: help clean test dist doc report uninstall install
help: ## help
#	@echo "##### subproject: $(proj) #####"
#	@echo "Usage: $(MAKE) [target] -- some valid targets:"
##	-@for fileX in $(MAKEFILE_LIST) `if [ -z "$(MAKEFILE_LIST)" ] ; then echo Makefile ./Makefile-targets.mk ; fi` ; do \
##		grep -ve '^[A-Z]' $$fileX | awk '/^[^.%][-A-Za-z0-9_]+[ ]*:.*$$/ { print "...", substr($$1, 1, length($$1)) }' | sort ; \
##	done
#	-@for fileX in $(MAKEFILE_LIST) `if [ -z "$(MAKEFILE_LIST)" ] ; then echo Makefile ./Makefile-targets.mk ; fi` ; do \
#		grep -E '^[ a-zA-Z_-]+:.*?## .*$$' $$fileX | \
#		awk 'BEGIN {FS = ":.*?## "}; {printf "%-25s%s\n", $$1, $$2}' ; \
#	done
	-cargo help
clean: ## clean build artifacts
	-cargo clean -p $(proj)
	-rm -fr core* *.profraw .coverage *.gcda *.gcno *.log
test: ## run test [TOPTS=""]
#	export [DY]LD_LIBRARY_PATH=. # ([da|ba|z]sh Linux)
#	setenv [DY]LD_LIBRARY_PATH . # (tcsh FreeBSD)
	-RUST_LOG=quickcheck LD_LIBRARY_PATH=$(LD_LIBRARY_PATH):lib cargo test \
		$(PROFILEFLAGS) --tests -p $(proj) $(TOPTS)
uninstall install: ## [un]install executable(s)
	-@if [ "uninstall" = "$@" ] ; then \
		cargo uninstall -p $(proj) ; \
	else \
		cargo install $(PROFILEFLAGS) --path . ; \
	fi
dist: ## archive source code
	-cargo package --allow-dirty --no-verify
	-cp $(par_dir)/target/package/$(distdir).crate \
		$(par_dir)/target/package/$(distdir).tar.gz
	-tar -xf $(par_dir)/target/package/$(distdir).tar.gz \
		-C $(par_dir)/target/package
	-cp exclude.lst $(par_dir)/target/package/
	-(cd $(par_dir)/target/package ; zip -9 -q --exclude @exclude.lst -r $(distdir).zip $(distdir))
	-@rm -r $(par_dir)/target/package/$(distdir)
doc: ## generate documentation [OPTS="??"]
	-cargo doc --no-deps $(PROFILEFLAGS) $(OPTS) -p $(proj)
#lint: ## lint check [OPTS="??"]
#	-rustfmt --check $(OPTS) src/lib.rs
report: ## report code coverage
#	# read coverage data w/ [llvm-cov] gcov -f -b -n *.gcda
#	find . -type f -name '*.gcda' -exec llvm-cov gcov -f -b --no-output {} \;
#	# read coverage data w/ lcov -c -d . -o .coverage ... *.gcda
	-mkdir -p build/cov
	-lcov --capture -d . -o build/.coverage --gcov-tool ./llvm-gcov.sh
	-genhtml -o build/cov build/.coverage
