### Motivation(why this exists): 
I have seen gnome/kde/cinnamon desktop to play a sound on USB device attach or detach event, but that is not present if you are using a Window Manager, so it is made for Window Manager/Compositor users who wants a sound cue for USB connections.

### Customization :
You can customize the sound(tone) that this programs plays, by replacing these files
- `~/.local/share/notify-usb/connect.mp3`
- `~/.local/share/notify-usb/disconnect.mp3`

### Installation : 
```
cargo install notify-usb
```
### Run it on startup(eg: hyprland) :
```
exec-once = ~/.cargo/bin/notify-usb 
```
