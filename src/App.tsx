import "./App.css";
import { Track } from "./definitions/player";
import Player from "./Player";
import { listen } from '@tauri-apps/api/event'
import { invoke } from "@tauri-apps/api/core";
import { useEffect, useState } from "react";

// Double-click on the widget (not on its controls) cycles the layout.
// Handled on the second mousedown: Tauri's drag region starts a native drag on the
// first one, which swallows the mouseup, so no `dblclick` event ever fires.
// Stopping propagation also keeps Tauri from maximizing the window.
const cycleLayout = (e: React.MouseEvent) => {
	if (e.button !== 0 || e.detail !== 2) return;
	if ((e.target as HTMLElement).closest("button, input")) return;
	e.stopPropagation();
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
		<main className="container" onMouseDown={cycleLayout}>
			<Player track={track}/>
		</main>
	);
}

export default App;
