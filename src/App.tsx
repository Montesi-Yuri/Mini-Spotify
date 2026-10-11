import "./App.css";
import Player from "./Player";
import { listen } from '@tauri-apps/api/event'
import { useEffect } from "react";

function App() {
	useEffect(() => {
		const unlisten = listen("track", (e) => console.log(e.payload));
		return () => {
			unlisten.then((f) => f());
		};
	}, []);

	return (
		<main className="container">
			<h1>Spotify Widget</h1>
			<Player />
		</main>
	);
}

export default App;
