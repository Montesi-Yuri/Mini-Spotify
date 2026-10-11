import "./App.css";
import { Track } from "./definitions/player";
import Player from "./Player";
import { listen } from '@tauri-apps/api/event'
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

// Double-click on the widget (not on its controls) cycles the layout
const cycleLayout = (e: React.MouseEvent) => {
	if ((e.target as HTMLElement).closest("button, input")) return;
	invoke("cycle_layout");
};

function App() {
	const [track, setTrack] = useState<Track | undefined>(undefined);

	useEffect(() => {
		const unlisten = listen("track", (e) => setTrack(e.payload as Track) );
		return () => {
			unlisten.then((f) => f());
		};
	}, []);

	return (
		<main className="container" onDoubleClick={cycleLayout}>
			<Player track={track}/>
		</main>
	);
}

export default App;
