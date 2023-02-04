export interface APIRequest {
    url: string,
    settings?: any,
    body?: any,
    headers?: any
}

export const supportedMethods: Array<string> = ["GET", "POST"]; 

