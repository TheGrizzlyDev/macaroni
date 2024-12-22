use libc_interposition_lib::InterposeEntry;

mod cwd;

#[used]
#[link_section = "__DATA,__interpose"]
static INTERPOSE_TABLE: [InterposeEntry; 1] = [
    cwd::getcwd::INTERPOSE_ENTRY,
];