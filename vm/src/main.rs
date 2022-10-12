mod system_call_names;

use byteorder::{LittleEndian, WriteBytesExt};
use linux_personality::personality;
use nix::sys::ptrace::{self, AddressType};
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, Pid};
use reqwest;
use std::env;

use std::os::unix::process::CommandExt;
use std::process::{exit, Command};

use libc::{c_long, c_void, user_regs_struct};

fn read_string(pid: Pid, address: AddressType) -> String {
    let mut string = String::new();
    // Move 8 bytes up each time for next read.
    let mut count = 0;
    let word_size = 8;

    'done: loop {
        let mut bytes: Vec<u8> = vec![];
        let address = unsafe { address.offset(count) };

        let res: c_long = ptrace::read(pid, address).unwrap_or_else(|err| {
            panic!("Failed to read data for pid {}: {}", pid, err);
        });
        bytes.write_i64::<LittleEndian>(res).unwrap_or_else(|err| {
            panic!("Failed to write {} as i64 LittleEndian: {}", res, err);
        });

        for b in bytes {
            if b != 0 {
                string.push(b as char);
            } else {
                break 'done;
            }
        }
        count += word_size;
    }

    string
}

fn handle_syscall(child: Pid, regs: &user_regs_struct) {
    println!(
        "Handle: {:?}",
        system_call_names::SYSTEM_CALL_NAMES[(regs.orig_rax) as usize]
    );
    ptrace::step(child, None).unwrap();
    wait().unwrap();

    if regs.orig_rax == 2 {
        let path = read_string(child, regs.rdi as *mut c_void);

        let client = reqwest::blocking::Client::new();
        let resp = client
            .post("http://localhost:8081/open")
            .json(&http_data::OpenRequest {
                path: path,
                oflag: regs.rsi,
                mode: regs.rdx,
            })
            .send()
            .unwrap()
            .json::<http_data::SysCallResp<http_data::OpenResp>>()
            .unwrap();

        if let Ok(mut new_regs) = ptrace::getregs(child) {
            new_regs.rax = resp.response.fd as u64;
            ptrace::setregs(child, new_regs).unwrap();
        }
    }
}

fn run_tracer(child: Pid) -> Result<(), nix::errno::Errno> {
    // Handle the initial execve
    wait().unwrap();

    loop {
        // Syscall will error out when the program finnishes
        // TODO: better error handling
        if let Err(_) = ptrace::syscall(child, None) {
            return Ok(());
        }

        wait()?;

        let regs = ptrace::getregs(child)?;
        handle_syscall(child, &regs);
    }
}

fn run_tracee(command: &str) {
    ptrace::traceme().unwrap();
    personality(linux_personality::ADDR_NO_RANDOMIZE).unwrap();

    Command::new(command).exec();

    exit(0)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} <executable>", args[0]);
        return;
    }

    match unsafe { fork() } {
        Ok(ForkResult::Child) => {
            run_tracee(&args[1]);
        }

        Ok(ForkResult::Parent { child }) => {
            if let Err(e) = run_tracer(child) {
                println!("Tracer failed: '{:?}'", e);
            }
        }

        Err(err) => {
            panic!("[main] fork() failed: {}", err);
        }
    }
}
