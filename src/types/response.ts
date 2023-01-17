export interface APIResponse {
    body: any,
    status_code: number,
    headers: Map<string, string>,
    duration: number,
}
