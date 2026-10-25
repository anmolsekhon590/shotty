# Bug: `--freeze` leaks hyprpicker SHM files until tmpfs fills → Hyprland SIGBUS crash

## Symptom
Pressing the region-screenshot bind can crash **Hyprland** outright (whole session
drops to a crash/recovery shell). The user-visible trigger is shotty's freeze
feature, which runs `hyprpicker -r -z`.

## Evidence (from `coredumpctl`)
Two Hyprland cores (PIDs 1401 @14:04:27, 17712 @14:05:07) both:

```
Signal: 7 (BUS) si_code: BUS_ADRERR
#4  Render::GL::CGLFramebuffer::readPixels(...)
#5  Screenshare::CScreenshareFrame::copyShm()
#6  Screenshare::CScreenshareFrame::copy()
#7  Screenshare::CScreenshareManager::onOutputCommit(...)
...
#13 IHyprRenderer::commitPendingAndDoExplicitSync(...)
```

The matching hyprpicker core (PID 17308) at the same second:

```
Command Line: hyprpicker -r -z
Signal: 7 (BUS) BUS_ADRERR
#0  libpixman-1.so  (pixman_image_composite32)
#9  hyprpicker + 0x24523   (cairo_paint into a frozen-image buffer)
```

Faulting mapping in the Hyprland core (gdb `$_siginfo.si_addr` = `0x7fe7d08bcfc0`):

```
0x00007fe7d0200000 0x00007fe7d09e9000 0x7e9000 0x0  /run/user/1000/.hyprpicker_CoFokJ
```

i.e. Hyprland mmap'd the fd passed by hyprpicker over `wl_shm` (its
`$XDG_RUNTIME_DIR/.hyprpicker_XXXXXX` temp file) and bus-faulted inside
`glReadPixels` writing into it.

## Actual root cause
`hyprpicker` creates one SHM temp file per output in `$XDG_RUNTIME_DIR`
(`/run/user/1000/.hyprpicker_XXXXXX`) and **unlinks them on clean exit**
(verified: `SIGTERM` → 0 leftovers, `SIGKILL` → all leftovers leak).

`shotty` terminates the freeze process with `std::process::Child::kill()`, which
sends **SIGKILL**, so hyprpicker never runs its cleanup. Every region screenshot
with freeze enabled leaks ~2 files:

* `14745600` bytes = 2560×1440×4 (DP-1)
* `8294400` bytes = 1920×1080×4 (DP-2)

`/run/user/1000` is a **3.1 GB tmpfs**. After enough screenshots it reaches 100%:

```
$ df -h /run/user/1000
tmpfs  3.1G  3.1G  0  100% /run/user/1000
$ du -sch /run/user/1000/.hyprpicker_*
289 files, 3.1G total   (145 × 1440p + 144 × 1080p)
```

Once tmpfs has no free pages, accessing a mapped shm page that cannot be backed
raises `SIGBUS / BUS_ADRERR`. Both processes touching hyprpicker's buffer die —
and because Hyprland is the one doing `glReadPixels` into that buffer, the
**compositor** dies.

So the crash is a *consequence* of the leak, not of screencopy itself. Fullscreen
`grim` shots (no hyprpicker) never crash.

## Fix
In `src/commands/screenshot.rs`, every `child.kill()` (lines 105, 116, 141, 151)
sends SIGKILL. Replace with a graceful stop so hyprpicker can unlink its files:

```rust
fn stop_freeze(child: &mut std::process::Child) {
    // SIGTERM so hyprpicker unlinks its $XDG_RUNTIME_DIR/.hyprpicker_* files
    unsafe { libc::kill(child.id() as libc::pid_t, libc::SIGTERM); }
    // give it a moment, then hard-kill as a fallback
    for _ in 0..20 {
        if let Ok(Some(_)) = child.try_wait() { return; }
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
    let _ = child.kill();
    let _ = child.wait();
}
```

Additionally (defence in depth):

* Reap the child (`wait()`) on every path so no zombie/leak is possible.
* On startup, sweep stale `.hyprpicker_*` files in `$XDG_RUNTIME_DIR`
  (`libc::getpid`/`kill(pid,0)` can't tell whose they are, so only sweep files
  older than the current process/session, or track our own PIDs).

## Interim workaround (already applied)
`~/.config/hypr/hyprland.lua` passes `shotty --freeze false` so hyprpicker is
never spawned. Also `shotty -z` had become a clap usage error because commit
`9183e49` changed `-z/--freeze` to take a value.
