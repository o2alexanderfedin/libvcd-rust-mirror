use super::*;
use crate::{exit, free, printf, strtol};
use crate::src::vcd::{
    vcd_get_signal_by_name, vcd_read_from_path,
    vcd_signal_get_value_at_timestamp,
};
use crate::src::vcd_h::{SignalT, TimestampT, VcdT};

pub(crate) extern "C" fn print_vcd(vcd: &mut VcdT) -> () {
    unsafe {
        printf(c"{\n\tdate=\"%s\",\n\tversion=\"%s\",\n\ttimescale= {\n\t\tunit=\"%s\",\n\t\tscale=\"%zu\"\n\t},\n\tsignal= {\n".as_ptr()
                    as *mut i8 as *const i8,
            &raw mut (*vcd).date[0 as usize] as *mut i8,
            &raw mut (*vcd).version[0 as usize] as *mut i8,
            &raw mut (*vcd).timescale.unit[0 as usize] as *mut i8,
            (*vcd).timescale.scale)
    };
    {
        let mut i: i32 = 0;
        '__b4: loop {
            if !((i as u64) < (*vcd).signals_count) { break '__b4; }
            '__c4: loop {
                unsafe {
                    printf(c"\t\t%s= {\n\t\t\tsize=%zu,\n\t\t\tchanges= {\n".as_ptr()
                                as *mut i8 as *const i8,
                        &raw mut (*vcd).signals[i as usize].name[0 as usize] as
                            *mut i8, (*vcd).signals[i as usize].size)
                };
                {
                    let mut j: i32 = 0;
                    '__b5: loop {
                        if !((j as u64) < (*vcd).signals[i as usize].changes_count)
                            {
                            break '__b5;
                        }
                        '__c5: loop {
                            unsafe {
                                printf(c"\t\t\t\t{\n\t\t\t\t\ttimestamp=%u,\n\t\t\t\t\tvalue=%s\n\t\t\t\t},\n".as_ptr()
                                            as *mut i8 as *const i8,
                                    (*vcd).signals[i as
                                                        usize].value_changes[j as usize].timestamp,
                                    &raw mut (*vcd).signals[i as
                                                                    usize].value_changes[j as usize].value[0 as usize] as
                                        *mut i8)
                            };
                            break '__c5;
                        }
                        { let __p = &mut j; *__p += 1; *__p };
                    }
                }
                unsafe {
                    printf(c"\t\t\t},\n\t\t},\n".as_ptr() as *mut i8 as
                            *const i8)
                };
                break '__c4;
            }
            { let __p = &mut i; *__p += 1; *__p };
        }
    }
    unsafe { printf(c"\t}\n}\n".as_ptr() as *mut i8 as *const i8) };
}

pub(crate) extern "C" fn __main_inner(argc: i32, argv: *const *mut i8)
    -> Result<(), i32> {
    if argc % 2 == 1 {
        eprintln!("Usage: test <vcd-file> [signal-name timestamp] ...");
        unsafe { exit(1) };
    }
    let vcd: *mut VcdT =
        vcd_read_from_path(unsafe { *argv.offset(1 as isize) });
    if vcd as *mut () == 0 as *mut () {
        eprintln!("Could not read the VCD");
        return Err(1);
    }
    unsafe { printf(c"\n".as_ptr() as *mut i8 as *const i8) };
    {
        let mut i: i32 = 2;
        '__b6: loop {
            if !(i < argc) { break '__b6; }
            '__c6: loop {
                let signal_name: *const i8 =
                    unsafe { *argv.offset(i as isize) } as *const i8;
                let signal: *mut SignalT =
                    vcd_get_signal_by_name(unsafe { &mut *vcd },
                        signal_name as *const i8);
                let timestamp: TimestampT =
                    unsafe {
                            strtol(unsafe { *argv.offset((i + 1) as isize) } as
                                    *const i8, 0 as *mut () as *mut *mut i8, 0)
                        } as TimestampT;
                unsafe {
                    printf(c"%s at %u equals %s\n".as_ptr() as *mut i8 as
                            *const i8, unsafe { *argv.offset(i as isize) }, timestamp,
                        vcd_signal_get_value_at_timestamp(unsafe { &mut *signal },
                            timestamp))
                };
                break '__c6;
            }
            i += 2;
        }
    }
    unsafe { free(vcd as *mut ()) };
    return Ok(());
}
