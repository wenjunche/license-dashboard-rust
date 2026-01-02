export interface AppConfig {
	id: number;
	contract: string;
	url: string;
	uuid: string;
	name: string;
	billable: boolean | null;
	ignore_files: boolean;
	created_at: string;
	updated_at: string;
}

export interface AppConfigQuery {
	page: number;
	per_page: number;
	contract?: string;
	url?: string;
	url_match: 'exact' | 'fuzzy';
	uuid?: string;
	uuid_match: 'exact' | 'fuzzy';
	app_name?: string;
	app_name_match: 'exact' | 'fuzzy';
	billable?: 'all' | 'review' | 'true' | 'false';
	ignore_files?: boolean;
	sort_by?: string;
	sort_order: 'asc' | 'desc';
}

export interface AppConfigListResponse {
	data: AppConfig[];
	total: number;
	page: number;
	per_page: number;
	total_pages: number;
}
