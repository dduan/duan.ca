# Ubuntu on the HP OmniBook Ultra 16 (RTX Spark N1X)
2026-11-21T03:05:20-08:00
tags: Linux, Ubuntu, NVIDIA, ARM

*Tested in November 2026 on the HP OmniBook Ultra Laptop 16-bk0xxx (board 8E6A), BIOS F.06*


Ubuntu 26.04 runs well on the HP OmniBook Ultra 16, but out of the box the GPU, speakers and battery don't work. You can fix all three in an afternoon with packages and two community workarounds. This post covers what I hit, how I fixed each problem, and what still doesn't work.

The laptop uses NVIDIA's **RTX Spark N1X**: an Arm chip (10 Cortex-X925 and 10 Cortex-A725 cores) with a Blackwell GPU, plus MediaTek Wi-Fi and audio. Support for this hardware is very new, and most of the fixes below aren't in Ubuntu yet.

## At a glance

<table>
<thead><tr><th>Area</th><th>Out of the box</th><th>After the fixes</th></tr></thead>
<tbody>
<tr><td>Wi-Fi, Bluetooth, SSD, card reader</td><td>Works</td><td>Works</td></tr>
<tr><td>Webcam (5MP + IR), touchpad, touchscreen, keyboard</td><td>Works</td><td>Works</td></tr>
<tr><td>NVIDIA GPU, brightness, external displays</td><td>Basic framebuffer only</td><td>Works (driver 595)</td></tr>
<tr><td>Speakers, headphone jack, internal mic</td><td>"Dummy Output"</td><td>Works (speakers sound thinner than on Windows)</td></tr>
<tr><td>Battery, charger, lid, thermals, USB-C ports</td><td>Not detected</td><td>Works</td></tr>
<tr><td>Fan speed reading</td><td>Stuck at 0 RPM</td><td>Live reading</td></tr>
<tr><td>Keyboard backlight control</td><td>None</td><td>None (the Fn key may still work)</td></tr>
<tr><td>Thunderbolt / USB4</td><td>Not working</td><td>Not working</td></tr>
<tr><td>Hibernate</td><td>Not supported</td><td>Not supported</td></tr>
</tbody>
</table>

## Which Ubuntu and kernel to use

Use **Ubuntu 26.04 LTS (arm64)** with the **`linux-nvidia-bos`** kernel. I ran `7.0.0-2024-nvidia-bos`. This NVIDIA-specific kernel includes the N1X platform support: the MediaTek audio and SoundWire drivers, the bridge to the embedded controller, and USB.

- **Kernel packages:** `linux-image-nvidia-bos` and `linux-headers-nvidia-bos`. My install already had them. Check with `uname -r`, which should end in `-nvidia-bos`.
- **Stay on this kernel flavour.** The NVIDIA driver modules and the audio stack are built for it. Don't switch to the generic kernel.
- **Secure Boot can stay on.** Everything below works with it. The only cost is enrolling one signing key (MOK) for the battery module.
- **Update everything first:** `sudo apt update && sudo apt full-upgrade`, then reboot. There's no BIOS update to install: on F.06, `fwupdmgr` finds nothing for this machine.

## What works out of the box

<table>
<thead><tr><th>Hardware</th><th>Driver</th><th>Notes</th></tr></thead>
<tbody>
<tr><td>MediaTek MT7925 Wi-Fi 7</td><td><code>mt7925e</code></td><td>The log says <code>sar cnt = 0</code> (no HP power-limit table). Harmless</td></tr>
<tr><td>MediaTek Bluetooth</td><td><code>btusb</code></td><td></td></tr>
<tr><td>KIOXIA 1 TB NVMe SSD</td><td><code>nvme</code></td><td></td></tr>
<tr><td>HP 5MP camera + IR camera</td><td><code>uvcvideo</code></td><td>Face unlock with the IR camera needs extra software (e.g. Howdy)</td></tr>
<tr><td>Realtek RTS525A card reader</td><td><code>rtsx_pci</code></td><td></td></tr>
<tr><td>Synaptics touchpad, ELAN touchscreen, keyboard</td><td>HID</td><td></td></tr>
<tr><td>HDMI/DisplayPort audio</td><td><code>snd_hda_codec_tegrahdmi</code></td><td></td></tr>
<tr><td>TPM 2.0</td><td></td><td></td></tr>
</tbody>
</table>

## What's broken out of the box

1. **GPU (`10de:2e03`): no driver.** The desktop runs on `simpledrm`, a basic framebuffer. That means no acceleration, no brightness control, and no external monitors.
2. **Sound: "Dummy Output" only.** The hardware is four Cirrus CS35L56 speaker amps and a CS42L43 headset/mic chip, on MediaTek's `mt8901` SoundWire controller. The kernel drivers load fine. What's missing is the ALSA audio profile (UCM) that tells PipeWire how to route sound, so PipeWire falls back to a dummy device.
3. **Battery, charger, lid, thermals, fan and USB-C: not detected.** `/sys/class/power_supply/` is empty and the lid state reads `unsupported`. The kernel log repeats this firmware error dozens of times per boot:

   ```
   ACPI BIOS Error (bug): AE_AML_BUFFER_LIMIT, Field [DATA] at bit offset/length 208/400 exceeds size of target Buffer (512 bits)
   ACPI Error: Aborting method \_SB.FFA0.RBUF due to previous error (AE_AML_BUFFER_LIMIT)
   ```

   HP's firmware defines a 50-byte field inside a 64-byte buffer, 26 bytes in, so it runs 12 bytes past the end. That method (`\_SB.FFA0.RBUF`) handles **every** read from the embedded controller. So everything that talks to the controller fails: battery, AC adapter, thermal zone, fan, USB-C, lid, and one of the GPU's firmware calls.

   This also breaks power-based settings. With no AC adapter detected, GNOME and systemd assume you're always plugged in, so the "on battery" sleep timeout never applies.

## Fix 1: NVIDIA GPU driver

Install the open-kernel driver together with the prebuilt modules for the `nvidia-bos` kernel:

```bash
sudo apt install --no-install-recommends nvidia-driver-595-open linux-modules-nvidia-595-open-nvidia-bos
sudo reboot
```

- The `linux-modules-nvidia-*-nvidia-bos` package contains modules Canonical has already signed. That means **no DKMS build and no MOK enrollment**, even with Secure Boot on.
- `ubuntu-drivers` doesn't suggest this driver, and the N1X's ID (`2e03`) isn't on the driver's list of supported GPUs. Only the DGX Spark's GB10 (`2e12`) is listed. It still binds and works fine: `nvidia-smi` reports *NVIDIA RTX Spark N1X (6144-core Blackwell RTX GPU)* with CUDA 13.2.
- Afterwards you get the built-in panel as `eDP-1`, the HDMI and USB-C display outputs, and an `nvidia_0` backlight, so brightness keys and the slider work.
- One harmless leftover in the log: `NVRM: osCallACPI_DSM: Error during 0x3 DSM subfunction 0x10`. That's the same firmware bug and goes away after Fix 3.

**If you get a black screen:** pick *Advanced options → recovery mode* in the boot menu (GRUB), then run `apt purge 'nvidia-*'`.

## Fix 2: Speakers and microphone

NVIDIA has submitted an audio profile for this sound chip, but it isn't merged yet: [alsa-ucm-conf PR #862](https://github.com/alsa-project/alsa-ucm-conf/pull/862). It covers six N1X laptop layouts. The OmniBook Ultra 16 is the one with four CS35L56 amps on link 0 and a CS42L43 on link 1. Ubuntu's own `alsa-ucm-conf` (1.2.15.3-1ubuntu1.7 at the time of writing) doesn't include it yet.

You can install it for your user only, without touching `/usr`. Build a private profile folder that links to the system's files, add the new profile, and point PipeWire at it with `ALSA_CONFIG_UCM2`:

```bash
#!/usr/bin/env bash
set -euo pipefail
SHA=c22cf4c960c7f68e13420140e6ec38f1e0ce70b5   # PR #862 head as of 2026-11-21
RAW=https://raw.githubusercontent.com/dcemin-nv/alsa-ucm-conf/$SHA/ucm2
U=$HOME/.config/alsa/ucm2; SYS=/usr/share/alsa/ucm2

mkdir -p "$U/conf.d" "$U/MediaTek/mt8901-soundwir" "$U/conf.d/mt8901-soundwir"
for e in "$SYS"/*; do b=$(basename "$e"); case $b in conf.d|MediaTek) ;; *) ln -sfn "$e" "$U/$b";; esac; done
for e in "$SYS"/conf.d/*;   do ln -sfn "$e" "$U/conf.d/"; done
for e in "$SYS"/MediaTek/*; do ln -sfn "$e" "$U/MediaTek/"; done

for f in mt8901-soundwir.conf HiFi-cs35l56x4-cs42l43.conf HiFi-cs42l43-cs35l56x4.conf \
         HiFi-cs42l43-cs35l56x2.conf HiFi-rt1320x2-rt712.conf HiFi-rt1320x2-rt712-refboard.conf \
         HiFi-rt1321x2-rt713.conf; do
  curl -sSfo "$U/MediaTek/mt8901-soundwir/$f" "$RAW/MediaTek/mt8901-soundwir/$f"
done
ln -sfn ../../MediaTek/mt8901-soundwir/mt8901-soundwir.conf "$U/conf.d/mt8901-soundwir/mt8901-soundwir.conf"

for s in pipewire wireplumber; do
  mkdir -p ~/.config/systemd/user/$s.service.d
  printf '[Service]\nEnvironment=ALSA_CONFIG_UCM2=%%h/.config/alsa/ucm2\n' > ~/.config/systemd/user/$s.service.d/10-alsa-ucm2.conf
done
mkdir -p ~/.config/environment.d
echo 'ALSA_CONFIG_UCM2=${HOME}/.config/alsa/ucm2' > ~/.config/environment.d/10-alsa-ucm2.conf

systemctl --user daemon-reload
systemctl --user restart wireplumber pipewire pipewire-pulse
```

Check that it loaded:

```bash
ALSA_CONFIG_UCM2=~/.config/alsa/ucm2 alsaucm -c hw:mt8901soundwire list _verbs   # expect: HiFi
wpctl status   # expect "Internal speakers", "Headphone jack", "Internal microphone array"
```

The profile chooses a layout based on which SoundWire devices are present. On this laptop `sdw:0:0:01fa:3556:01:0` exists, so it picks `HiFi-cs35l56x4-cs42l43.conf`.

### Fix the hissing microphone

Once the mic works it records loud hiss, because **PipeWire's 100% mic volume equals the CS42L43's maximum boost of +31.5 dB** (`Decimator 3/4 Volume` = 191 of 191). At that level the mic's own noise is boosted until it clips, even in a silent room. I measured silence by mic volume:

<table>
<thead><tr><th>PipeWire mic volume</th><th>Hardware gain</th><th>Silent-room level</th></tr></thead>
<tbody>
<tr><td>100%</td><td>+31.5 dB</td><td>clipping, about 9% of samples at full scale</td></tr>
<tr><td>40%</td><td>+7.5 dB</td><td>steady, about −54 dBFS</td></tr>
<tr><td>30%</td><td>0 dB</td><td>about −62 dBFS</td></tr>
</tbody>
</table>

Set it to about 40% and leave it there. WirePlumber remembers it:

```bash
wpctl set-volume @DEFAULT_AUDIO_SOURCE@ 0.4
```

### Speaker caveat

The speaker amps report `Cirrus Logic CS35L56 Rev B0 OTP3 fw:3.4.4 (patched=0)`. That's because HP's tuning files for this laptop (subsystem `103c8e6a`) aren't in `linux-firmware`. Without them the amps run in a basic fallback mode, so expect the speakers to sound thinner and quieter than on Windows. There's no fix until HP or Cirrus submits the files.

**When Ubuntu ships the profile**, remove the override so you get updates:

```bash
rm -r ~/.config/alsa/ucm2 ~/.config/systemd/user/*/10-alsa-ucm2.conf ~/.config/environment.d/10-alsa-ucm2.conf
```

## Fix 3: Battery, charger, lid, thermals and USB-C

There's no BIOS fix yet. You also can't load corrected firmware tables (a DSDT override) at boot, because Secure Boot puts the kernel in lockdown, which blocks it. The workaround is **`omnibook_ffa_fix`**, a small community kernel module from [keejkrej/omarchy-omnibook-ultra-16](https://github.com/keejkrej/omarchy-omnibook-ultra-16) (root cause in [Omarchy issue #18112](https://github.com/omacom/omarchy/issues/18112)). It makes two in-memory fixes when it loads:

- **Firmware buffer bug:** it shrinks the oversized field in `\_SB.FFA0.RBUF` from 400 to 304 bits, so it fits in the 64-byte buffer. Nothing ever reads that field, so this is safe.
- **Battery status:** it points `\_SB.BAT0._BST` at HP's own `UPBS()` method. As shipped, `_BST` asks for NVIDIA's reference controller ID, which doesn't exist on HP's controller.

After it loads, it re-probes the battery, AC adapter, thermal zone, USB-C and fan devices.

**Read the source before you load it.** It works by changing the kernel's internal ACPI structures (ACPICA) directly, at hard-coded positions: `aml_start` at +32 and `aml_length` at +48 in `struct acpi_object_method`. I checked these against Linux 7.0 (ACPICA 20251212) and they match. The module also checks the object's type before writing anything. Still, if a future kernel changes that structure, re-check it.

Install it with DKMS, so it's rebuilt for every kernel update:

```bash
sudo apt install build-essential dkms
git clone https://github.com/keejkrej/omarchy-omnibook-ultra-16
sudo mkdir -p /usr/src/omnibook-ffa-fix-1.0.0
sudo cp omarchy-omnibook-ultra-16/kmod/{omnibook_ffa_fix.c,Makefile,dkms.conf} /usr/src/omnibook-ffa-fix-1.0.0/
sudo dkms add -m omnibook-ffa-fix -v 1.0.0
sudo dkms install -m omnibook-ffa-fix -v 1.0.0
echo omnibook_ffa_fix | sudo tee /etc/modules-load.d/omnibook-ffa-fix.conf
```

### Secure Boot: enroll the signing key

DKMS signs the module with a local key, `/var/lib/shim-signed/mok/MOK.der`. The kernel won't load it until that key is enrolled. Until then you'll see `Key was rejected by service` and a failed `systemd-modules-load.service`.

```bash
sudo mokutil --import /var/lib/shim-signed/mok/MOK.der   # choose a one-time password
sudo reboot
```

**Watch the screen during this reboot.** A blue "MOK management" screen appears and continues booting after about 10 seconds, which drops the request. I missed it the first time. Press a key straight away, then choose **Enroll MOK → Continue → Yes**, type the password, and reboot. Check it worked with `mokutil --test-key /var/lib/shim-signed/mok/MOK.der`, which should say "already enrolled".

### Result

```
omnibook_ffa_fix: Patched \_SB.FFA0.RBUF CreateField DATA bit length (0x0190 -> 0x0130 at +0x64)
omnibook_ffa_fix: Redirected \_SB.BAT0._BST to evaluate \_SB.BAT0.UPBS()
ACPI: battery: Slot [BAT0] (battery present)
ACPI: AC: AC Adapter [ADP1] (on-line)
ACPI: thermal: Thermal Zone [TZ01] (47 C)
```

```
$ ls /sys/class/power_supply/
ADP1  BAT0  ucsi-source-psy-USBC000:001  ucsi-source-psy-USBC000:002
$ upower -i /org/freedesktop/UPower/devices/battery_BAT0 | grep -E 'state|percentage|energy-full'
    state:               charging
    energy-full:         98.463 Wh
    energy-full-design:  99.33 Wh
    percentage:          35%
```

The battery percentage, time remaining, charger detection, lid switch, the second thermal zone and both USB-C ports all work now. GNOME's AC and battery power settings behave properly again. You'll still see about 70 `AE_AML_BUFFER_LIMIT` errors in the first 5 seconds of each boot, before the module loads. That's expected, and none appear after it.

## Fix 4: Fan speed reading

With Fix 3 in place, the fan still reads 0 RPM, even under load. HP's `\_SB.FAN0._FST` method only returns a stored value (`F1CS`). That value is refreshed by `F1LS()` only when the embedded controller sends a fan event, and all of those failed before the module loaded. I added a few lines to the module so `_FST` calls `F1LS()` first, which makes every read live. It applies on top of the module from Fix 3; bump `PACKAGE_VERSION` in `dkms.conf` to 1.1.0 when you reinstall.

<details>
<summary><code>fan-speed.patch</code> (applies to <code>kmod/omnibook_ffa_fix.c</code>)</summary>

```diff
--- a/omnibook_ffa_fix.c
+++ b/omnibook_ffa_fix.c
@@ -67,6 +67,18 @@
 static u8 *orig_bst_aml;
 static u32 orig_bst_len;
 
+/*
+ * \_SB.FAN0._FST only returns F1CS, which F1LS() refreshes from the EC when
+ * an EC fan event arrives. Prepend a call to F1LS so every _FST read is live:
+ *   46 31 4c 53       F1LS ()   (resolved in the enclosing FAN0 scope)
+ */
+static const u8 fst_prefix_aml[] = { 0x46, 0x31, 0x4c, 0x53 };
+
+static void *fst_method_obj;
+static u8 *orig_fst_aml;
+static u32 orig_fst_len;
+static u8 *patched_fst_aml;
+
 static struct delayed_work rescan_work;
 static int rescan_attempts;
 
@@ -159,6 +171,45 @@
 	return 0;
 }
 
+static int patch_fst_aml(void)
+{
+	acpi_handle h;
+
+	if (ACPI_FAILURE(acpi_get_handle(NULL, "\\_SB.FAN0.F1LS", &h))) {
+		pr_warn("\\_SB.FAN0.F1LS not found, skipping _FST patch\n");
+		return -ENODEV;
+	}
+
+	fst_method_obj = get_acpica_method_obj("\\_SB.FAN0._FST");
+	if (!fst_method_obj) {
+		pr_warn("\\_SB.FAN0._FST not found, skipping _FST patch\n");
+		return -ENODEV;
+	}
+
+	orig_fst_aml = *(u8 **)((u8 *)fst_method_obj + ACPICA_METHOD_AML_START_OFFSET);
+	orig_fst_len = *(u32 *)((u8 *)fst_method_obj + ACPICA_METHOD_AML_LEN_OFFSET);
+	if (!orig_fst_aml || !orig_fst_len || orig_fst_len > 4096) {
+		pr_warn("\\_SB.FAN0._FST invalid AML pointer/length (%u)\n", orig_fst_len);
+		orig_fst_aml = NULL;
+		return -EINVAL;
+	}
+
+	patched_fst_aml = kmalloc(sizeof(fst_prefix_aml) + orig_fst_len, GFP_KERNEL);
+	if (!patched_fst_aml) {
+		orig_fst_aml = NULL;
+		return -ENOMEM;
+	}
+	memcpy(patched_fst_aml, fst_prefix_aml, sizeof(fst_prefix_aml));
+	memcpy(patched_fst_aml + sizeof(fst_prefix_aml), orig_fst_aml, orig_fst_len);
+
+	*(u32 *)((u8 *)fst_method_obj + ACPICA_METHOD_AML_LEN_OFFSET) =
+		sizeof(fst_prefix_aml) + orig_fst_len;
+	*(u8 **)((u8 *)fst_method_obj + ACPICA_METHOD_AML_START_OFFSET) = patched_fst_aml;
+
+	pr_info("Patched \\_SB.FAN0._FST to refresh fan speed via F1LS()\n");
+	return 0;
+}
+
 static void attach_platform_dev_by_name(const char *name)
 {
 	struct device *dev;
@@ -229,6 +280,7 @@
 		return ret;
 
 	patch_bst_aml();
+	patch_fst_aml();
 
 	INIT_DELAYED_WORK(&rescan_work, rescan_ffa_consumers);
 	rescan_ffa_consumers(&rescan_work.work);
@@ -240,6 +292,12 @@
 {
 	cancel_delayed_work_sync(&rescan_work);
 
+	if (fst_method_obj && orig_fst_aml) {
+		*(u8 **)((u8 *)fst_method_obj + ACPICA_METHOD_AML_START_OFFSET) = orig_fst_aml;
+		*(u32 *)((u8 *)fst_method_obj + ACPICA_METHOD_AML_LEN_OFFSET) = orig_fst_len;
+	}
+	kfree(patched_fst_aml);
+
 	if (bst_method_obj && orig_bst_aml) {
 		*(u8 **)((u8 *)bst_method_obj + ACPICA_METHOD_AML_START_OFFSET) = orig_bst_aml;
 		*(u32 *)((u8 *)bst_method_obj + ACPICA_METHOD_AML_LEN_OFFSET) = orig_bst_len;
@@ -259,4 +317,5 @@
 
 MODULE_LICENSE("GPL");
 MODULE_AUTHOR("keejkrej");
-MODULE_DESCRIPTION("HP OmniBook Ultra 16-bk0xxx ACPI FF-A EC RBUF & Battery Fix");
+MODULE_DESCRIPTION("HP OmniBook Ultra 16-bk0xxx ACPI FF-A EC RBUF, Battery & Fan Speed Fix");
+MODULE_VERSION("1.1.0");
```

</details>

I ran all 20 cores flat out for 75 seconds to test it:

<table>
<thead><tr><th>Time</th><th>Fan (<code>fan_speed_rpm</code>)</th><th>SoC / system temp</th></tr></thead>
<tbody>
<tr><td>Idle</td><td>0 RPM (the fan really is off)</td><td>51 °C / 52 °C</td></tr>
<tr><td>15 s</td><td>2658 RPM</td><td>98 °C / 97 °C</td></tr>
<tr><td>30–75 s</td><td>about 3990 RPM</td><td>95–98 °C</td></tr>
<tr><td>20 s after the test</td><td>3711 RPM</td><td>57 °C / 76 °C</td></tr>
</tbody>
</table>

## What still doesn't work

<table>
<thead><tr><th>Problem</th><th>What's going on</th><th>Outlook</th></tr></thead>
<tbody>
<tr><td><strong>Keyboard backlight control</strong></td><td>Linux sees no keyboard backlight. I checked the firmware tables: HP gives the operating system no control, only a BIOS timeout setting. The embedded controller handles brightness, presumably via the Fn key.</td><td>No OS control until HP adds one. Try the Fn key.</td></tr>
<tr><td><strong>Thunderbolt / USB4</strong></td><td>The USB4 driver loads but no USB4 controller ever appears, and the log shows <code>NVIDIA USB4 disable _DSM: returned 0x00 (failure)</code>. Docks and enclosures that need USB4/Thunderbolt probably won't run at full speed.</td><td>Waiting on NVIDIA/HP. Ordinary USB-C devices and charging work.</td></tr>
<tr><td><strong>USB-C alternate modes</strong></td><td><code>ucsi_acpi: failed to register partner alt modes (-5)</code>, <code>GET_CABLE_PROPERTY failed</code>.</td><td>Video over USB-C to some docks may not work. Untested.</td></tr>
<tr><td><strong>Speaker tuning</strong></td><td>No HP tuning files for the CS35L56 amps (<code>patched=0</code>).</td><td>Waiting on <code>linux-firmware</code>.</td></tr>
<tr><td><strong>Hibernate</strong></td><td>There's no <code>/sys/power/disk</code> on this platform.</td><td>Normal for Arm laptops.</td></tr>
<tr><td><strong>Slow ACPI warnings</strong></td><td><code>workqueue: acpi_os_execute_deferred hogged CPU for &gt;20000us</code>: each embedded-controller call over the FF-A bridge takes over 20 ms.</td><td>Cosmetic. It may cost a little battery life.</td></tr>
</tbody>
</table>

**Not yet tested:** suspend/resume (I haven't run a sleep cycle yet), an external monitor over HDMI or USB-C, the headphone jack and headset mic with something plugged in, and the HP-specific function keys.

## Keeping it working

- **After kernel updates:** DKMS rebuilds the battery module automatically. If the battery disappears after an update, check `dkms status` and `journalctl -b -u systemd-modules-load`.
- **When HP releases a BIOS that fixes `\_SB.FFA0.RBUF`:** remove the module (`sudo dkms remove omnibook-ffa-fix/1.1.0 --all` and delete `/etc/modules-load.d/omnibook-ffa-fix.conf`), then reboot.
- **When Ubuntu's `alsa-ucm-conf` includes `ucm2/MediaTek/mt8901-soundwir`:** delete the per-user audio override (see Fix 2).

## Sources

- [alsa-ucm-conf PR #862](https://github.com/alsa-project/alsa-ucm-conf/pull/862): mt8901 SoundWire UCM profiles
- [NVIDIA NV-Kernels PR #567](https://github.com/NVIDIA/NV-Kernels/pull/567): N1X audio kernel stack
- [Omarchy issue #18112](https://github.com/omacom/omarchy/issues/18112): the `\_SB.FFA0.RBUF` firmware bug on this model, with the root cause
- [keejkrej/omarchy-omnibook-ultra-16](https://github.com/keejkrej/omarchy-omnibook-ultra-16): the `omnibook_ffa_fix` module
- [johannhartmann/asus-proart-p16-rtx-spark-linux](https://github.com/johannhartmann/asus-proart-p16-rtx-spark-linux): related N1X work on the ASUS ProArt P16
