<script setup lang="ts">
import Request from "@/components/Request/Request.vue";
import Response from "@/components/Response/Response.vue";
import { ref, onMounted } from "vue";
import { APIResponse } from "@/types/response";
/* import test from "@/test.test.test.test" */

const response = ref<APIResponse>();

function receiveResponse(res: APIResponse) {
	response.value = res;
}

onMounted(async () => {
	/* await test() */
});
</script>
<template>
	<div class="main-container">
		<Request @emitResponse="receiveResponse" />
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
