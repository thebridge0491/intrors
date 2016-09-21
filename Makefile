# Multi-package project Makefile script.
.POSIX:
help:

#MAKE = make # (GNU make variants: make (Linux) gmake (FreeBSD)

parent = intrors
SUBDIRS = $(parent)-util $(parent)-intro

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
FMTS ?= tar.gz
distdir = $(parent)-0.1.0

.PHONY: dist doc report uninstall install run debug valgrind
dist: $(SUBDIRS)
	-@mkdir -p build/$(distdir) ; cp -f exclude.lst build/
#	#-zip -9 -q --exclude @exclude.lst -r - . | unzip -od build/$(distdir) -
	-tar --format=posix --dereference --exclude-from=exclude.lst -cf - . | tar -xpf - -C build/$(distdir)
	
	-@for fmt in `echo $(FMTS) | tr ',' ' '` ; do \
		case $$fmt in \
			zip) echo "### build/$(distdir).zip ###" ; \
				rm -f build/$(distdir).zip ; \
				(cd build ; zip -9 -q -r $(distdir).zip $(distdir)) ;; \
			*) tarext=`echo $$fmt | grep -e '^tar$$' -e '^tar.xz$$' -e '^tar.bz2$$' || echo tar.gz` ; \
				echo "### build/$(distdir).$$tarext ###" ; \
				rm -f build/$(distdir).$$tarext ; \
				(cd build ; tar --posix -L -caf $(distdir).$$tarext $(distdir)) ;; \
		esac \
	done
	-@rm -r build/$(distdir)
	-for dirX in $^ ; do $(MAKE) -C $$dirX $@ ; done
doc report: $(SUBDIRS)
	-for dirX in $^ ; do $(MAKE) -C $$dirX $@ ; done
uninstall install run debug valgrind: $(parent)-intro
	-$(MAKE) -C $(parent)-intro $@
