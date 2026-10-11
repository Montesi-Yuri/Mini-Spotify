import { useEffect, useRef, useState } from "react"
import { player } from "./hooks/player"
import PlayIcon from "./icons/Play"
import ForwardIcon from "./icons/Forward"
import BackwardIcon from "./icons/Backward"
import { Track } from "./definitions/player"
import PauseIcon from "./icons/Pause"
import "./Player.css"

const formatTime = (ms: number) => {
    const total = Math.max(0, Math.floor(ms / 1000))
    const m = Math.floor(total / 60)
    const s = total % 60
    return `${m}:${s.toString().padStart(2, '0')}`
}

const Player = ({ track }: { track?: Track }) => {
    const isPlaying = Boolean(track?.playing)
    return (
        <div className="player" data-tauri-drag-region>
            <TrackInfo track={track} />
            <div className="player-center" data-tauri-drag-region>
                <PlayerControls isPlaying={isPlaying} />
                <ProgressBar track={track} />
            </div>
            <VolumeControl />
        </div>
    )
}

const PlayerControls = ({ isPlaying }: { isPlaying: boolean }) => {
    return (
        <div className="player-controls">
            <button className="icon-button" onClick={() => player.prev()}>
                <BackwardIcon />
            </button>
            <button className="icon-button play-button" onClick={() => player.playPause()}>
                {isPlaying ? <PauseIcon size={36} /> : <PlayIcon size={36} />}
            </button>
            <button className="icon-button" onClick={() => player.next()}>
                <ForwardIcon />
            </button>
        </div>
    )
}

const ProgressBar = ({ track }: { track?: Track }) => {
    const [positionMs, setPositionMs] = useState(0)
    const [dragMs, setDragMs] = useState<number | null>(null)
    const duration = track?.durationMs ?? 0

    // Interpolate the position between track updates while playing
    useEffect(() => {
        if (!track) return setPositionMs(0)
        const compute = () => {
            const elapsed = track.playing ? Date.now() - track.timestampMs : 0
            return Math.min(duration, track.positionMs + Math.max(0, elapsed))
        }
        setPositionMs(compute())
        if (!track.playing) return
        const id = setInterval(() => setPositionMs(compute()), 250)
        return () => clearInterval(id)
    }, [track, duration])

    const shown = dragMs ?? positionMs
    const percent = duration > 0 ? (shown / duration) * 100 : 0

    const commit = () => {
        if (dragMs === null) return
        setPositionMs(dragMs)
        player.seek(Math.round(dragMs))
        setDragMs(null)
    }

    return (
        <div className="progress">
            <span className="time">{formatTime(shown)}</span>
            <input
                type="range"
                className="slider"
                min={0}
                max={duration || 1}
                value={shown}
                disabled={!track}
                style={{ '--fill': `${percent}%` } as React.CSSProperties}
                onChange={(e) => setDragMs(Number(e.target.value))}
                onPointerUp={commit}
                onKeyUp={commit}
            />
            <span className="time">{formatTime(duration)}</span>
        </div>
    )
}

const VolumeControl = () => {
    const [volume, setVolume] = useState(50)
    const lastSent = useRef(0)

    const change = (value: number) => {
        setVolume(value)
        // Throttle calls to the backend while dragging
        const now = Date.now()
        if (now - lastSent.current < 100 && value !== 0 && value !== 100) return
        lastSent.current = now
        player.setVolume(value)
    }

    return (
        <div className="volume">
            <svg viewBox="0 0 24 24" width={18} height={18} fill="currentColor">
                <path d="M3 9v6h4l5 5V4L7 9H3z" />
                {volume > 0 && <path d="M14 8.5a4.5 4.5 0 0 1 0 7v-7z" />}
                {volume > 50 && <path d="M14 4.2v1.8a7 7 0 0 1 0 12v1.8a9 9 0 0 0 0-15.6z" />}
            </svg>
            <input
                type="range"
                className="slider"
                min={0}
                max={100}
                value={volume}
                style={{ '--fill': `${volume}%` } as React.CSSProperties}
                onChange={(e) => change(Number(e.target.value))}
                onPointerUp={() => player.setVolume(volume)}
            />
        </div>
    )
}

const TrackInfo = ({ track }: { track?: Track }) => {
    return (
        <div className="track-info" data-tauri-drag-region>
            {track?.artUrl && <img className="track-art" src={track.artUrl} alt="" />}
            {track && (
                <div className="track-text">
                    <div className="track-title">{track.title}</div>
                    <div className="track-artist">{track.artist}</div>
                </div>
            )}
        </div>
    )
}

export default Player
