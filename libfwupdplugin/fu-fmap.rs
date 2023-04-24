#[derive(New, Validate, Parse)]
struct Fmap {
    signature: 8s: const=__FMAP__
    ver_major: u8: default=0x1
    ver_minor: u8: default=0x1
    base: u64le			// address of the firmware binary
    size: u32le			// bytes
    name: 32s
    nareas: u16le		// number of areas
}
#[derive(New, Validate, Parse)]
struct FmapArea {		// area of volatile and static regions
    offset: u32le		// offset relative to base
    size: u32le			// bytes
    name: 32s			// descriptive name
    flags: u16le
}