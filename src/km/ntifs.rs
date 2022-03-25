use crate::shared::ntdef::*;

/// Device object flags.
#[repr(C)]
pub enum DEVICE_FLAGS {
    NONE = 0,
    DO_VERIFY_VOLUME = 0x00000002,
    DO_BUFFERED_IO = 0x00000004,
    DO_EXCLUSIVE = 0x00000008,
    DO_DIRECT_IO = 0x00000010,
    DO_MAP_IO_BUFFER = 0x00000020,
    DO_DEVICE_HAS_NAME = 0x00000040,
    DO_DEVICE_INITIALIZING = 0x00000080,
    DO_SYSTEM_BOOT_PARTITION = 0x00000100,
    DO_LONG_TERM_REQUESTS = 0x00000200,
    DO_NEVER_LAST_DEVICE = 0x00000400,
    DO_SHUTDOWN_REGISTERED = 0x00000800,
    DO_BUS_ENUMERATED_DEVICE = 0x00001000,
    DO_POWER_PAGABLE = 0x00002000,
    DO_POWER_INRUSH = 0x00004000,
    DO_POWER_NOOP = 0x00008000,
    DO_LOW_PRIORITY_FILESYSTEM = 0x00010000,
    DO_XIP = 0x00020000,
}

#[link(name = "ntoskrnl")]
extern "system" {
    pub fn RtlUTF8ToUnicodeN(
        UnicodeStringDestination: PWSTR,
        UnicodeStringMaxByteCount: ULONG,
        UnicodeStringActualByteCount: PULONG,
        UTF8StringSource: PCCH,
        UTF8StringByteCount: ULONG,
    ) -> NTSTATUS;
    pub fn RtlUnicodeToUTF8N(
        UTF8StringDestination: PCHAR,
        UTF8StringMaxByteCount: ULONG,
        UTF8StringActualByteCount: PULONG,
        UnicodeStringSource: PCWCH,
        UnicodeStringByteCount: ULONG,
    ) -> NTSTATUS;
}
