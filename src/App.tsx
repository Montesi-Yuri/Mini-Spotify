import "./App.css";
import { Track } from "./definitions/player";
import Player from "./Player";
import { listen } from '@tauri-apps/api/event'
import { useEffect, useState } from "react";

function App() {
	const [track, setTrack] = useState<Track | undefined>(undefined);

	useEffect(() => {
		const unlisten = listen("track", (e) => setTrack(e.payload as Track) );
		return () => {
			unlisten.then((f) => f());
		};
	}, []);

	return (
		<main className="container">
			<Player track={track}/>
		</main>
	);
}

export default App;
