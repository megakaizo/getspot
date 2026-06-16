# getspot

A CLI utility to download Spotify tracks, albums, or playlists in original audio quality.
It works by intercepting audio stream from spotify using `librespot` and `ffmpeg`, saving the recording as a audio file and automatically managing the playback state.

- Guaranteed to work on Linux
- Not Guaranteed on Windows (need testing)


## Installation

```bash
cargo install --git https://github.com/megakaizo/getspot
```


## Usage

1. Find audio monitor
```bash
pactl list short sources | grep monitor
```
Copy the name (e.g., `alsa_output.pci-0000_00_1f.3.analog-stereo.monitor`).

```bash
getspot track <spotfy_track_id> -s <audio_monitor> [-o /path]
getspot album <spotify_album_id> -s <audio_monitor> [-o /path]
getspot playlist <spotify_playlist_id> -s <audio_monitor> -n <start_track> [-o /path]
```
-o – output directory (default: `./data`)
-n – starting track number in playlist (`1` for the first track) (only for playlist)


## Example
downloading track:
```
getspot track 3z8h0TU7ReDPLIbEnYhWZb -s alsa_output.pci-0000_00_1f.3.analog-stereo.monitor -o ~/Music
```

playlist tracks for the 3rd:

```
getspot playlist 37i9dQZF1DXcBWIGoYBM5M -s alsa_output.pci-0000_00_1f.3.analog-stereo.monitor -n 3 -o ~/Music
```


spotify ids you can found like:

Track: `https://open.spotify.com/track/<id>`
Album: `https://open.spotify.com/album/<id>`
Playlist: `https://open.spotify.com/playlist/<id>`


## Note
The tool controls the player state automatically (play/pause) to capture the exact audio.

Metadata (track name, artists, album, cover, etc.) is extracted and added to the output file.

The output format is MP3.



