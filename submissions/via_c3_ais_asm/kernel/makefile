run:
	cp target/viac3-unknown-none/debug/kernel isofiles/boot/kernel.elf
	grub-mkrescue -o img.iso isofiles
	qemu-system-i386 -nographic -cdrom img.iso -no-reboot -no-shutdown -d cpu_reset
