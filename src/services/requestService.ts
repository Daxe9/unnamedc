import { invoke } from "@tauri-apps/api"
import { APIResponse } from "@/types/response";

async function getRequest(url: string, headers?: any): Promise<APIResponse> {
    try {
        const requestBody: any = {
            url
        }

        if (headers) {
            requestBody.headers = headers
        }

        const response: APIResponse = await invoke("get_request", requestBody);
        response.body = JSON.parse(response.body);
    
        return response
    } catch (error: any) {
        return Promise.reject(Error(`[ERROR]: ${error}`)) 
    }     
}

async function postRequest(url: string, body?: any, headers?: any): Promise<APIResponse> {
    try {
        const requestBody: any = {
            url,
        }

        if (body) {
            body = JSON.stringify(body);
            requestBody.body = body;
        }

        if (headers) {
            requestBody.headers = headers
        }
        const response: APIResponse = await invoke("post_request", requestBody);
        response.body = JSON.parse(response.body);
    
        return response
    } catch (error: any) {
        return Promise.reject(Error(`[ERROR]: ${error}`)) 
    }     
}

export {
    type APIResponse,
    getRequest,
    postRequest
}

