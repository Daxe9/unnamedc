import { invoke } from "@tauri-apps/api";
import { APIResponse } from "@/types/response";
import { APIRequest, supportedMethods } from "@/types/request";

async function processRequest(request: APIRequest): Promise<APIResponse> {
    try {
        if (!request.url) {
            throw new Error("URL is required");
        }

        if (!request.settings) {
            throw new Error("Request settings are required");
        }

        if (!request.settings.method) {
            throw new Error("Request method is required");
        }
        
        if (!supportedMethods.includes(request.settings.method)) {
            throw new Error("Request method not supported");
        }

        switch (request.settings.method) {
            case "GET":
                return await getRequest(request.url, request.headers);
            case "POST":
                return await postRequest(request.url, request.body, request.headers);
            default: 
                throw new Error("Request method not supported");
        }


    } catch (error: any) {
        return Promise.reject(error);
    }
}

async function getRequest(url: string, headers?: any): Promise<APIResponse> {
	try {
		const requestBody: any = {
			url,
		};

		if (headers) {
			requestBody.headers = headers;
		}

		const response: APIResponse = await invoke("get_request", requestBody);

		response.body = JSON.parse(response.body);
		response.duration += "µs";

		return response;
	} catch (error: any) {
		return Promise.reject(Error(error));
	}
}

async function postRequest(
	url: string,
	body?: any,
	headers?: any
): Promise<APIResponse> {
	try {
		const requestBody: any = {
			url,
		};

		if (body) {
			body = JSON.stringify(body);
			requestBody.body = body;
		}

		if (headers) {
			requestBody.headers = headers;
		}
		const response: APIResponse = await invoke("post_request", requestBody);

		response.body = JSON.parse(response.body);
		response.duration += "µs";

		return response;
	} catch (error: any) {
		return Promise.reject(Error(error));
	}
}

export { processRequest, getRequest, postRequest };
