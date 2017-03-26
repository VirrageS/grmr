.SILENT: all install clean

all: clean install

install:
	cp grmr /usr/local/bin/grmr
	mkdir -p /usr/local/share/grmr
	cp -a bin/. /usr/local/share/grmr/
	echo "Installing done"

clean:
	rm -f /usr/local/bin/grmr
	rm -rf /usr/local/share/grmr
	echo "Cleaning done"

.PHONY: all
