# Motivation: 
I have seen gnome/kde/cinnamon desktop to play a sound on USB device attach or detach event, but that is not present if you are using a Window Manager, so it is made for Window Manager/Compositor users who wants a sound cue for USB connections.

# Customization :
You can customize the sound(tone) that this programs plays, by replacing this file `~/.local/share/notify-usb/connect.mp3`

# Installation : 
```
git clone https://github.com/DebNation/notify-usb
cd notify-usb
cargo install --path .
```
# Run it on startup :
```
exec-once = ~/.cargo/bin/notify-usb 
```
