<script lang="ts">
	import { createEventDispatcher } from 'svelte';
	import type { AppConfig } from '../types';

	export let data: AppConfig[];
	export let total: number;
	export let currentPage: number;
	export let perPage: number;
	export let loading = false;

	const dispatch = createEventDispatcher();

	let selectedIds = new Set<number>();

	function toggleSelection(id: number) {
		if (selectedIds.has(id)) {
			selectedIds.delete(id);
		} else {
			selectedIds.add(id);
		}
		selectedIds = selectedIds; // Trigger reactivity
	}

	function toggleAll() {
		if (selectedIds.size === data.length) {
			selectedIds.clear();
		} else {
			selectedIds = new Set(data.map(item => item.id));
		}
		selectedIds = selectedIds;
	}

	function handlePageChange(page: number) {
		dispatch('pageChange', page);
	}

	$: totalPages = Math.ceil(total / perPage);
</script>

<div class="bg-gray-800 rounded-lg overflow-hidden">
	<div class="flex gap-3 px-4 py-3 border-b border-gray-700">
		<button class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded text-sm">
			Bulk Edit
		</button>
		<button class="px-4 py-2 bg-gray-700 hover:bg-gray-600 rounded text-sm">
			Edit Contract
		</button>
	</div>

	<div class="overflow-x-auto">
		<table class="w-full">
			<thead class="bg-gray-900">
				<tr>
					<th class="px-4 py-3 text-left">
						<input 
							type="checkbox" 
							checked={selectedIds.size === data.length && data.length > 0}
							on:change={toggleAll}
							class="rounded"
						/>
					</th>
					<th class="px-4 py-3 text-left text-sm font-medium text-gray-400">Contract</th>
					<th class="px-4 py-3 text-left text-sm font-medium text-gray-400">URL</th>
					<th class="px-4 py-3 text-left text-sm font-medium text-gray-400">UUID</th>
					<th class="px-4 py-3 text-left text-sm font-medium text-gray-400">Name</th>
					<th class="px-4 py-3 text-left text-sm font-medium text-gray-400">Billable</th>
					<th class="px-4 py-3 text-left text-sm font-medium text-gray-400"></th>
				</tr>
			</thead>
			<tbody>
				{#if loading}
					<tr>
						<td colspan="7" class="px-4 py-8 text-center text-gray-400">
							Loading...
						</td>
					</tr>
				{:else if data.length === 0}
					<tr>
						<td colspan="7" class="px-4 py-8 text-center text-gray-400">
							No data found
						</td>
					</tr>
				{:else}
					{#each data as item (item.id)}
						<tr class="border-t border-gray-700 hover:bg-gray-750">
							<td class="px-4 py-3">
								<input 
									type="checkbox" 
									checked={selectedIds.has(item.id)}
									on:change={() => toggleSelection(item.id)}
									class="rounded"
								/>
							</td>
							<td class="px-4 py-3 text-sm">{item.contract}</td>
							<td class="px-4 py-3 text-sm text-blue-400">{item.url}</td>
							<td class="px-4 py-3 text-sm">{item.uuid}</td>
							<td class="px-4 py-3 text-sm">{item.name}</td>
							<td class="px-4 py-3 text-sm">
								{item.billable === null ? '??' : item.billable ? 'true' : 'false'}
							</td>
							<td class="px-4 py-3 text-right">
								<button class="px-3 py-1 bg-blue-600 hover:bg-blue-700 rounded text-sm">
									✏️
								</button>
							</td>
						</tr>
					{/each}
				{/if}
			</tbody>
		</table>
	</div>

	{#if totalPages > 1}
		<div class="flex justify-center gap-2 px-4 py-4 border-t border-gray-700">
			<button 
				disabled={currentPage === 1}
				on:click={() => handlePageChange(currentPage - 1)}
				class="px-3 py-1 bg-gray-700 hover:bg-gray-600 rounded disabled:opacity-50"
			>
				Previous
			</button>
			
			{#each Array(totalPages) as _, i}
				<button 
					on:click={() => handlePageChange(i + 1)}
					class="px-3 py-1 rounded {currentPage === i + 1 ? 'bg-blue-600' : 'bg-gray-700 hover:bg-gray-600'}"
				>
					{i + 1}
				</button>
			{/each}
			
			<button 
				disabled={currentPage === totalPages}
				on:click={() => handlePageChange(currentPage + 1)}
				class="px-3 py-1 bg-gray-700 hover:bg-gray-600 rounded disabled:opacity-50"
			>
				Next
			</button>
		</div>
	{/if}
</div>
