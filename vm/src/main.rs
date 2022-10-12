mod system_call_names;

use linux_personality::personality;
use nix::sys::ptrace;
use nix::sys::wait::wait;
use nix::unistd::{fork, ForkResult, Pid};
use reqwest;
use std::env;
use std::os::unix::process::CommandExt;
use std::process::{exit, Command};

use libc::user_regs_struct;

fn handle_syscall(child: Pid, regs: &user_regs_struct) {
    println!(
        "Handle: {:?}",
        system_call_names::SYSTEM_CALL_NAMES[(regs.orig_rax) as usize]
    );
    ptrace::step(child, None).unwrap();
    wait().unwrap();

    if regs.orig_rax == 2 {
        let resp = reqwest::blocking::get("http://localhost:8081/open")
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
