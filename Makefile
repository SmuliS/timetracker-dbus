NAME = timetracker-dbus
COMMIT = $(shell git rev-parse --short HEAD)
REMOTE_TARBALL = https://github.com/smulis/${NAME}/tarball/$(COMMIT)

DEFAULT:
	echo "No default target defined"

/tmp/${NAME}-$(COMMIT).tar.gz:
	wget $(REMOTE_TARBALL) -O $@

~/rpmbuild/SOURCES/${NAME}-$(COMMIT).tar.gz: /tmp/${NAME}-$(COMMIT).tar.gz
	# Remove GitHub username
	tar -zxvf $^ --transform='s/SmuliS-//' -C /tmp
	tar -czvf $@ -C /tmp ${NAME}-$(COMMIT)/

rpm: ~/rpmbuild/SOURCES/${NAME}-$(COMMIT).tar.gz
	rpmbuild -ba pkg/${NAME}.spec --debug --define '_commit $(COMMIT)'


