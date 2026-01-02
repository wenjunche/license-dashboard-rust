<script lang="ts">
	import { onMount } from 'svelte';
	import type { AppConfig, AppConfigQuery, AppConfigListResponse } from '../lib/types';
	import AppConfigTable from '../lib/components/AppConfigTable.svelte';
	import FilterBar from '../lib/components/FilterBar.svelte';

	let data: AppConfig[] = [];
	let total = 0;
	let currentPage = 1;
	let perPage = 20;
	let loading = false;

	let filters: AppConfigQuery = {
		page: 1,
		per_page: 20,
		contract: undefined,
		url: undefined,
		url_match: 'exact',
		uuid: undefined,
		uuid_match: 'exact',
		app_name: undefined,
		app_name_match: 'exact',
		billable: undefined,
		ignore_files: undefined,
		sort_by: 'id',
		sort_order: 'asc'
	};

	async function loadData() {
		loading = true;
		try {
			const params = new URLSearchParams();
			Object.entries(filters).forEach(([key, value]) => {
				if (value !== undefined && value !== null) {
					params.append(key, String(value));
				}
			});

			const response = await fetch(`/api/app-configs?${params.toString()}`);
			const result: AppConfigListResponse = await response.json();
			
			data = result.data;
			total = result.total;
			currentPage = result.page;
			perPage = result.per_page;
		} catch (error) {
			console.error('Failed to load data:', error);
		} finally {
			loading = false;
		}
	}

	function handleFilterChange(event: CustomEvent) {
		filters = { ...filters, ...event.detail, page: 1 };
		loadData();
	}

	function handlePageChange(page: number) {
		filters.page = page;
		loadData();
	}

	function handleExport() {
		const params = new URLSearchParams();
		Object.entries(filters).forEach(([key, value]) => {
			if (value !== undefined && value !== null) {
				params.append(key, String(value));
			}
		});
		window.location.href = `/api/app-configs/export?${params.toString()}`;
	}

	onMount(() => {
		loadData();
	});
</script>

<div class="min-h-screen bg-gray-900 text-white p-6">
	<div class="max-w-7xl mx-auto">
		<header class="mb-8">
			<div class="flex items-center gap-3 mb-6">
				<div class="w-12 h-12 bg-blue-500 rounded-lg flex items-center justify-center">
					<span class="text-2xl">📊</span>
				</div>
				<h1 class="text-3xl font-bold">OpenFin Licensing</h1>
			</div>

			<nav class="flex gap-6 border-b border-gray-700 pb-2 mb-6">
				<button class="text-gray-400 hover:text-white">Overview</button>
				<button class="text-gray-400 hover:text-white">Features</button>
				<button class="text-gray-400 hover:text-white">Billing</button>
				<button class="text-gray-400 hover:text-white">Unique Users</button>
				<button class="text-blue-500 border-b-2 border-blue-500 pb-2">App Configs</button>
				<button class="text-gray-400 hover:text-white">Companies</button>
				<button class="text-gray-400 hover:text-white">Contracts</button>
				<button class="text-gray-400 hover:text-white">Desktops</button>
			</nav>
		</header>

		<FilterBar on:filterChange={handleFilterChange} on:export={handleExport} />

		<AppConfigTable 
			{data} 
			{total} 
			{currentPage} 
			{perPage}
			{loading}
			on:pageChange={(e) => handlePageChange(e.detail)}
		/>
	</div>
</div>
