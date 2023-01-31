export interface APIResponse {
	body: any;
	status_code: number;
	headers: Record<string, string>;
	duration: number | string;
}

export interface ApiFailedResponse {
	error_message: string;
	status_code: number;
}
