<script lang="ts">
	import { emit } from '@tauri-apps/api/event';
	import { appWindow } from '@tauri-apps/api/window';
	import { onDestroy, onMount } from 'svelte';

	let unlisten: () => void;
	let status = 'idle';
	let payload;

	onMount(async () => {
		unlisten = await appWindow.onFileDropEvent(async (event) => {
			payload = event.payload;
			if (event.payload.type === 'hover') {
				status = `User is hovering with ${event.payload.paths.join(', ')}`;
			} else if (event.payload.type === 'drop') {
				status = `User dropped ${event.payload.paths.join(', ')}`;
				await emit('file_drop', event.payload.paths[0]);
			} else {
				status = 'Drop cancelled';
			}
		});
	});

	onDestroy(() => {
		unlisten();
	});
</script>

{status}
