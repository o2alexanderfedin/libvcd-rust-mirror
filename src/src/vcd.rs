use super::*;
use crate::src::vcd_h::{SignalT, TimestampT, ValueChangeT, VcdT, VCD_SIGNAL_COUNT};
use crate::{
    __builtin___strncpy_chk, __builtin_object_size, calloc, fclose, fgetc, fopen, free, fscanf,
    isspace, sscanf, strchr, strcmp, strlen, ungetc, FILE,
};

extern "C" fn new_vcd() -> *mut VcdT {
    return unsafe { calloc(1 as u64, core::mem::size_of::<VcdT>() as u64) } as *mut VcdT;
}

pub(crate) const BEFORE_MODULE_DEFINITIONS: u32 = 0;

pub(crate) const INSIDE_TOP_MODULE: u32 = 1;

pub(crate) const INSIDE_INNER_MODULES: u32 = 2;

pub(crate) type StateT = u32;

extern "C" fn get_signal_index(string: &i8) -> i32 {
    let id: i32 = *string - '!' as i32;
    if id >= VCD_SIGNAL_COUNT {
        return -1;
    }
    return id;
}

pub(crate) extern "C" fn parse_instruction(
    file: *mut FILE,
    vcd: &mut VcdT,
    state: &mut StateT,
) -> bool {
    let mut instruction: [i8; 512] = [0; 512];
    if unsafe {
        fscanf(
            file,
            c"%s".as_ptr() as *mut i8 as *const i8,
            &raw mut instruction[0 as usize] as *mut i8,
        )
    } != 1
    {
        return 0;
    }
    if unsafe {
        strcmp(
            &raw mut instruction[0 as usize] as *mut i8 as *const i8,
            c"end".as_ptr() as *mut i8 as *const i8,
        )
    } == 0
        || unsafe {
            strcmp(
                &raw mut instruction[0 as usize] as *mut i8 as *const i8,
                c"dumpvars".as_ptr() as *mut i8 as *const i8,
            )
        } == 0
        || unsafe {
            strcmp(
                &raw mut instruction[0 as usize] as *mut i8 as *const i8,
                c"dumpall".as_ptr() as *mut i8 as *const i8,
            )
        } == 0
    {
        return 1;
    }
    if unsafe {
        strcmp(
            &raw mut instruction[0 as usize] as *mut i8 as *const i8,
            c"scope".as_ptr() as *mut i8 as *const i8,
        )
    } == 0
    {
        '__s0: {
            match *state {
                BEFORE_MODULE_DEFINITIONS => {
                    *state = INSIDE_TOP_MODULE;
                }
                INSIDE_TOP_MODULE => {
                    *state = INSIDE_INNER_MODULES;
                }
                _ => {}
            }
        }
        unsafe { fscanf(file, c"\n%*[^$]".as_ptr() as *mut i8 as *const i8) };
        return 1;
    }
    if unsafe {
        strcmp(
            &raw mut instruction[0 as usize] as *mut i8 as *const i8,
            c"scope".as_ptr() as *mut i8 as *const i8,
        )
    } == 0
        || unsafe {
            strcmp(
                &raw mut instruction[0 as usize] as *mut i8 as *const i8,
                c"upscope".as_ptr() as *mut i8 as *const i8,
            )
        } == 0
        || unsafe {
            strcmp(
                &raw mut instruction[0 as usize] as *mut i8 as *const i8,
                c"enddefinitions".as_ptr() as *mut i8 as *const i8,
            )
        } == 0
        || unsafe {
            strcmp(
                &raw mut instruction[0 as usize] as *mut i8 as *const i8,
                c"comment".as_ptr() as *mut i8 as *const i8,
            )
        } == 0
    {
        unsafe { fscanf(file, c"\n%*[^$]".as_ptr() as *mut i8 as *const i8) };
        return 1;
    }
    if unsafe {
        strcmp(
            &raw mut instruction[0 as usize] as *mut i8 as *const i8,
            c"var".as_ptr() as *mut i8 as *const i8,
        )
    } == 0
    {
        if *state == INSIDE_INNER_MODULES {
            unsafe { fscanf(file, c" %*[^\n]\n".as_ptr() as *mut i8 as *const i8) };
            return 1;
        }
        let signal: *mut SignalT = &mut (*vcd).signals[(*vcd).signals_count as usize];
        (*vcd).signals_count = (*vcd).signals_count.wrapping_add(1 as u64);
        let mut signal_id: [i8; 32] = [0; 32];
        unsafe {
            fscanf(
                file,
                c" %*s %zu %[^ ] %[^ $]%*[^$]".as_ptr() as *mut i8 as *const i8,
                unsafe { &raw mut (*signal).size } as *mut u64,
                &raw mut signal_id[0 as usize] as *mut i8,
                unsafe { &raw mut (*signal).name[0 as usize] } as *mut i8,
            )
        };
        let index: i32 = get_signal_index(&(&raw mut signal_id[0 as usize] as *mut i8));
        if (*vcd).signals[index as usize].size != 0 as u64 {
            return 1;
        }
        return 1;
    }
    if unsafe {
        strcmp(
            &raw mut instruction[0 as usize] as *mut i8 as *const i8,
            c"date".as_ptr() as *mut i8 as *const i8,
        )
    } == 0
    {
        unsafe {
            fscanf(
                file,
                c"\n%[^$\n]".as_ptr() as *mut i8 as *const i8,
                &raw mut (*vcd).date[0 as usize] as *mut i8,
            )
        };
        return 1;
    }
    if unsafe {
        strcmp(
            &raw mut instruction[0 as usize] as *mut i8 as *const i8,
            c"version".as_ptr() as *mut i8 as *const i8,
        )
    } == 0
    {
        unsafe {
            fscanf(
                file,
                c"\n%[^$\n]".as_ptr() as *mut i8 as *const i8,
                &raw mut (*vcd).version[0 as usize] as *mut i8,
            )
        };
        return 1;
    }
    if unsafe {
        strcmp(
            &raw mut instruction[0 as usize] as *mut i8 as *const i8,
            c"timescale".as_ptr() as *mut i8 as *const i8,
        )
    } == 0
    {
        unsafe {
            fscanf(
                file,
                c"\n\t%zu%[^$\n]".as_ptr() as *mut i8 as *const i8,
                &raw mut (*vcd).timescale.scale as *mut u64,
                &raw mut (*vcd).timescale.unit[0 as usize] as *mut i8,
            )
        };
        return 1;
    }
    return 0;
}

pub(crate) extern "C" fn parse_timestamp(file: *mut FILE, timestamp: *mut TimestampT) -> bool {
    let successful: bool =
        unsafe { fscanf(file, c"%u".as_ptr() as *mut i8 as *const i8, timestamp) } == 1;
    return successful;
}

pub(crate) extern "C" fn parse_assignment(
    file: *mut FILE,
    vcd: &mut VcdT,
    timestamp: TimestampT,
) -> bool {
    let mut buffer: [i8; 512] = [0; 512];
    let mut value: [i8; 64] = [0; 64];
    let mut signal_id: [i8; 32] = [0; 32];
    unsafe {
        fscanf(
            file,
            c"%[^\n]".as_ptr() as *mut i8 as *const i8,
            &raw mut buffer[0 as usize] as *mut i8,
        )
    };
    let is_vector: bool = unsafe {
        strchr(
            c"01xXzZ".as_ptr() as *mut i8 as *const i8,
            buffer[0 as usize] as i32,
        )
    } as *mut ()
        == 0 as *mut ();
    let assignment_format_string: *const i8 = if is_vector {
        c"%[^ ] %[^\n]".as_ptr() as *mut i8
    } else {
        c"%1s%[^\n]".as_ptr() as *mut i8
    } as *const i8;
    if unsafe {
        sscanf(
            &raw mut buffer[0 as usize] as *mut i8 as *const i8,
            assignment_format_string as *const i8,
            &raw mut value[0 as usize] as *mut i8,
            &raw mut signal_id[0 as usize] as *mut i8,
        )
    } != 2
    {
        return 0;
    }
    if unsafe { strlen(&raw mut signal_id[0 as usize] as *mut i8 as *const i8) } > 1 as u64 {
        return 1;
    }
    let index: u64 = get_signal_index(&(&raw mut signal_id[0 as usize] as *mut i8)) as u64;
    if index == -1i32 as u64 || index >= (*vcd).signals_count {
        return 1;
    }
    let changes_count: u64 = (*vcd).signals[index as usize].changes_count;
    (*vcd).signals[index as usize].value_changes[changes_count as usize].timestamp = timestamp;
    unsafe {
        __builtin___strncpy_chk(
            &raw mut (*vcd).signals[index as usize].value_changes[changes_count as usize].value
                [0 as usize] as *mut i8,
            &raw mut value[0 as usize] as *mut i8 as *const i8,
            64 as u64,
            unsafe {
                __builtin_object_size(
                    &raw mut (*vcd).signals[index as usize].value_changes[changes_count as usize]
                        .value[0 as usize] as *mut i8 as *const (),
                    if 2 > 1 { 1 } else { 0 },
                )
            },
        )
    };
    (*vcd).signals[index as usize].changes_count = (*vcd).signals[index as usize]
        .changes_count
        .wrapping_add(1 as u64);
    return 1;
}

pub(crate) extern "C" fn vcd_read_from_path(path: *mut i8) -> *mut VcdT {
    let file: *mut FILE =
        unsafe { fopen(path as *const i8, c"r".as_ptr() as *mut i8 as *const i8) };
    if file as *mut () == 0 as *mut () {
        return 0 as *mut () as *mut VcdT;
    }
    let vcd: *mut VcdT = new_vcd();
    let mut current_timestamp: TimestampT = 0 as TimestampT;
    let mut state: StateT = BEFORE_MODULE_DEFINITIONS;
    let mut character: i32 = 0;
    while {
        character = unsafe { fgetc(file) };
        character
    } != -1
    {
        if character == '$' as i32 {
            let successful: bool =
                unsafe { parse_instruction(file, unsafe { &mut *vcd }, &mut state) };
            if successful {
                continue;
            }
        } else if character == '#' as i32 {
            let successful: bool = unsafe { parse_timestamp(file, &mut current_timestamp) };
            if successful {
                continue;
            }
        } else if !(unsafe {
            strchr(
                c"-0123456789zZxXbU".as_ptr() as *mut i8 as *const i8,
                character,
            )
        })
        .is_null()
        {
            unsafe { ungetc(character, file) };
            let successful: bool =
                unsafe { parse_assignment(file, unsafe { &mut *vcd }, current_timestamp) };
            if successful {
                continue;
            }
        } else if unsafe { isspace(character) } != 0 {
            continue;
        }
        unsafe { fclose(file) };
        unsafe { free(vcd as *mut ()) };
        return 0 as *mut () as *mut VcdT;
    }
    unsafe { fclose(file) };
    return vcd;
}

pub(crate) extern "C" fn vcd_get_signal_by_name(
    vcd: &mut VcdT,
    signal_name: *const i8,
) -> *mut SignalT {
    {
        let mut i: i32 = 0;
        '__b2: loop {
            if !((i as u64) < (*vcd).signals_count) {
                break '__b2;
            }
            '__c2: loop {
                if unsafe {
                    strcmp(
                        &raw mut (*vcd).signals[i as usize].name[0 as usize] as *mut i8
                            as *const i8,
                        signal_name,
                    )
                } == 0
                {
                    return &mut (*vcd).signals[i as usize];
                }
                break '__c2;
            }
            {
                let __p = &mut i;
                *__p += 1;
                *__p
            };
        }
    }
    return 0 as *mut () as *mut SignalT;
}

pub(crate) extern "C" fn vcd_signal_get_value_at_timestamp(
    signal: &mut SignalT,
    timestamp: TimestampT,
) -> *mut i8 {
    let mut previous_value: *mut i8 = 0 as *mut () as *mut i8;
    {
        let mut i: i32 = 0;
        '__b3: loop {
            if !((i as u64) < (*signal).changes_count) {
                break '__b3;
            }
            '__c3: loop {
                let value_change: &mut ValueChangeT = &mut (*signal).value_changes[i as usize];
                if timestamp < (*value_change).timestamp {
                    break '__b3;
                }
                previous_value = &raw mut (*value_change).value[0 as usize] as *mut i8;
                break '__c3;
            }
            {
                let __p = &mut i;
                *__p += 1;
                *__p
            };
        }
    }
    return previous_value;
}
