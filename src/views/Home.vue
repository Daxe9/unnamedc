<script setup lang="ts">
import Request from "@/components/Request/Request.vue";
import Response from "@/components/Response/Response.vue";
import { ref } from "vue";
import type { APIResponse } from "@/types/response";
import type { APIRequest } from "@/types/request";
import { processRequest } from "@/services/requestService";

const errorMessage = ref<string>("");
const response = ref<APIResponse>();

// receive requset from Request component and send it to the requestService
async function handleRequest(request: APIRequest) {
    try {
        // set response if successful
        response.value = await processRequest(request);
        // remove error message
        errorMessage.value = "";
    } catch (error: any) {
        if (import.meta.env.MODE === "development") {
            console.error(error);
        }
        errorMessage.value = error.message;
    } 
}
</script>
<template>
	<div class="main-container">
		<Request :renderError="errorMessage" @sendRequest="handleRequest" />
		<Response :response="response" />
	</div>
</template>

<style scoped>
.main-container {
	display: flex;
	flex-direction: row;
	width: 100%;
	height: 100%;
	color: #ffffff;
}

.main-container > * {
	max-width: 50%;
	flex-basis: 50%;
}

@media (max-width: 768px) {
	.main-container {
		flex-direction: column;
	}

	.main-container > * {
		max-width: 100%;
		max-height: 50%;
	}
}
</style>
