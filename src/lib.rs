use zsh_module::{Builtin, MaybeError, Module, ModuleBuilder, Opts};

zsh_module::export_module!(timer, setup);

struct Timer;

impl Timer {
    fn timer_cmd(&mut self, _name: &str, args: &[&str], _opts: Opts) -> MaybeError {
        if args.len() != 2 {
            eprintln!("usage: _timer <start-epoch> <end-epoch>");
            return Ok(());
        }
        let t0: f64 = args[0].parse().map_err(|_| "Invalid start-epoch")?;
        let t1: f64 = args[1].parse().map_err(|_| "Invalid end-epoch")?;
        let mut ns = ((t1 - t0) * 1_000_000_000.0) as i64;
        let sec = ns / 1_000_000_000;
        ns %= 1_000_000_000;
        const Y: i64 = 31_536_000; const W: i64 = 604_800; const D: i64 = 86_400; const H: i64 = 3_600; const M: i64 = 60;
        let mut rs = sec;
        let y = rs / Y; rs %= Y;
        let w = rs / W; rs %= W;
        let d = rs / D; rs %= D;
        let h = rs / H; rs %= H;
        let m = rs / M; rs %= M;
        let s = rs;
        let ms = ns / 1_000_000; ns %= 1_000_000;
        let us = ns / 1_000; ns %= 1_000;
        let mut o = String::new();
        let mut l = false;
        if y > 0 { o.push_str(&format!("{:02}y ", y)); l = true; }
        if w > 0 { o.push_str(&format!("{:02}w ", w)); l = true; }
        if d > 0 { o.push_str(&format!("{:02}d ", d)); l = true; }
        if h > 0 { o.push_str(&format!("{:02}h ", h)); l = true; }
        if m > 0 { o.push_str(&format!("{:02}m ", m)); l = true; }
        if s > 0 || l { o.push_str(&format!("{:02}s ", s)); l = true; }
        if ms > 0 || l { o.push_str(&format!("{:03}ms ", ms)); l = true; }
        if us > 0 || l { o.push_str(&format!("{:03}μs ", us)); l = true; }
        if ns > 0 || l { o.push_str(&format!("{:03}ns", ns)); }
        println!("{}", o);
        Ok(())
    }
}

fn setup() -> Result<Module, Box<dyn std::error::Error>> {
    Ok(ModuleBuilder::new(Timer).builtin(Timer::timer_cmd, Builtin::new("_timer")).build())
}