use zsh_module::{Builtin, MaybeError, Module, ModuleBuilder, Opts};

// Export the timer module
zsh_module::export_module!(timer, setup);

struct Timer;

impl Timer {
    fn timer_cmd(&mut self, _name: &str, args: &[&str], _opts: Opts) -> MaybeError {
        // Check for correct number of arguments
        if args.len() != 2 {
            eprintln!("usage: _timer <start-epoch> <end-epoch>");
            return Ok(());
        }

        // Parse timestamps
        let t0: f64 = args[0].parse().map_err(|_| "Invalid start-epoch")?;
        let t1: f64 = args[1].parse().map_err(|_| "Invalid end-epoch")?;

        // Calculate time difference in nanoseconds
        let total_ns = ((t1 - t0) * 1_000_000_000.0) as i64;
        let mut ns = total_ns;
        let sec = ns / 1_000_000_000;
        ns %= 1_000_000_000;

        // Time unit constants
        const Y: i64 = 31_536_000;  // seconds in a year
        const W: i64 = 604_800;     // seconds in a week
        const D: i64 = 86_400;      // seconds in a day
        const H: i64 = 3_600;       // seconds in an hour
        const M: i64 = 60;          // seconds in a minute

        // Break down time units
        let mut remaining_sec = sec;

        let y = remaining_sec / Y;
        remaining_sec %= Y;

        let w = remaining_sec / W;
        remaining_sec %= W;

        let d = remaining_sec / D;
        remaining_sec %= D;

        let h = remaining_sec / H;
        remaining_sec %= H;

        let m = remaining_sec / M;
        remaining_sec %= M;

        let s = remaining_sec;
        let ms = ns / 1_000_000;
        ns %= 1_000_000;
        let us = ns / 1_000;
        ns %= 1_000;

        // Build formatted output
        let mut output = String::new();
        let mut has_larger_component = false;

        if y > 0 {
            output.push_str(&format!("{:02}y ", y));
            has_larger_component = true;
        }
        if w > 0 {
            output.push_str(&format!("{:02}w ", w));
            has_larger_component = true;
        }
        if d > 0 {
            output.push_str(&format!("{:02}d ", d));
            has_larger_component = true;
        }
        if h > 0 {
            output.push_str(&format!("{:02}h ", h));
            has_larger_component = true;
        }
        if m > 0 {
            output.push_str(&format!("{:02}m ", m));
            has_larger_component = true;
        }

        // Only show seconds if non-zero or if larger components are present
        if s > 0 || has_larger_component {
            output.push_str(&format!("{:02}s ", s));
            has_larger_component = true;
        }

        // Only show milliseconds if non-zero or if larger components are present
        if ms > 0 || has_larger_component {
            output.push_str(&format!("{:03}ms ", ms));
            has_larger_component = true;
        }

        // Only show microseconds if non-zero or if larger components are present
        if us > 0 || has_larger_component {
            output.push_str(&format!("{:03}μs ", us));
            has_larger_component = true;
        }

        // Only show nanoseconds if non-zero or if larger components are present
        if ns > 0 || has_larger_component {
            output.push_str(&format!("{:03}ns", ns));
        }

        println!("{}", output);
        Ok(())
    }
}

fn setup() -> Result<Module, Box<dyn std::error::Error>> {
    let module = ModuleBuilder::new(Timer)
        .builtin(Timer::timer_cmd, Builtin::new("_timer"))
        .build();
    Ok(module)
}