use std::io;
use argparse::{ArgumentParser, Store};

fn main() {

    let mut ns_delay = 0;
    let mut s_delay = 0;
    {
        let mut parser = ArgumentParser::new();

        parser.refer(&mut ns_delay)
            .add_option(&["-n", "--nanoseconds"], Store,
                "Delay between printed lines in nanoseconds.");

        parser.refer(&mut s_delay)
            .add_option(&["-s", "--seconds"], Store,
                "Delay between printed lines in seconds.");
        
        parser.parse_args_or_exit();
    }
    
    // TODO boundary check for s_delay
    if ns_delay > 999999999 {
        println!("Too big nanoseconds delay. Maximum is 999999999.");
        std::process::exit(1);
    }

    // TODO przeliczyc s_delay i ns_delay
    
    let rqtp = libc::timespec {
        tv_sec: s_delay,
        tv_nsec: ns_delay,
    };

    loop {
        let mut line = String::new();

        io::stdin().read_line(&mut line)
            .expect("failed to read line");
        
        // TODO check err
        let _err = unsafe { 
            libc::nanosleep(&rqtp, std::ptr::null_mut()); 
        };

        print!("{}", &line);

    }
}
