//! ffmpeg pixel decoder used for palette generation.

use std::io::Read;
use std::path::Path;
use std::process::{Command, Stdio};

use anyhow::{anyhow, Context};

use rze_core::color::Rgb8;

pub fn decode_rgb24_cover(path: &Path, width: u32, height: u32) -> anyhow::Result<Vec<Rgb8>> {
    let vf = format!(
        "scale={width}:{height}:force_original_aspect_ratio=increase,crop={width}:{height},format=rgb24"
    );

    let ffmpeg = std::env::var_os("RZE_FFMPEG_BIN").unwrap_or_else(|| "ffmpeg".into());

    let mut cmd = Command::new(ffmpeg);
    cmd.arg("-v")
        .arg("error")
        .arg("-i")
        .arg(path)
        .arg("-vf")
        .arg(vf)
        .arg("-frames:v")
        .arg("1")
        .arg("-f")
        .arg("rawvideo")
        .arg("-pix_fmt")
        .arg("rgb24")
        .arg("-")
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = cmd.spawn().map_err(|e| {
        if e.kind() == std::io::ErrorKind::NotFound {
            anyhow!("ffmpeg not found (install ffmpeg or set RZE_FFMPEG_BIN)")
        } else {
            anyhow!(e)
        }
    })?;

    let mut out = Vec::new();
    child
        .stdout
        .take()
        .ok_or_else(|| anyhow!("ffmpeg stdout missing"))?
        .read_to_end(&mut out)
        .context("read ffmpeg stdout")?;

    let stderr = {
        let mut buf = Vec::new();
        if let Some(mut s) = child.stderr.take() {
            let _ = s.read_to_end(&mut buf);
        }
        String::from_utf8_lossy(&buf).to_string()
    };

    let status = child.wait().context("wait ffmpeg")?;
    if !status.success() {
        if stderr.trim().is_empty() {
            return Err(anyhow!("ffmpeg failed"));
        }
        return Err(anyhow!("ffmpeg failed: {}", stderr.trim()));
    }

    let expected = (width * height * 3) as usize;
    if out.len() != expected {
        return Err(anyhow!("unexpected pixel output size"))
            .with_context(|| format!("got {} expected {expected}", out.len()));
    }

    let mut px = Vec::with_capacity((width * height) as usize);
    for i in (0..out.len()).step_by(3) {
        px.push(Rgb8::new(out[i], out[i + 1], out[i + 2]));
    }
    Ok(px)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::Mutex;
    use std::time::{SystemTime, UNIX_EPOCH};

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn write_exe(path: &std::path::Path, body: &str) {
        fs::write(path, body).unwrap();
        let mut perm = fs::metadata(path).unwrap().permissions();
        perm.set_mode(0o755);
        fs::set_permissions(path, perm).unwrap();
    }

    fn temp_dir() -> std::path::PathBuf {
        let nonce = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let dir = std::env::temp_dir().join(format!("rze-test-{nonce}-{}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        dir
    }

    #[test]
    fn decode_1x1_png() {
        let _g = ENV_LOCK.lock().unwrap();

        let dir = temp_dir();
        let fake = dir.join("ffmpeg");
        // Write exactly 3 bytes (rgb24 for 1x1) and succeed.
        write_exe(&fake, "#!/bin/sh\n\nhead -c 3 /dev/zero\nexit 0\n");

        let old = std::env::var_os("RZE_FFMPEG_BIN");
        unsafe { std::env::set_var("RZE_FFMPEG_BIN", &fake) };

        let p = dir.join("a.png");
        fs::write(
            &p,
            [
                0x89, 0x50, 0x4e, 0x47, 0x0d, 0x0a, 0x1a, 0x0a, 0x00, 0x00, 0x00, 0x0d, 0x49, 0x48,
                0x44, 0x52, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x01, 0x08, 0x06, 0x00, 0x00,
                0x00, 0x1f, 0x15, 0xc4, 0x89, 0x00, 0x00, 0x00, 0x0a, 0x49, 0x44, 0x41, 0x54, 0x78,
                0x9c, 0x63, 0x60, 0x00, 0x00, 0x00, 0x02, 0x00, 0x01, 0xe5, 0x27, 0xd4, 0xa2, 0x00,
                0x00, 0x00, 0x00, 0x49, 0x45, 0x4e, 0x44, 0xae, 0x42, 0x60, 0x82,
            ],
        )
        .unwrap();

        let px = decode_rgb24_cover(&p, 1, 1).unwrap();
        let _ = fs::remove_file(&p);
        let _ = fs::remove_file(&fake);
        assert_eq!(px.len(), 1);

        unsafe {
            match old {
                Some(v) => std::env::set_var("RZE_FFMPEG_BIN", v),
                None => std::env::remove_var("RZE_FFMPEG_BIN"),
            }
        }
    }

    #[test]
    fn decode_errors_on_nonzero_exit() {
        let _g = ENV_LOCK.lock().unwrap();

        let dir = temp_dir();
        let fake = dir.join("ffmpeg");
        write_exe(&fake, "#!/bin/sh\n\necho 'boom' 1>&2\nexit 1\n");

        let old = std::env::var_os("RZE_FFMPEG_BIN");
        unsafe { std::env::set_var("RZE_FFMPEG_BIN", &fake) };

        let img = dir.join("img.png");
        fs::write(&img, [0u8; 8]).unwrap();

        let err = decode_rgb24_cover(&img, 1, 1).unwrap_err();
        let s = format!("{err:#}");
        assert!(s.contains("ffmpeg failed"));

        unsafe {
            match old {
                Some(v) => std::env::set_var("RZE_FFMPEG_BIN", v),
                None => std::env::remove_var("RZE_FFMPEG_BIN"),
            }
        }
    }

    #[test]
    fn decode_errors_on_unexpected_size() {
        let _g = ENV_LOCK.lock().unwrap();

        let dir = temp_dir();
        let fake = dir.join("ffmpeg");
        // Write 3 bytes but ask for 2x2 (= 12 bytes expected).
        write_exe(&fake, "#!/bin/sh\n\nhead -c 3 /dev/zero\nexit 0\n");

        let old = std::env::var_os("RZE_FFMPEG_BIN");
        unsafe { std::env::set_var("RZE_FFMPEG_BIN", &fake) };

        let img = dir.join("img.png");
        fs::write(&img, [0u8; 8]).unwrap();

        let err = decode_rgb24_cover(&img, 2, 2).unwrap_err();
        assert!(format!("{err:#}").contains("unexpected pixel output size"));

        unsafe {
            match old {
                Some(v) => std::env::set_var("RZE_FFMPEG_BIN", v),
                None => std::env::remove_var("RZE_FFMPEG_BIN"),
            }
        }
    }
}
