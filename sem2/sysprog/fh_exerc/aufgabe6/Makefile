PCKGS = lib bin

.PHONY: all clean install uninstall

all clean:
	for p in $(PCKGS) ; do \
		(cd $$p && $(MAKE) $@) ; \
	done