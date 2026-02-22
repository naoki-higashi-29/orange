set dotenv-load
target  := "x86_64-unknown-uefi"
package := "bootloader"
profile := "debug"
esp     := "fs"
ovmf    := env('OVMF')

efi_src := "target/" + target + "/" + profile + "/" + package + ".efi"
efi_dst := esp + "/EFI/BOOT/BOOTX64.EFI"

build:
    cargo build -p {{package}} --target {{target}}

run: build
    cp {{efi_src}} {{efi_dst}}
    qemu-system-x86_64 \
        -drive if=pflash,format=raw,readonly=on,file={{ovmf}} \
        -drive format=raw,file=fat:rw:{{esp}} \
        -net none
