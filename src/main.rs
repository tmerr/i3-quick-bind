extern crate clap;
extern crate colored;
extern crate i3ipc;

use colored::Colorize;
use std::process::{Command, Child};
use std::os::unix::process::ExitStatusExt;
use std::io::{self, Write};
use clap::{Arg, App};
use i3ipc::I3EventListener;
use i3ipc::Subscription;
use i3ipc::event::{BindingEventInfo, Event};
use i3ipc::event::inner::{Binding, InputType, BindingChange};

fn print_waiting(sharg: &str, mods: &[&str], keysym: &str) {
    let joinedmods: String;
    let shownmods = if mods.len() == 0 {
        "AnyMod"
    } else {
        joinedmods = mods.join("+");
        &joinedmods
    };

    let formatted = format!(r#"<waiting on {}+{} to run sh -c "{}">"#,
                            &shownmods, keysym, sharg);
    print!("{}", formatted.green());
    let mut stdout = io::stdout();
    stdout.flush()
          .expect("unexpected failure to flush stdout");
}

fn print_warning(text: &str) {
    let bracketed = format!("<{}>", text);
    println!("{}", bracketed.yellow());
}

fn wait_on_child_process(mut child: Child) {
    match child.wait() {
        Ok(exit_status) => {
            if exit_status.success() {
                return;
            }

            if let Some(code) = exit_status.code() {
                print_warning(&format!("command exited with non-zero exit code {}", code));
            } else {
                print_warning(&format!("command was terminated by signal {}",
                                       exit_status.signal()
                                                  .map(|x| x.to_string())
                                                  .unwrap_or("?".to_string())));
            }
        }
        Err(_) => print_warning("couldn't wait on child process, weird!"),
    }
}

fn event_loop(mods: &[&str], keysym: &str, sharg: &str) {
    let mut listener = I3EventListener::connect().expect("unexpected error connecting to i3 socket");
    let subs = [Subscription::Binding];
    listener.subscribe(&subs).expect("unexpected error when subscribing to i3 events");
    print_waiting(&sharg, mods, keysym);
    for event in listener.listen() {
        match event.expect("unexpected error when receiving i3 event") {
            Event::BindingEvent(BindingEventInfo {
                change: BindingChange::Run,
                binding: Binding {
                    command,
                    event_state_mask,
                    input_type: InputType::Keyboard,
                    input_code: 0,
                    symbol: Some(matchsym),
                },
            }) => {
                let mut cloned_mods = event_state_mask.clone();
                cloned_mods.sort();
                if mods.len() == 0 || mods == &cloned_mods[..] {
                    if command == "nop" && &matchsym == keysym {
                        println!("");
                        if let Ok(child) = Command::new("sh")
                                                      .arg("-c")
                                                      .arg(&sharg)
                                                      .spawn() {
                            wait_on_child_process(child);
                        } else {
                            print_warning(&format!("failed to spawn sh for {}", sharg));
                        }
                        let mut stdout = io::stdout();
                        stdout.flush()
                              .expect("unexpected failure to flush stdout");
                        print_waiting(&sharg, mods, keysym);
                    }
                }
            },
            _ => unreachable!()
        }
    }
}

fn main() {
    let matches = App::new("i3-quick-bind")
                          .version("0.1")
                          .author("Trevor Merrifield <trevorm42@gmail.com>")
                          .about("Temporarily bind a shell command to a hotkey")
                          .arg(Arg::with_name("key")
                               .help("a keysym or <modifiers+>keysym")
                               .takes_value(true)
                               .required(true))
                          .arg(Arg::with_name("command")
                               .help("the text to be sent through to the shell")
                               .required(true)
                               .multiple(true))
                          .get_matches();

    // this unwrap is OK because "key" is required
    let unparsed_key = matches.value_of("key").unwrap();
    let mut keyvec: Vec<&str> = unparsed_key.split("+").collect();
    keyvec.sort();

    let mods = &keyvec[0..keyvec.len()-1];
    let keysym = &keyvec[keyvec.len() - 1];

    // this unwrap is OK because "command" is required
    let tokens: Vec<_> = matches.values_of("command").unwrap().collect();

    let sharg = tokens.join(" ");

    event_loop(mods, keysym, &sharg);
}
