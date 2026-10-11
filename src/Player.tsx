import { player } from "./hooks/player"

const Player = () => {

    return (
        <div style={{ display: 'flex', justifyContent:'center', gap: 6}}>
            <button onClick={() => player.prev()}>
                Prev
            </button>

            <button onClick={() => player.playPause()}>
                Play
            </button>

            <button onClick={() => player.next()}>
                Next
            </button>
        </div>
    )
}
export default Player