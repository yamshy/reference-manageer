<script lang="ts">
	import { emit } from '@tauri-apps/api/event';
	import { copyFile } from '@tauri-apps/api/fs';
	import { appDataDir } from '@tauri-apps/api/path';
	import { appWindow } from '@tauri-apps/api/window';
	import { onDestroy, onMount } from 'svelte';

	let unlisten: () => void;
	let status = 'idle';

	onMount(async () => {
		unlisten = await appWindow.onFileDropEvent(async (event) => {
			if (event.payload.type === 'hover') {
				status = 'hover';
			} else if (event.payload.type === 'drop') {
				const appDataDirPath = await appDataDir();
				const path = event.payload.paths[0];
				const fileName = path.split('/').pop();

				status = 'drop';

				await emit('file_drop', event.payload.paths[0]);
				await copyFile(path, `${appDataDirPath}/local_files/${fileName}`);
				console.log(`${path} copied to ${appDataDirPath}/local_files/${fileName}`);
			} else {
				status = 'idle';
			}
		});
	});

	onDestroy(() => {
		unlisten();
	});
</script>

{status}
