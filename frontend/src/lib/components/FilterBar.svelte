<script lang="ts">
	import { createEventDispatcher } from 'svelte';

	const dispatch = createEventDispatcher();

	let contract = 'OpenFin / OpenFin';
	let url = '';
	let urlMatch = 'exact';
	let uuid = '';
	let uuidMatch = 'exact';
	let appName = '';
	let appNameMatch = 'exact';
	let billable = 'all';
	let ignoreFiles: boolean | undefined = undefined;

	function handleSearch() {
		dispatch('filterChange', {
			contract: contract || undefined,
			url: url || undefined,
			url_match: urlMatch,
			uuid: uuid || undefined,
			uuid_match: uuidMatch,
			app_name: appName || undefined,
			app_name_match: appNameMatch,
			billable: billable === 'all' ? undefined : billable,
			ignore_files: ignoreFiles
		});
	}

	function handleExport() {
		dispatch('export');
	}
</script>

<div class="bg-gray-800 rounded-lg p-6 mb-6">
	<div class="grid grid-cols-2 gap-6 mb-6">
		<!-- Contract Filter -->
		<div>
			<label class="block text-sm text-gray-400 mb-2">contract:</label>
			<select bind:value={contract} class="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2">
				<option>OpenFin / OpenFin</option>
			</select>
		</div>

		<!-- URL Filter -->
		<div class="flex gap-2">
			<div class="flex-1">
				<label class="block text-sm text-gray-400 mb-2">url:</label>
				<input 
					type="text" 
					bind:value={url}
					placeholder="enter manifest URL"
					class="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-gray-300 placeholder-gray-600"
				/>
			</div>
			<div class="w-32 self-end">
				<select bind:value={urlMatch} class="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2">
					<option value="exact">exact</option>
					<option value="fuzzy">fuzzy</option>
				</select>
			</div>
		</div>
	</div>

	<div class="grid grid-cols-2 gap-6 mb-6">
		<!-- Billable Filter -->
		<div>
			<label class="block text-sm text-gray-400 mb-2">billable:</label>
			<div class="flex gap-2">
				<button 
					class="px-4 py-2 rounded {billable === 'all' ? 'bg-blue-600' : 'bg-gray-900 border border-gray-700'}"
					on:click={() => billable = 'all'}
				>
					all
				</button>
				<button 
					class="px-4 py-2 rounded {billable === 'review' ? 'bg-blue-600' : 'bg-gray-900 border border-gray-700'}"
					on:click={() => billable = 'review'}
				>
					review
				</button>
				<button 
					class="px-4 py-2 rounded {billable === 'true' ? 'bg-blue-600' : 'bg-gray-900 border border-gray-700'}"
					on:click={() => billable = 'true'}
				>
					true
				</button>
				<button 
					class="px-4 py-2 rounded {billable === 'false' ? 'bg-blue-600' : 'bg-gray-900 border border-gray-700'}"
					on:click={() => billable = 'false'}
				>
					false
				</button>
			</div>
		</div>

		<!-- UUID Filter -->
		<div class="flex gap-2">
			<div class="flex-1">
				<label class="block text-sm text-gray-400 mb-2">uuid:</label>
				<input 
					type="text" 
					bind:value={uuid}
					placeholder="enter application UUID"
					class="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-gray-300 placeholder-gray-600"
				/>
			</div>
			<div class="w-32 self-end">
				<select bind:value={uuidMatch} class="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2">
					<option value="exact">exact</option>
					<option value="fuzzy">fuzzy</option>
				</select>
			</div>
		</div>
	</div>

	<div class="grid grid-cols-2 gap-6">
		<!-- Ignore Files Filter -->
		<div>
			<label class="block text-sm text-gray-400 mb-2">ignore files:</label>
			<div class="flex gap-2">
				<button 
					class="px-4 py-2 rounded {ignoreFiles === true ? 'bg-blue-600' : 'bg-gray-900 border border-gray-700'}"
					on:click={() => ignoreFiles = true}
				>
					yes
				</button>
				<button 
					class="px-4 py-2 rounded {ignoreFiles === false ? 'bg-blue-600' : 'bg-gray-900 border border-gray-700'}"
					on:click={() => ignoreFiles = false}
				>
					no
				</button>
			</div>
		</div>

		<!-- App Name Filter -->
		<div class="flex gap-2">
			<div class="flex-1">
				<label class="block text-sm text-gray-400 mb-2">app name:</label>
				<input 
					type="text" 
					bind:value={appName}
					placeholder="enter application name"
					class="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2 text-gray-300 placeholder-gray-600"
				/>
			</div>
			<div class="w-32 self-end">
				<select bind:value={appNameMatch} class="w-full bg-gray-900 border border-gray-700 rounded px-3 py-2">
					<option value="exact">exact</option>
					<option value="fuzzy">fuzzy</option>
				</select>
			</div>
		</div>
	</div>

	<div class="flex justify-end gap-3 mt-6">
		<button 
			on:click={handleExport}
			class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded flex items-center gap-2"
		>
			<span>⬇</span>
		</button>
		<button 
			on:click={handleSearch}
			class="px-6 py-2 bg-blue-600 hover:bg-blue-700 rounded"
		>
			🔍
		</button>
	</div>
</div>
