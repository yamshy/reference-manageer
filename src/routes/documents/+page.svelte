<script lang="ts">
	import { copyFile } from '@tauri-apps/api/fs';
	import { appDataDir, basename, join } from '@tauri-apps/api/path';
	import { invoke } from '@tauri-apps/api/tauri';
	import { appWindow } from '@tauri-apps/api/window';
	import { onDestroy, onMount } from 'svelte';

	let unlisten: () => void;
	let status = 'idle';

	onMount(async () => {
		await invoke('setup_local_db');

		const appDataDirPath = await appDataDir();
		const localFilePath = await join(appDataDirPath, 'local_files');

		unlisten = await appWindow.onFileDropEvent(async (event) => {
			if (event.payload.type === 'hover') {
				status = 'hover';
			} else if (event.payload.type === 'drop') {
				status = 'drop';

				const payloadPath = event.payload.paths[0];
				const fileName = await basename(payloadPath);
				const filePath = await join(localFilePath, fileName as string);

				await copyFile(payloadPath, filePath);
				await invoke('import_metadata', { filePath });
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
{#await appDataDir() then appDataDirPath}
	{appDataDirPath}
{/await}
