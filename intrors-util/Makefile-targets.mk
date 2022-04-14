# Targets Makefile script.
#----------------------------------------
# Common automatic variables legend (GNU make: make (Linux) gmake (FreeBSD)):
# $* - basename (cur target)  $^ - name(s) (all depns)  $< - name (1st depn)
# $@ - name (cur target)      $% - archive member name  $? - changed depns

FMTS ?= tar.gz,zip
distdir = $(proj)-$(version)

.PHONY: help clean test dist doc report
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
dist: ## [FMTS="tar.gz,zip"] archive source code
	-cargo package --allow-dirty --no-verify
	-cp $(par_dir)/target/package/$(distdir).crate \
		$(par_dir)/target/package/$(distdir).tar.gz
	-tar -xf $(par_dir)/target/package/$(distdir).tar.gz \
		-C $(par_dir)/target/package
	-cp exclude.lst $(par_dir)/target/package/
	-@for fmt in `echo $(FMTS) | tr ',' ' '` ; do \
		case $$fmt in \
			7z) echo "### $(par_dir)/target/package/$(distdir).7z ###" ; \
				rm -f $(par_dir)/target/package/$(distdir).7z ; \
				(cd $(par_dir)/target/package ; 7za a -t7z -mx=9 $(distdir).7z $(distdir)) ;; \
			zip) echo "### $(par_dir)/target/package/$(distdir).zip ###" ; \
				rm -f $(par_dir)/target/package/$(distdir).zip ; \
				(cd $(par_dir)/target/package ; zip -9 -q --exclude @exclude.lst -r $(distdir).zip $(distdir)) ;; \
			tar.gz) ;;
			*) tarext=`echo $$fmt | grep -e '^tar$$' -e '^tar.xz$$' -e '^tar.zst$$' -e '^tar.bz2$$' || echo tar.bz2` ; \
				echo "### $(par_dir)/target/package/$(distdir).$$tarext ###" ; \
				rm -f $(par_dir)/target/package/$(distdir).$$tarext ; \
				(cd $(par_dir)/target/package ; tar --posix -h --exclude-from exclude.lst -caf $(distdir).$$tarext $(distdir)) ;; \
		esac \
	done
	-@rm -r $(par_dir)/target/package/$(distdir)
doc: ## generate documentation [OPTS="??"]
	-cargo doc --no-deps $(PROFILEFLAGS) $(OPTS) -p $(proj)
lint: ## lint check [OPTS="??"]
	-rustfmt --check --edition=2021 $(OPTS) src/lib.rs
report: ## report code coverage
#	# read coverage data w/ [llvm-cov] gcov -f -b -n *.gcda
#	find . -type f -name '*.gcda' -exec llvm-cov gcov -f -b --no-output {} \;
#	# read coverage data w/ lcov -c -d . -o .coverage ... *.gcda
	-mkdir -p build/cov
	-lcov --capture -d . -o build/.coverage --gcov-tool ./llvm-gcov.sh
	-genhtml -o build/cov build/.coverage
