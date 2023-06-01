INIT := $(shell find ./init -type f -print)
CONFIG := $(shell find ./config -type f -print)

.PHONY: clean-all clean-image clean-base clean-uboot run

void : aarch-linux-sd-image.qcow2

resources/aarch-linux-sd-image.img.zst :
	wget \
		-O resources/aarch-linux-sd-image.img.zst \
		https://hydra.nixos.org/job/nixos/release-22.11/nixos.sd_image.aarch64-linux/latest/download-by-type/file/sd-image

resources/uboot.bin :
	nix build nixpkgs#pkgsCross.aarch64-multiplatform.ubootQemuAarch64
	cp result/u-boot.bin resources/uboot.bin
	rm result
	chmod u+w uboot.bin

aarch-linux-sd-image.qcow2 : resources/uboot.bin resources/aarch-linux-sd-image.img.zst $(INIT) $(CONFIG)
	unzstd resources/aarch-linux-sd-image.img.zst
	chmod u+w resources/aarch-linux-sd-image.img
	$(eval TMP := $(shell mktemp -d))
	sudo mount \
	   --read-write \
	   --verbose \
	   --options loop,offset=39845888 \
	   resources/aarch-linux-sd-image.img \
	   '$(TMP)'
	sudo cp --recursive init/* "$(TMP)"
	sudo mkdir --parents "$(TMP)/etc/nixos"
	sudo cp --recursive config/* "$(TMP)/etc/nixos"
	sudo umount "$(TMP)"
	rmdir "$(TMP)"
	qemu-img convert resources/aarch-linux-sd-image.img -f raw -O qcow2 $@
	rm resources/aarch-linux-sd-image.img
	qemu-img resize $@ +35G
	./start-vm

clean-all: clean-image clean-uboot clean-base

clean-image:
	rm aarch-linux-sd-image.qcow2

clean-base:
	rm resources/aarch-linux-sd-image.img.zst

clean-uboot:
	rm resources/uboot.bin

