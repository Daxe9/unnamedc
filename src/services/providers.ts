import type { InjectionKey } from "vue";
import type { APIRequest } from "@/types/request";

export const requestProvider = Symbol("request") as InjectionKey<APIRequest>;
