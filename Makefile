# Multi-package project Makefile script.
.POSIX:
help:

#MAKE = make # (GNU make variants: make (Linux) gmake (FreeBSD)

parent = intrors
version = 0.1.0
SUBDIRS = $(parent)-util $(parent)-foreignc $(parent)-practice $(parent)-intro

.PHONY: configure build testCompile help clean test
configure: $(SUBDIRS) ## configure [OPTS=""]
	-for dirX in $^ ; do mkdir -p $$dirX ; \
		(cd $$dirX ; sh ./configure.sh $(OPTS)) ; done
help: $(SUBDIRS)
	-for dirX in $^ ; do $(MAKE) -C $$dirX $@ ; done
	@echo "##### Top-level multiproject: $(parent) #####"
	@echo "       $(MAKE) [SUBDIRS="$(SUBDIRS)"] configure [OPTS=??]"
	@echo "Usage: $(MAKE) [SUBDIRS="$(SUBDIRS)"] [target]"
build testCompile test: $(SUBDIRS)
	-for dirX in $^ ; do $(MAKE) -C $$dirX $@ ; done
clean: $(SUBDIRS)
	-for dirX in $^ ; do $(MAKE) -C $$dirX $@ ; done
	-rm -fr core* *~ .*~ build/* *.log */*.log

#----------------------------------------
FMTS ?= tar.gz,zip
distdir = $(parent)-$(version)


build/$(distdir) : 
	-@mkdir -p build/$(distdir) ; cp -f exclude.lst build/
#	#-zip -9 -q --exclude @exclude.lst -r - . | unzip -od build/$(distdir) -
	-tar --format=posix --dereference --exclude-from=exclude.lst -cf - . | tar -xpf - -C build/$(distdir)

.PHONY: dist doc report uninstall install run debug valgrind
dist | build/$(distdir): $(SUBDIRS)

	-@for fmt in `echo $(FMTS) | tr ',' ' '` ; do \
		case $$fmt in \
			7z) echo "### build/$(distdir).7z ###" ; \
				rm -f build/$(distdir).7z ; \
				(cd build ; 7za a -t7z -mx=9 $(distdir).7z $(distdir)) ;; \
			zip) echo "### build/$(distdir).zip ###" ; \
				rm -f build/$(distdir).zip ; \
				(cd build ; zip -9 -q -r $(distdir).zip $(distdir)) ;; \
			*) tarext=`echo $$fmt | grep -e '^tar$$' -e '^tar.xz$$' -e '^tar.zst$$' -e '^tar.bz2$$' || echo tar.gz` ; \
				echo "### build/$(distdir).$$tarext ###" ; \
				rm -f build/$(distdir).$$tarext ; \
				(cd build ; tar --posix -h -caf $(distdir).$$tarext $(distdir)) ;; \
		esac \
	done
	-@rm -r build/$(distdir)
	-for dirX in $^ ; do $(MAKE) -C $$dirX $@ ; done
doc lint report: $(SUBDIRS)
	-for dirX in $^ ; do $(MAKE) -C $$dirX $@ ; done
uninstall install run debug valgrind: $(parent)-intro
	-$(MAKE) -C $(parent)-intro $@
