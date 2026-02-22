#![no_std]
#![no_main]

extern crate alloc;

use alloc::format;
use uefi::fs::FileSystem;
use uefi::mem::memory_map::{MemoryMap, MemoryMapMut, MemoryType};
use uefi::prelude::*;
use uefi::proto::loaded_image::LoadedImage;
use uefi::proto::media::fs::SimpleFileSystem;

#[entry]
fn main() -> Status {
    // UEFIのブートサービス、アロケータ、ロガーなどの初期化
    uefi::helpers::init().unwrap();

    uefi::println!("Retrieving Memory Map...");

    // 1. メモリマップの取得
    let mut memory_map =
        uefi::boot::memory_map(MemoryType::LOADER_DATA).expect("Failed to retrieve memory map");

    // アドレス順にソート
    memory_map.sort();

    // 2. CSVデータの作成 (ヘッダー)
    let mut csv_content = alloc::string::String::from("Type,PhysicalStart,PhysicalEnd,PageCount\n");

    for descriptor in memory_map.entries() {
        let row = format!(
            "{:?},0x{:x},0x{:x},{}\n",
            descriptor.ty,
            descriptor.phys_start,
            descriptor.phys_start + (descriptor.page_count * 4096) - 1,
            descriptor.page_count
        );
        csv_content.push_str(&row);
    }

    // 3. ファイルシステムを介してファイルに書き込み
    // LoadedImage から起動デバイスのハンドルを取得し、SimpleFileSystem を開く
    uefi::println!("Saving to \\memmap.csv...");
    let image_handle = uefi::boot::image_handle();
    let loaded_image = uefi::boot::open_protocol_exclusive::<LoadedImage>(image_handle)
        .expect("Failed to open LoadedImage protocol");
    let device_handle = loaded_image.device().expect("Failed to get device handle");
    drop(loaded_image);
    let sfs = uefi::boot::open_protocol_exclusive::<SimpleFileSystem>(device_handle)
        .expect("Failed to open SimpleFileSystem protocol");
    let mut fs = FileSystem::new(sfs);

    // ルートディレクトリの下に memmap.csv を作成 (既存なら上書き)
    match fs.write(cstr16!("\\memmap.csv"), csv_content.as_bytes()) {
        Ok(_) => uefi::println!("Successfully saved to \\memmap.csv"),
        Err(e) => uefi::println!("Failed to write file: {:?}", e),
    }

    uefi::println!("\nDone. You can find memmap.csv in the root of the EFI partition.");

    loop {
        core::hint::spin_loop();
    }
}
