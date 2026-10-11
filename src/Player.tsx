import { player } from "./hooks/player"
import PlayIcon from "./icons/Play"
import ForwardIcon from "./icons/Forward"
import BackwardIcon from "./icons/Backward"
import { Track } from "./definitions/player"
import PauseIcon from "./icons/Pause"

const Player = ({ track }: { track?: Track }) => {
    console.log('Track: ', track)
    const isPlaying = Boolean(track?.playing)
    return (
        <div>
            <PlayerControls isPlaying={isPlaying} />
            <TrackInfo track={track} />
        </div>
    )
}

const PlayerControls = ({ isPlaying }: { isPlaying: boolean }) => {
    return (
        <div style={{ display: 'flex', justifyContent: 'center', gap: 6 }}>
            <button
                onClick={() => player.prev()}
                style={{ display: 'flex', alignItems: 'center' }}>
                <BackwardIcon />
            </button>
            <button
                onClick={() => player.playPause()}
                style={{ display: 'flex', alignItems: 'center' }}>
                {
                    isPlaying ? <PauseIcon /> : <PlayIcon />
                }
            </button>
            <button
                onClick={() => player.next()}
                style={{ display: 'flex', alignItems: 'center' }}>
                <ForwardIcon />
            </button>
        </div>
    )
}

const TrackInfo = ({ track }: { track?: Track }) => {
    if (!track) return null

    const { artist, title, } = track
    return (
        <div style={{ }}>
            <h4>
                {title}
            </h4>
            <h5>
                {artist}
            </h5>
        </div>
    )
}


export default Player